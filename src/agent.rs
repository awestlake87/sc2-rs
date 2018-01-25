use futures::prelude::*;
use organelle::{self, Axon, Constraint, Impulse, Organelle, Soma};
use sc2_proto::sc2api;
use tokio_core::reactor;
use url::Url;

use super::{
    Dendrite,
    Error,
    GamePorts,
    GameSettings,
    IntoProto,
    Map,
    MeleeContract,
    MeleeDendrite,
    PlayerSetup,
    Race,
    Result,
    Synapse,
    Terminal,
};
use client::{ClientSoma, ClientTerminal};
//use observer::ObserverSoma;

/// manages a player soma
pub struct AgentSoma {
    controller: Option<MeleeDendrite>,
    client: Option<ClientTerminal>,
}

impl AgentSoma {
    fn axon() -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                controller: None,
                client: None,
            },
            vec![Constraint::One(Synapse::Melee)],
            vec![Constraint::One(Synapse::Client)],
        ))
    }

    /// compose an agent organelle to interact with a controller soma
    pub fn organelle<T>(
        _soma: T,
        handle: reactor::Handle,
    ) -> Result<Organelle<Axon<Self>>>
    where
        T: Soma + 'static,

        T::Synapse: From<Synapse> + Into<Synapse>,
        <T::Synapse as organelle::Synapse>::Terminal: From<Terminal>
            + Into<Terminal>,
        <T::Synapse as organelle::Synapse>::Dendrite: From<Dendrite>
            + Into<Dendrite>,
    {
        let mut organelle = Organelle::new(AgentSoma::axon()?, handle);

        let agent = organelle.nucleus();
        // let player = organelle.add_soma(soma);

        // // TODO: find out why these explicit annotation is needed.
        // it's // possible that it's a bug in the rust type
        // system because it will // work when the function is generic
        // across two soma types, but not one         let client =
        //
        let client = organelle.add_soma(ClientSoma::axon()?);
        //         let observer =

        // organelle.add_soma::<Sheath<ObserverSoma>>(ObserverSoma::sheath()?);

        //         organelle.connect(agent, client, Synapse::InstanceProvider);
        organelle.connect(agent, client, Synapse::Client)?;
        //         organelle.connect(observer, client, Synapse::Client);

        //         organelle.connect(agent, observer, Synapse::Observer);
        //         organelle.connect(agent, player, Synapse::Agent);

        Ok(organelle)
    }
}

impl Soma for AgentSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddDendrite(Synapse::Melee, Dendrite::Melee(rx)) => {
                Ok(Self {
                    controller: Some(rx),
                    client: self.client,
                })
            },
            Impulse::AddTerminal(Synapse::Client, Terminal::Client(tx)) => {
                Ok(Self {
                    controller: self.controller,
                    client: Some(tx),
                })
            },
            Impulse::Start(tx, handle) => {
                assert!(self.controller.is_some());
                assert!(self.client.is_some());

                handle.spawn(
                    self.controller
                        .unwrap()
                        .wrap(AgentMeleeDendrite {
                            client: self.client.unwrap(),
                        })
                        .or_else(move |e| {
                            tx.send(Impulse::Error(e.into()))
                                .map(|_| ())
                                .map_err(|_| ())
                        }),
                );

                Ok(Self {
                    controller: None,
                    client: None,
                })
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

pub struct AgentMeleeDendrite {
    client: ClientTerminal,
}

impl MeleeContract for AgentMeleeDendrite {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, _: GameSettings) -> Result<(Self, PlayerSetup)> {
        Ok((self, PlayerSetup::Player { race: Race::Zerg }))
    }

    #[async(boxed)]
    fn connect(self, url: Url) -> Result<Self> {
        await!(self.client.clone().connect(url))?;
        Ok(self)
    }

    #[async(boxed)]
    fn create_game(
        self,
        settings: GameSettings,
        players: Vec<PlayerSetup>,
    ) -> Result<Self> {
        let mut req = sc2api::Request::new();

        match settings.map {
            Map::LocalMap(ref path) => {
                req.mut_create_game().mut_local_map().set_map_path(
                    match path.clone().into_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => bail!("invalid path string"),
                    },
                );
            },
            Map::BlizzardMap(ref map) => {
                req.mut_create_game().set_battlenet_map_name(map.clone());
            },
        };

        for player in players {
            let mut setup = sc2api::PlayerSetup::new();

            match player {
                PlayerSetup::Computer {
                    difficulty, race, ..
                } => {
                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(difficulty.to_proto());
                    setup.set_race(race.into_proto()?);
                },
                PlayerSetup::Player { race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Participant);

                    setup.set_race(race.into_proto()?);
                }, /*PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }*/
            }

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(false);

        let rsp = await!(self.client.clone().request(req))?;

        println!("rsp: {:#?}", rsp);

        Ok(self)
    }

    #[async(boxed)]
    fn join_game(
        self,
        setup: PlayerSetup,
        ports: Option<GamePorts>,
    ) -> Result<Self> {
        let mut req = sc2api::Request::new();

        match setup {
            PlayerSetup::Computer { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            PlayerSetup::Player { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            }, //_ => req.mut_join_game().set_race(common::Race::NoRace)
        };

        if let Some(ports) = ports {
            req.mut_join_game()
                .set_shared_port(ports.shared_port as i32);

            {
                let s = req.mut_join_game().mut_server_ports();

                s.set_game_port(ports.server_ports.game_port as i32);
                s.set_base_port(ports.server_ports.base_port as i32);
            }

            {
                let client_ports = req.mut_join_game().mut_client_ports();

                for c in &ports.client_ports {
                    let mut p = sc2api::PortSet::new();

                    p.set_game_port(c.game_port as i32);
                    p.set_base_port(c.base_port as i32);

                    client_ports.push(p);
                }
            }
        }

        {
            let options = req.mut_join_game().mut_options();

            options.set_raw(true);
            options.set_score(true);
        }

        let rsp = await!(self.client.clone().request(req))?;

        println!("rsp: {:#?}", rsp);

        Ok(self)
    }
}

//     fn create_game(
//         self,
//         axon: &Axon,
//         src: Handle,
//         settings: GameSettings,
//         players: Vec<PlayerSetup>,
//     ) -> Result<AgentSoma> {
//         assert_eq!(src, axon.req_input(Synapse::Controller)?);

//         let transactor = Transactor::send(axon, ClientRequest::new(req))?;

//         Ok(AgentSoma::CreateGame(CreateGame {
//             transactor: transactor,
//         }))
//     }

//     fn on_game_ready(
//         self,
//         axon: &Axon,
//         setup: PlayerSetup,
//         ports: Option<GamePorts>,
//     ) -> Result<AgentSoma> {
//         let this_soma = axon.effector()?.this_soma();

//         axon.effector()?
//             .send(this_soma, Signal::GameReady(setup, ports));

//         Ok(AgentSoma::GameCreated(GameCreated {}))
//     }
// }

// pub struct CreateGame {
//     transactor: Transactor,
// }

// impl CreateGame {
//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.transactor.expect(src, result)?;

//                 GameCreated::game_created(axon)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct GameCreated;

// impl GameCreated {
//     fn game_created(axon: &Axon) -> Result<AgentSoma> {
//         axon.send_req_input(Synapse::Controller, Signal::GameCreated)?;

//         Ok(AgentSoma::GameCreated(GameCreated {}))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(_, Signal::GameReady(setup, ports)) => {
//                 JoinGame::join_game(axon, setup, ports)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct JoinGame {
//     transactor: Transactor,
// }

// impl JoinGame {
//     fn join_game(
//         axon: &Axon,
//         setup: PlayerSetup,
//         ports: Option<GamePorts>,
//     ) -> Result<AgentSoma> {
//         let mut req = sc2api::Request::new();

//         match setup {
//             PlayerSetup::Computer { race, .. } => {
//                 req.mut_join_game().set_race(race.into_proto()?);
//             },
//             PlayerSetup::Player { race, .. } => {
//                 req.mut_join_game().set_race(race.into_proto()?);
//             }, //_ => req.mut_join_game().set_race(common::Race::NoRace)
//         };

//         if let Some(ports) = ports {
//             req.mut_join_game()
//                 .set_shared_port(ports.shared_port as i32);

//             {
//                 let s = req.mut_join_game().mut_server_ports();

//                 s.set_game_port(ports.server_ports.game_port as i32);
//                 s.set_base_port(ports.server_ports.base_port as i32);
//             }

//             {
//                 let client_ports = req.mut_join_game().mut_client_ports();

//                 for c in &ports.client_ports {
//                     let mut p = sc2api::PortSet::new();

//                     p.set_game_port(c.game_port as i32);
//                     p.set_base_port(c.base_port as i32);

//                     client_ports.push(p);
//                 }
//             }
//         }

//         {
//             let options = req.mut_join_game().mut_options();

//             options.set_raw(true);
//             options.set_score(true);
//         }

//         let transactor = Transactor::send(
//             axon,
//             ClientRequest::with_timeout(req, time::Duration::from_secs(60)),
//         )?;

//         Ok(AgentSoma::JoinGame(JoinGame {
//             transactor: transactor,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.on_join_game(axon, src, result)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn on_join_game(
//         self,
//         axon: &Axon,
//         src: Handle,
//         result: ClientResult,
//     ) -> Result<AgentSoma> {
//         self.transactor.expect(src, result)?;

//         FetchGameData::fetch(axon)
//     }
// }

// pub struct FetchGameData;

// impl FetchGameData {
//     fn fetch(axon: &Axon) -> Result<AgentSoma> {
//         axon.send_req_output(Synapse::Observer, Signal::FetchGameData)?;

//         Ok(AgentSoma::FetchGameData(FetchGameData {}))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(_, Signal::GameDataReady) => {
//                 StepperSetup::setup(axon)
//             },
// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct StepperSetup {
//     stepper: Handle,
// }

// impl StepperSetup {
//     fn setup(axon: &Axon) -> Result<AgentSoma> {
//         let stepper = axon.req_output(Synapse::Agent)?;

//         axon.effector()?
//             .send(stepper, Signal::RequestUpdateInterval);

//         Ok(AgentSoma::StepperSetup(StepperSetup { stepper: stepper }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::UpdateInterval(interval)) => {
//                 self.on_update_interval(axon, src, interval)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn on_update_interval(
//         self,
//         axon: &Axon,
//         src: Handle,
//         interval: u32,
//     ) -> Result<AgentSoma> {
//         if src == self.stepper {
//             Step::first(axon, interval)
//         } else {
//             bail!("unexpected source of update interval: {}", src)
//         }
//     }
// }

// pub struct Update {
//     interval: u32,
//     commands: Vec<Command>,
//     debug_commands: Vec<DebugCommand>,
// }

// impl Update {
//     fn next(
//         axon: &Axon,
//         interval: u32,
//         frame: Rc<FrameData>,
//     ) -> Result<AgentSoma> {
//         axon.send_req_output(Synapse::Agent, Signal::Observation(frame))?;

//         Ok(AgentSoma::Update(Update {
//             interval: interval,
//             commands: vec![],
//             debug_commands: vec![],
//         }))
//     }

//     fn update(
//         mut self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(_, Signal::Command(cmd)) => {
//                 self.commands.push(cmd);
//                 Ok(AgentSoma::Update(self))
//             },
//             Impulse::Signal(_, Signal::DebugCommand(cmd)) => {
//                 self.debug_commands.push(cmd);
//                 Ok(AgentSoma::Update(self))
//             },

//             Impulse::Signal(_, Signal::UpdateComplete) => {
//                 self.on_update_complete(axon)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn on_update_complete(self, axon: &Axon) -> Result<AgentSoma> {
//         SendActions::send_actions(
//             axon,
//             self.interval,
//             self.commands,
//             self.debug_commands,
//         )
//     }
// }

// pub struct SendActions {
//     interval: u32,
//     transactor: Transactor,

//     debug_commands: Vec<DebugCommand>,
// }

// impl SendActions {
//     fn send_actions(
//         axon: &Axon,
//         interval: u32,
//         commands: Vec<Command>,
//         debug_commands: Vec<DebugCommand>,
//     ) -> Result<AgentSoma> {
//         let mut req = sc2api::Request::new();
//         req.mut_action().mut_actions();

//         for cmd in commands {
//             match cmd {
//                 Command::Action {
//                     units,
//                     ability,
//                     target,
//                 } => {
//                     let mut a = sc2api::Action::new();

//                     {
//                         let cmd = a.mut_action_raw().mut_unit_command();

//                         cmd.set_ability_id(ability.into_proto()? as i32);

//                         match target {
//                             Some(ActionTarget::UnitTag(tag)) => {
//                                 cmd.set_target_unit_tag(tag);
//                             },
//                             Some(ActionTarget::Location(pos)) => {
// let target =
// cmd.mut_target_world_space_pos();
// target.set_x(pos.x);                                 target.set_y(pos.y);
//                             },
//                             None => (),
//                         }

//                         for u in units {
//                             cmd.mut_unit_tags().push(u.tag);
//                         }
//                     }

//                     req.mut_action().mut_actions().push(a);
//                 },
//             }
//         }

//         let transactor = Transactor::send(&axon, ClientRequest::new(req))?;

//         Ok(AgentSoma::SendActions(SendActions {
//             interval: interval,
//             transactor: transactor,

//             debug_commands: debug_commands,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.transactor.expect(src, result)?;

// SendDebug::send_debug(axon, self.interval,
// self.debug_commands)             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct SendDebug {
//     interval: u32,
//     transactor: Transactor,
// }

// impl SendDebug {
//     fn send_debug(
//         axon: &Axon,
//         interval: u32,
//         commands: Vec<DebugCommand>,
//     ) -> Result<AgentSoma> {
//         let mut req = sc2api::Request::new();
//         req.mut_debug().mut_debug();

//         for cmd in commands {
//             match cmd {
//                 DebugCommand::DebugText {
//                     text,
//                     target,
//                     color,
//                 } => {
//                     let mut cmd = debug::DebugCommand::new();
//                     let mut debug_text = debug::DebugText::new();

//                     debug_text.set_text(text);

//                     match target {
//                         Some(DebugTextTarget::Screen(p)) => {
//                             debug_text.mut_virtual_pos().set_x(p.x);
//                             debug_text.mut_virtual_pos().set_y(p.y);
//                         },
//                         Some(DebugTextTarget::World(p)) => {
//                             debug_text.mut_world_pos().set_x(p.x);
//                             debug_text.mut_world_pos().set_y(p.y);
//                             debug_text.mut_world_pos().set_z(p.z);
//                         },
//                         None => (),
//                     }

//                     debug_text.mut_color().set_r(color.0 as u32);
//                     debug_text.mut_color().set_g(color.1 as u32);
//                     debug_text.mut_color().set_b(color.2 as u32);

//                     cmd.mut_draw().mut_text().push(debug_text);
//                     req.mut_debug().mut_debug().push(cmd);
//                 },
//                 DebugCommand::DebugLine { p1, p2, color } => {
//                     let mut cmd = debug::DebugCommand::new();
//                     let mut debug_line = debug::DebugLine::new();

//                     debug_line.mut_line().mut_p0().set_x(p1.x);
//                     debug_line.mut_line().mut_p0().set_y(p1.y);
//                     debug_line.mut_line().mut_p0().set_z(p1.z);

//                     debug_line.mut_line().mut_p1().set_x(p2.x);
//                     debug_line.mut_line().mut_p1().set_y(p2.y);
//                     debug_line.mut_line().mut_p1().set_z(p2.z);

//                     debug_line.mut_color().set_r(color.0 as u32);
//                     debug_line.mut_color().set_g(color.1 as u32);
//                     debug_line.mut_color().set_b(color.2 as u32);

//                     cmd.mut_draw().mut_lines().push(debug_line);
//                     req.mut_debug().mut_debug().push(cmd);
//                 },
//                 DebugCommand::DebugBox { min, max, color } => {
//                     let mut cmd = debug::DebugCommand::new();
//                     let mut debug_box = debug::DebugBox::new();

//                     debug_box.mut_min().set_x(min.x);
//                     debug_box.mut_min().set_y(min.y);
//                     debug_box.mut_min().set_z(min.z);

//                     debug_box.mut_max().set_x(max.x);
//                     debug_box.mut_max().set_y(max.y);
//                     debug_box.mut_max().set_z(max.z);

//                     debug_box.mut_color().set_r(color.0 as u32);
//                     debug_box.mut_color().set_g(color.1 as u32);
//                     debug_box.mut_color().set_b(color.2 as u32);

//                     cmd.mut_draw().mut_boxes().push(debug_box);
//                     req.mut_debug().mut_debug().push(cmd);
//                 },
//                 DebugCommand::DebugSphere {
//                     center,
//                     radius,
//                     color,
//                 } => {
//                     let mut cmd = debug::DebugCommand::new();
//                     let mut debug_sphere = debug::DebugSphere::new();

//                     debug_sphere.mut_p().set_x(center.x);
//                     debug_sphere.mut_p().set_y(center.y);
//                     debug_sphere.mut_p().set_z(center.z);

//                     debug_sphere.set_r(radius);

//                     debug_sphere.mut_color().set_r(color.0 as u32);
//                     debug_sphere.mut_color().set_g(color.1 as u32);
//                     debug_sphere.mut_color().set_b(color.2 as u32);

//                     cmd.mut_draw().mut_spheres().push(debug_sphere);
//                     req.mut_debug().mut_debug().push(cmd);
//                 },
//             }
//         }

//         let transactor = Transactor::send(&axon, ClientRequest::new(req))?;

//         Ok(AgentSoma::SendDebug(SendDebug {
//             interval: interval,
//             transactor: transactor,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.transactor.expect(src, result)?;

//                 Step::step(axon, self.interval)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct Step {
//     interval: u32,
//     transactor: Transactor,
// }

// impl Step {
//     fn first(axon: &Axon, interval: u32) -> Result<AgentSoma> {
//         axon.send_req_output(Synapse::Agent, Signal::GameStarted)?;

//         Step::step(axon, interval)
//     }
//     fn step(axon: &Axon, interval: u32) -> Result<AgentSoma> {
//         let mut req = sc2api::Request::new();

//         req.mut_step().set_count(interval);

//         let transactor = Transactor::send(axon, ClientRequest::new(req))?;

//         Ok(AgentSoma::Step(Step {
//             interval: interval,
//             transactor: transactor,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.on_step(axon, src, result)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn on_step(
//         self,
//         axon: &Axon,
//         src: Handle,
//         result: ClientResult,
//     ) -> Result<AgentSoma> {
//         self.transactor.expect(src, result)?;

//         Observe::observe(axon, self.interval)
//     }
// }

// pub struct Observe {
//     interval: u32,
// }

// impl Observe {
//     fn observe(axon: &Axon, interval: u32) -> Result<AgentSoma> {
//         axon.send_req_output(Synapse::Observer, Signal::Observe)?;

//         Ok(AgentSoma::Observe(Observe { interval: interval }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(_, Signal::Observation(frame)) => {
//                 Update::next(axon, self.interval, frame)
//             },
//             Impulse::Signal(_, Signal::GameEnded) => LeaveGame::leave(axon),

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct LeaveGame {
//     transactor: Transactor,
// }

// impl LeaveGame {
//     fn leave(axon: &Axon) -> Result<AgentSoma> {
//         let mut req = sc2api::Request::new();

//         req.mut_leave_game();

//         let transactor = Transactor::send(axon, ClientRequest::new(req))?;

//         Ok(AgentSoma::LeaveGame(LeaveGame {
//             transactor: transactor,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.transactor.expect(src, result)?;

//                 Reset::reset(axon)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct Reset;

// impl Reset {
//     fn reset(axon: &Axon) -> Result<AgentSoma> {
//         axon.send_req_output(Synapse::Client, Signal::ClientDisconnect)?;

//         Ok(AgentSoma::Reset(Reset {}))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(_, Signal::ClientError(_)) => {
//                 // client does not close cleanly anyway right now, so just
//                 // ignore the error and wait for ClientClosed.
//                 Ok(AgentSoma::Reset(self))
//             },
//             Impulse::Signal(_, Signal::ClientClosed) => {
//                 axon.send_req_input(Synapse::Controller, Signal::GameEnded)?;
//                 axon.send_req_output(Synapse::Agent, Signal::GameEnded)?;

//                 Setup::setup()
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }
