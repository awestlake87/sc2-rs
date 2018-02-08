use std::collections::{HashMap, HashSet};
use std::mem;
use std::rc::Rc;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use sc2_proto::sc2api;

use super::{Error, FromProto, IntoSc2, Result};
use client::ClientTerminal;
use data::{
    Action,
    Alliance,
    DisplayType,
    GameEvent,
    MapInfo,
    Observation,
    Point2,
    SpatialAction,
    Tag,
    Unit,
    Upgrade,
};
use synapses::{Dendrite, Synapse, Terminal};

pub struct ObserverSoma {
    client: Option<ClientTerminal>,
    controller: Option<ObserverControlDendrite>,
    users: Vec<ObserverDendrite>,
}

impl ObserverSoma {
    pub fn axon() -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                client: None,
                controller: None,
                users: vec![],
            },
            vec![
                Constraint::One(Synapse::ObserverControl),
                Constraint::Variadic(Synapse::Observer),
            ],
            vec![Constraint::One(Synapse::Client)],
        ))
    }
}

impl Soma for ObserverSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(mut self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddTerminal(_, Synapse::Client, Terminal::Client(tx)) => {
                self.client = Some(tx);

                Ok(self)
            },
            Impulse::AddDendrite(
                _,
                Synapse::ObserverControl,
                Dendrite::ObserverControl(rx),
            ) => {
                self.controller = Some(rx);

                Ok(self)
            },
            Impulse::AddDendrite(
                _,
                Synapse::Observer,
                Dendrite::Observer(rx),
            ) => {
                self.users.push(rx);

                Ok(self)
            },

            Impulse::Start(_, main_tx, handle) => {
                assert!(self.controller.is_some());
                assert!(self.client.is_some());

                let (tx, rx) = mpsc::channel(10);

                // merge all queues
                for user in self.users {
                    handle.spawn(
                        tx.clone()
                            .send_all(user.rx.map_err(|_| unreachable!()))
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                }

                let task = ObserverTask::new(
                    self.controller.unwrap(),
                    self.client.unwrap(),
                    rx,
                );

                handle.spawn(task.run().or_else(move |e| {
                    main_tx
                        .send(Impulse::Error(e.into()))
                        .map(|_| ())
                        .map_err(|_| ())
                }));

                Ok(Self {
                    controller: None,
                    client: None,
                    users: vec![],
                })
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

struct ObserverTask {
    controller: Option<ObserverControlDendrite>,
    client: ClientTerminal,
    request_rx: Option<mpsc::Receiver<ObserverRequest>>,

    previous_step: u32,
    current_step: u32,
    previous_units: HashMap<Tag, Rc<Unit>>,
    units: HashMap<Tag, Rc<Unit>>,

    previous_upgrades: HashSet<Upgrade>,
    upgrades: HashSet<Upgrade>,

    actions: Vec<Action>,
    spatial_actions: Vec<SpatialAction>,
}

impl ObserverTask {
    fn new(
        controller: ObserverControlDendrite,
        client: ClientTerminal,
        request_rx: mpsc::Receiver<ObserverRequest>,
    ) -> Self {
        Self {
            controller: Some(controller),
            client: client,
            request_rx: Some(request_rx),

            previous_step: 0,
            current_step: 0,
            previous_units: HashMap::new(),
            units: HashMap::new(),

            previous_upgrades: HashSet::new(),
            upgrades: HashSet::new(),

            actions: vec![],
            spatial_actions: vec![],
        }
    }

    #[async]
    fn run(mut self) -> Result<()> {
        let queue = mem::replace(&mut self.controller, None)
            .unwrap()
            .rx
            .map(|req| Either::Control(req))
            .select(
                mem::replace(&mut self.request_rx, None)
                    .unwrap()
                    .map(|req| Either::Request(req)),
            );

        let mut observation = None;
        let mut map_info = None;

        #[async]
        for req in queue.map_err(|_| -> Error { unreachable!() }) {
            match req {
                Either::Control(ObserverControlRequest::Reset(tx)) => {
                    // clear data cache every game
                    map_info = None;

                    tx.send(())
                        .map_err(|_| Error::from("unable to ack reset"))?;
                },
                Either::Control(ObserverControlRequest::Step(tx)) => {
                    let (observer, new_observation, events, game_ended) =
                        await!(self.get_observation())?;

                    self = observer;
                    observation = Some(new_observation);

                    tx.send((events, game_ended))
                        .map_err(|_| Error::from("unable to ack step"))?;
                },

                Either::Request(ObserverRequest::Observe(tx)) => {
                    // observation should exist because step should create it
                    tx.send(Rc::clone(observation.as_ref().unwrap()))
                        .map_err(|_| {
                            Error::from("unable to return observation")
                        })?;
                },
                Either::Request(ObserverRequest::GetMapInfo(tx)) => {
                    if map_info.is_none() {
                        let (observer, new_map_info) =
                            await!(self.get_map_info())?;

                        self = observer;
                        map_info = Some(new_map_info);
                    }

                    tx.send(Rc::clone(map_info.as_ref().unwrap()))
                        .map_err(|_| Error::from("unable to return map info"))?;
                },
            }
        }

        Ok(())
    }

    #[async]
    fn get_observation(
        mut self,
    ) -> Result<(Self, Rc<Observation>, Vec<GameEvent>, bool)> {
        let mut req = sc2api::Request::new();
        req.mut_observation();

        let mut rsp = await!(self.client.clone().request(req))?;

        let mut observation = rsp.take_observation().take_observation();

        self.previous_step = self.current_step;
        self.current_step = observation.get_game_loop();
        let is_new_frame = self.current_step != self.previous_step;

        let player_common = observation.take_player_common();
        let mut raw = observation.take_raw_data();
        let mut player_raw = raw.take_player();

        self.previous_units = mem::replace(&mut self.units, HashMap::new());
        for unit in raw.take_units().into_iter() {
            match Unit::from_proto(unit) {
                Ok(mut unit) => {
                    let tag = unit.tag;

                    unit.last_seen_game_loop = self.current_step;

                    self.units.insert(tag, Rc::from(unit));
                },
                _ => (),
            }
        }

        self.previous_upgrades =
            mem::replace(&mut self.upgrades, HashSet::new());

        for u in player_raw.take_upgrade_ids().into_iter() {
            self.upgrades.insert(Upgrade::from_proto(u)?);
        }

        let mut map_state = raw.take_map_state();

        let new_observation = Rc::from(Observation {
            player_id: player_common.get_player_id(),
            previous_step: self.previous_step,
            current_step: self.current_step,
            camera_pos: {
                let camera = player_raw.get_camera();

                Point2::new(camera.get_x(), camera.get_y())
            },

            units: self.units.values().map(|u| Rc::clone(u)).collect(),
            power_sources: {
                let mut power_sources = vec![];

                for p in player_raw.take_power_sources().into_iter() {
                    power_sources.push(p.into());
                }

                power_sources
            },
            upgrades: self.upgrades.iter().map(|u| *u).collect(),
            effects: vec![],

            minerals: player_common.get_minerals(),
            vespene: player_common.get_vespene(),
            food_used: player_common.get_food_used(),
            food_cap: player_common.get_food_cap(),
            food_army: player_common.get_food_army(),
            food_workers: player_common.get_food_workers(),
            idle_worker_count: player_common.get_idle_worker_count(),
            army_count: player_common.get_army_count(),
            warp_gate_count: player_common.get_warp_gate_count(),
            larva_count: player_common.get_larva_count(),

            creep: map_state.take_creep().into_sc2()?,
            visibility: map_state.take_visibility().into_sc2()?,

            score: observation.take_score().into_sc2()?,
        });

        if is_new_frame {
            self.actions.clear();
            self.spatial_actions.clear();
        }

        for action in rsp.get_observation().get_actions() {
            if !action.has_action_raw() {
                continue;
            }

            let raw = action.get_action_raw();
            if !raw.has_unit_command() {
                continue;
            }

            let cmd = raw.get_unit_command();
            if !cmd.has_ability_id() {
                continue;
            }

            self.actions.push(cmd.clone().into_sc2()?);
        }

        for action in rsp.get_observation().get_actions() {
            if !action.has_action_feature_layer() {
                continue;
            }

            let fl = action.get_action_feature_layer();

            if fl.has_unit_command() {
                self.spatial_actions
                    .push(fl.get_unit_command().clone().into_sc2()?);
            } else if fl.has_camera_move() {
                self.spatial_actions
                    .push(fl.get_camera_move().clone().into_sc2()?);
            } else if fl.has_unit_selection_point() {
                self.spatial_actions
                    .push(fl.get_unit_selection_point().clone().into_sc2()?);
            } else if fl.has_unit_selection_rect() {
                self.spatial_actions
                    .push(fl.get_unit_selection_rect().clone().into_sc2()?);
            }
        }

        let mut events = vec![];

        if raw.has_event() {
            let event = raw.get_event();

            for tag in event.get_dead_units() {
                match self.previous_units.get(tag) {
                    Some(ref mut unit) => {
                        events.push(GameEvent::UnitDestroyed(Rc::clone(unit)));
                    },
                    None => (),
                }
            }
        }

        for ref unit in self.units.values() {
            match self.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if unit.orders.is_empty() && !prev_unit.orders.is_empty() {
                        events.push(GameEvent::UnitIdle(Rc::clone(unit)));
                    } else if unit.build_progress >= 1.0
                        && prev_unit.build_progress < 1.0
                    {
                        events.push(GameEvent::BuildingCompleted(Rc::clone(
                            unit,
                        )));
                    }
                },
                None => {
                    if unit.alliance == Alliance::Enemy
                        && unit.display_type == DisplayType::Visible
                    {
                        events.push(GameEvent::UnitDetected(Rc::clone(unit)));
                    } else {
                        events.push(GameEvent::UnitCreated(Rc::clone(unit)));
                    }

                    events.push(GameEvent::UnitIdle(Rc::clone(unit)));
                },
            }
        }

        let prev_upgrades =
            mem::replace(&mut self.previous_upgrades, HashSet::new());

        for upgrade in &self.upgrades {
            match prev_upgrades.get(upgrade) {
                Some(_) => (),
                None => {
                    events.push(GameEvent::UpgradeCompleted(*upgrade));
                },
            }
        }

        self.previous_upgrades = prev_upgrades;

        let mut nukes = 0;
        let mut nydus_worms = 0;

        for alert in observation.get_alerts() {
            match *alert {
                sc2api::Alert::NuclearLaunchDetected => nukes += 1,
                sc2api::Alert::NydusWormDetected => nydus_worms += 1,
            }
        }

        if nukes > 0 {
            events.push(GameEvent::NukesDetected(nukes));
        }

        if nydus_worms > 0 {
            events.push(GameEvent::NydusWormsDetected(nydus_worms));
        }

        let game_ended = if rsp.get_status() != sc2api::Status::in_game {
            true
        } else {
            false
        };

        Ok((self, new_observation, events, game_ended))
    }

    #[async]
    fn get_map_info(self) -> Result<(Self, Rc<MapInfo>)> {
        let mut req = sc2api::Request::new();
        req.mut_game_info();

        let mut rsp = await!(self.client.clone().request(req))?;

        let info = Rc::from(MapInfo::from_proto(rsp.take_game_info())?);

        Ok((self, info))
    }
}

#[derive(Debug)]
enum ObserverControlRequest {
    Reset(oneshot::Sender<()>),
    Step(oneshot::Sender<(Vec<GameEvent>, bool)>),
}

#[derive(Debug)]
enum ObserverRequest {
    Observe(oneshot::Sender<Rc<Observation>>),
    GetMapInfo(oneshot::Sender<Rc<MapInfo>>),
}

enum Either {
    Control(ObserverControlRequest),
    Request(ObserverRequest),
}

#[derive(Debug, Clone)]
pub struct ObserverControlTerminal {
    tx: mpsc::Sender<ObserverControlRequest>,
}
impl ObserverControlTerminal {
    #[async]
    pub fn reset(self) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverControlRequest::Reset(tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send reset"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv reset ack")))
    }

    /// returns a list of game events that have occurred since last step
    #[async]
    pub fn step(self) -> Result<(Vec<GameEvent>, bool)> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverControlRequest::Step(tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send step"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv step ack")))
    }
}

#[derive(Debug)]
pub struct ObserverControlDendrite {
    rx: mpsc::Receiver<ObserverControlRequest>,
}

/// an interface for the observer soma
#[derive(Debug, Clone)]
pub struct ObserverTerminal {
    tx: mpsc::Sender<ObserverRequest>,
}

impl ObserverTerminal {
    /// observe the current game state
    #[async]
    pub fn observe(self) -> Result<Rc<Observation>> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverRequest::Observe(tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send observation"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv observation")))
    }

    /// get information about the current map
    #[async]
    pub fn get_map_info(self) -> Result<Rc<MapInfo>> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverRequest::GetMapInfo(tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send map info request"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv map info")))
    }
}

#[derive(Debug)]
pub struct ObserverDendrite {
    rx: mpsc::Receiver<ObserverRequest>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObserverControlSynapse;

pub fn synapse() -> (ObserverTerminal, ObserverDendrite) {
    let (tx, rx) = mpsc::channel(10);

    (ObserverTerminal { tx: tx }, ObserverDendrite { rx: rx })
}

pub fn control_synapse() -> (ObserverControlTerminal, ObserverControlDendrite) {
    let (tx, rx) = mpsc::channel(1);

    (
        ObserverControlTerminal { tx: tx },
        ObserverControlDendrite { rx: rx },
    )
}

// pub struct FetchGameData {
//     transactor: Transactor,
// }

// impl FetchGameData {
//     fn fetch(axon: &Axon) -> Result<ObserverSoma> {
//         let mut req = sc2api::Request::new();
//         req.mut_data().set_unit_type_id(true);

//         let transactor = Transactor::send(axon, ClientRequest::new(req))?;

//         Ok(ObserverSoma::FetchGameData(FetchGameData {
//             transactor: transactor,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<ObserverSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.on_game_data(axon, src, result)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn on_game_data(
//         self,
//         axon: &Axon,
//         src: Handle,
//         result: ClientResult,
//     ) -> Result<ObserverSoma> {
//         let mut rsp = self.transactor.expect(src, result)?;

//         let mut unit_type_data = HashMap::new();
//         let mut ability_data = HashMap::new();
//         let mut upgrade_data = HashMap::new();
//         let mut buff_data = HashMap::new();

//         for data in rsp.mut_data().take_units().into_iter() {
//             let u = UnitTypeData::from_proto(data)?;

//             let unit_type = u.unit_type;
//             unit_type_data.insert(unit_type, u);
//         }

//         for data in rsp.mut_data().take_abilities().into_iter() {
//             let a = AbilityData::from_proto(data)?;

//             let ability = a.ability;
//             ability_data.insert(ability, a);
//         }

//         for data in rsp.mut_data().take_upgrades().into_iter() {
//             let u = UpgradeData::from_proto(data)?;

//             let upgrade = u.upgrade;
//             upgrade_data.insert(upgrade, u);
//         }

//         for data in rsp.mut_data().take_buffs().into_iter() {
//             let b = BuffData::from_proto(data)?;

//             let buff = b.buff;
//             buff_data.insert(buff, b);
//         }

//         FetchTerrainData::fetch(
//             axon,
//             unit_type_data,
//             ability_data,
//             upgrade_data,
//             buff_data,
//         )
//     }
// }
