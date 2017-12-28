
use std::time;

use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol, Constraint };
use sc2_proto::{ sc2api, common };
use url::Url;

use super::super::{ Result, IntoProto, ErrorKind };
use lobes::{ Message, Role, Soma, Cortex };
use lobes::client::{
    ClientLobe, TransactionId, ClientRequest, ClientResponse, ClientMessageKind
};

use data::{ GameSettings, GamePorts, PlayerSetup, Map };

pub enum AgentLobe {
    Init(AgentInit),
    Setup(AgentSetup),
}

impl AgentLobe {
    fn new() -> Result<Self> {
        Ok(
            AgentLobe::Init(
                AgentInit {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::Controller),
                            Constraint::RequireOne(Role::InstanceProvider),
                        ],
                        vec![
                            Constraint::RequireOne(Role::Client),
                            Constraint::RequireOne(Role::Agent),
                            Constraint::RequireOne(Role::InstanceProvider),
                        ],
                    )?,
                }
            )
        )
    }

    pub fn cortex<L>(lobe: L) -> Result<Cortex> where
        L: Lobe + 'static,

        L::Message: From<Message>,
        L::Role: From<Role>,

        Message: From<L::Message>,
        Role: From<L::Role>,
    {
        let mut cortex = Cortex::new(AgentLobe::new()?);

        let agent = cortex.get_main_handle();
        let player = cortex.add_lobe(lobe);

        // TODO: find out why this explicit annotation is needed. it's possible
        // that it's a bug in the rust type system because it will work when
        // the function is generic across two lobe types, but not one
        let client = cortex.add_lobe::<ClientLobe>(ClientLobe::new()?);

        cortex.connect(agent, client, Role::InstanceProvider);
        cortex.connect(agent, client, Role::Client);
        cortex.connect(agent, player, Role::Agent);

        Ok(cortex)
    }
}

impl Lobe for AgentLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
        -> cortical::Result<Self>
    {
        match self {
            AgentLobe::Init(state) => state.update(msg),
            AgentLobe::Setup(state) => state.update(msg),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}

pub struct AgentInit {
    soma:           Soma,
}

impl AgentInit {
    fn update(mut self, msg: Protocol<Message, Role>)
        -> Result<AgentLobe>
    {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Start => {
                Ok(
                    AgentLobe::Setup(
                        AgentSetup {
                            soma: self.soma,
                            transaction: None
                        }
                    )
                )
            },

            _ => Ok(AgentLobe::Init(self)),
        }
    }
}

pub struct AgentSetup {
    soma:           Soma,
    transaction:    Option<(TransactionId, ClientMessageKind)>,
}

impl AgentSetup {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::Ready) => {
                self.on_ready(src)
            },

            Protocol::Message(src, Message::RequestPlayerSetup(settings)) => {
                self.on_req_player_setup(src, settings)
            },
            Protocol::Message(src, Message::PlayerSetup(setup)) => {
                self.on_player_setup(src, setup)
            },
            Protocol::Message(
                src, Message::ProvideInstance(instance, url)
            ) => {
                self.provide_instance(src, instance, url)
            }
            Protocol::Message(src, Message::CreateGame(settings, players)) => {
                self.create_game(src, settings, players)
            },
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                self.on_response(src, rsp)
            },
            Protocol::Message(src, Message::GameReady(setup, ports)) => {
                self.on_game_ready(src, setup, ports)
            }

            _ => Ok(AgentLobe::Setup(self))
        }
    }

    fn on_ready(self, src: Handle) -> Result<AgentLobe> {
        assert_eq!(src, self.soma.req_output(Role::Client)?);

        self.soma.send_req_input(Role::Controller, Message::Ready)?;

        Ok(AgentLobe::Setup(self))
    }

    fn on_req_player_setup(self, src: Handle, settings: GameSettings)
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        self.soma.send_req_output(
            Role::Agent, Message::RequestPlayerSetup(settings)
        )?;

        Ok(AgentLobe::Setup(self))
    }

    fn on_player_setup(self, src: Handle, setup: PlayerSetup)
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_output(Role::Agent)?);

        self.soma.send_req_input(
            Role::Controller, Message::PlayerSetup(setup)
        )?;

        Ok(AgentLobe::Setup(self))
    }

    fn provide_instance(self, src: Handle, instance: Handle, url: Url)
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::InstanceProvider)?);

        self.soma.send_req_output(
            Role::InstanceProvider, Message::ProvideInstance(instance, url)
        )?;

        Ok(AgentLobe::Setup(self))
    }

    fn create_game(
        self,
        src: Handle,
        settings: GameSettings,
        players: Vec<PlayerSetup>
    )
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        println!("create game with settings: {:#?}", settings);

        let mut req = sc2api::Request::new();

        match settings.map {
            Map::LocalMap(ref path) => {
                req.mut_create_game().mut_local_map().set_map_path(
                    match path.clone().into_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => bail!("invalid path string")
                    }
                );
            },
            Map::BlizzardMap(ref map) => {
                req.mut_create_game().set_battlenet_map_name(map.clone());
            }
        };

        for player in players {
            let mut setup = sc2api::PlayerSetup::new();

            match player {
                PlayerSetup::Computer { difficulty, race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(difficulty.to_proto());
                    setup.set_race(race.into_proto()?);
                },
                PlayerSetup::Player { race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Participant);

                    setup.set_race(race.into_proto()?);
                },
                PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }
            }

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(false);

        self.send(ClientRequest::new(req))
    }

    fn send(mut self, req: ClientRequest) -> Result<AgentLobe> {
        self.transaction = Some((req.transaction, req.kind));

        self.soma.send_req_output(
            Role::Client, Message::ClientRequest(req)
        )?;

        Ok(AgentLobe::Setup(self))
    }

    fn on_response(self, src: Handle, rsp: ClientResponse)
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_output(Role::Client)?);

        let (transaction, req_kind) = {
            if let Some(transaction) = self.transaction {
                transaction
            }
            else {
                bail!("unexpected message {:#?}", rsp.response)
            }
        };

        if transaction != rsp.transaction {
            bail!("transaction id mismatch")
        }

        if req_kind != rsp.kind {
            bail!("expected {:?} message, got {:?}", req_kind, rsp.kind)
        }

        if rsp.response.get_error().len() != 0 {
            bail!(
                ErrorKind::GameErrors(
                    rsp.response.get_error().iter()
                        .map(|e| e.clone())
                        .collect()
                )
            )
        }

        match rsp.kind {
            ClientMessageKind::CreateGame => {
                self.soma.send_req_input(
                    Role::Controller, Message::GameCreated
                )?;
            },
            ClientMessageKind::JoinGame => {
                println!("game joined {:#?}", rsp.response);
            },
            _ => bail!("unexpected message {:#?}", rsp.response)
        }

        Ok(AgentLobe::Setup(self))
    }

    fn on_game_ready(
        self, src: Handle, setup: PlayerSetup, ports: GamePorts
    )
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        println!("join game with setup {:#?} and ports {:#?}", setup, ports);

        let mut req = sc2api::Request::new();

        match setup {
            PlayerSetup::Computer { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            PlayerSetup::Player { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            _ => req.mut_join_game().set_race(common::Race::NoRace)
        };

        req.mut_join_game().set_shared_port(ports.shared_port as i32);

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

        {
            let options = req.mut_join_game().mut_options();

            options.set_raw(true);
            options.set_score(true);
        }

        self.send(
            ClientRequest::with_timeout(req, time::Duration::from_secs(60))
        )
    }
}
