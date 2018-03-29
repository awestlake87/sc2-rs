use std::collections::{HashMap, HashSet};
use std::mem;
use std::rc::Rc;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use sc2_proto::sc2api;
use tokio_core::reactor;

use super::{Error, FromProto, IntoSc2, Result};
use agent::Event;
use client::ProtoClient;
use data::{
    Ability,
    AbilityData,
    Action,
    Alliance,
    Buff,
    BuffData,
    DisplayType,
    Effect,
    ImageData,
    MapInfo,
    Point2,
    PowerSource,
    Score,
    Tag,
    Unit,
    UnitType,
    UnitTypeData,
    Upgrade,
    UpgradeData,
    Visibility,
};

/// state of the game (changes every frame)
#[derive(Debug, Clone)]
pub struct Observation {
    /// the player id associated with the participant
    player_id: u32,
    /// the previous game step
    previous_step: u32,
    /// the current game step
    current_step: u32,
    /// position of the center of the camera
    camera_pos: Point2,

    /// a list of all known units at the moment
    units: Vec<Rc<Unit>>,

    /// all power sources associated with the current player
    power_sources: Vec<PowerSource>,
    /// all active effects in vision of the current player
    effects: Vec<Effect>,
    /// all upgrades
    upgrades: Vec<Upgrade>,

    /// current mineral count
    minerals: u32,
    /// current vespene count
    vespene: u32,
    /// the total supply cap given the players max supply
    food_cap: u32,
    /// the total supply used by the player
    food_used: u32,
    /// the total supply consumed by army units alone
    food_army: u32,
    /// the total supply consumed by workers alone
    food_workers: u32,
    /// the number of workers that currently have no orders
    idle_worker_count: u32,
    /// the number of army units
    army_count: u32,
    /// the number of warp gates owned by the player
    warp_gate_count: u32,
    /// the number of larva owned by the player
    larva_count: u32,

    /// creep image (sample pixels to find tiles with creep)
    creep: ImageData,
    /// visibility image (sample pixels to find visible tiles)
    visibility: ImageData,

    /// detailed current set of scores
    score: Score,
}

impl Observation {
    /// the player id associated with the participant
    pub fn get_player_id(&self) -> u32 {
        self.player_id
    }
    /// the previous game step
    pub fn get_previous_step(&self) -> u32 {
        self.previous_step
    }
    /// the current game step
    pub fn get_current_step(&self) -> u32 {
        self.current_step
    }
    /// position of the center of the camera
    pub fn get_camera_pos(&self) -> Point2 {
        self.camera_pos
    }

    /// a list of all known units at the moment
    pub fn get_units(&self) -> &[Rc<Unit>] {
        &self.units
    }

    /// all power sources associated with the current player
    pub fn get_power_sources(&self) -> &[PowerSource] {
        &self.power_sources
    }
    /// all active effects in vision of the current player
    pub fn get_effects(&self) -> &[Effect] {
        &self.effects
    }
    /// all upgrades
    pub fn get_upgrades(&self) -> &[Upgrade] {
        &self.upgrades
    }

    /// current mineral count
    pub fn get_minerals(&self) -> u32 {
        self.minerals
    }
    /// current vespene count
    pub fn get_vespene(&self) -> u32 {
        self.vespene
    }
    /// the total supply cap given the players max supply
    pub fn get_food_cap(&self) -> u32 {
        self.food_cap
    }
    /// the total supply used by the player
    pub fn get_food_used(&self) -> u32 {
        self.food_used
    }
    /// the total supply consumed by army units alone
    pub fn get_food_army(&self) -> u32 {
        self.food_army
    }
    /// the total supply consumed by workers alone
    pub fn get_food_workers(&self) -> u32 {
        self.food_workers
    }
    /// the number of workers that currently have no orders
    pub fn get_idle_worker_count(&self) -> u32 {
        self.idle_worker_count
    }
    /// the number of army units
    pub fn get_army_count(&self) -> u32 {
        self.army_count
    }
    /// the number of warp gates owned by the player
    pub fn get_warp_gate_count(&self) -> u32 {
        self.warp_gate_count
    }
    /// the number of larva owned by the player
    pub fn get_larva_count(&self) -> u32 {
        self.larva_count
    }

    /// creep image (sample pixels to find tiles with creep)
    pub fn get_creep(&self) -> &ImageData {
        &self.creep
    }
    /// visibility image (sample pixels to find visible tiles)
    pub fn get_visibility(&self) -> &ImageData {
        &self.visibility
    }

    /// detailed current set of scores
    pub fn get_score(&self) -> &Score {
        &self.score
    }

    /// filter all units based on a custom condition
    pub fn filter_units<F>(&self, filter: F) -> Vec<Rc<Unit>>
    where
        F: Fn(&Unit) -> bool,
    {
        self.units
            .iter()
            .filter(|u| filter(u))
            .map(|u| Rc::clone(u))
            .collect()
    }
    /// check if the given point contains creep
    pub fn sample_creep(&self, _: Point2) -> bool {
        unimplemented!("has creep")
    }
    /// get the visibility of the given point for the current player
    pub fn sample_visibility(&self, _: Point2) -> Visibility {
        unimplemented!("get visibility")
    }
}

pub struct ObserverBuilder {
    client: Option<ProtoClient>,

    control_tx: mpsc::Sender<ObserverControlRequest>,
    control_rx: mpsc::Receiver<ObserverControlRequest>,

    user_tx: mpsc::Sender<ObserverRequest>,
    user_rx: mpsc::Receiver<ObserverRequest>,
}

impl ObserverBuilder {
    pub fn new() -> Self {
        let (control_tx, control_rx) = mpsc::channel(10);
        let (user_tx, user_rx) = mpsc::channel(10);

        Self {
            client: None,

            control_tx: control_tx,
            control_rx: control_rx,

            user_tx: user_tx,
            user_rx: user_rx,
        }
    }

    pub fn proto_client(self, client: ProtoClient) -> Self {
        Self {
            client: Some(client),
            ..self
        }
    }

    pub fn add_control_client(&self) -> ObserverControlClient {
        ObserverControlClient {
            tx: self.control_tx.clone(),
        }
    }

    pub fn add_client(&self) -> ObserverClient {
        ObserverClient {
            tx: self.user_tx.clone(),
        }
    }

    pub fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        let task = ObserverService::new(
            self.control_rx,
            self.client.unwrap(),
            self.user_rx,
        );

        handle.spawn(task.run().map_err(move |e| panic!("{:#?}", e)));

        Ok(())
    }
}

struct ObserverService {
    controller: Option<mpsc::Receiver<ObserverControlRequest>>,
    client: ProtoClient,
    request_rx: Option<mpsc::Receiver<ObserverRequest>>,

    previous_step: u32,
    current_step: u32,
    previous_units: HashMap<Tag, Rc<Unit>>,
    units: HashMap<Tag, Rc<Unit>>,

    previous_upgrades: HashSet<Upgrade>,
    upgrades: HashSet<Upgrade>,

    actions: Vec<Action>,
    // spatial_actions: Vec<SpatialAction>,
}

impl ObserverService {
    fn new(
        controller: mpsc::Receiver<ObserverControlRequest>,
        client: ProtoClient,
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
            // spatial_actions: vec![],
        }
    }

    #[async]
    fn run(mut self) -> Result<()> {
        let queue = mem::replace(&mut self.controller, None)
            .unwrap()
            .map(|req| Either::Control(req))
            .select(
                mem::replace(&mut self.request_rx, None)
                    .unwrap()
                    .map(|req| Either::Request(req)),
            );

        let mut observation = None;
        let mut map_info = None;
        let mut unit_data = None;
        let mut ability_data = None;
        let mut upgrade_data = None;
        let mut buff_data = None;

        #[async]
        for req in queue.map_err(|_| -> Error { unreachable!() }) {
            match req {
                Either::Control(ObserverControlRequest::Reset(tx)) => {
                    // clear data cache every game
                    map_info = None;
                    unit_data = None;

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
                Either::Request(ObserverRequest::GetUnitData(_))
                | Either::Request(ObserverRequest::GetAbilityData(_))
                | Either::Request(ObserverRequest::GetUpgradeData(_))
                | Either::Request(ObserverRequest::GetBuffData(_)) => {
                    if unit_data.is_none() {
                        let (
                            observer,
                            new_unit_data,
                            new_ability_data,
                            new_upgrade_data,
                            new_buff_data,
                        ) = await!(self.get_game_data())?;

                        self = observer;
                        unit_data = Some(new_unit_data);
                        ability_data = Some(new_ability_data);
                        upgrade_data = Some(new_upgrade_data);
                        buff_data = Some(new_buff_data);
                    }

                    match req {
                        Either::Request(ObserverRequest::GetUnitData(tx)) => {
                            tx.send(Rc::clone(unit_data.as_ref().unwrap()))
                                .map_err(|_| {
                                    Error::from("unable to return unit data")
                                })?;
                        },
                        Either::Request(ObserverRequest::GetAbilityData(
                            tx,
                        )) => {
                            tx.send(Rc::clone(ability_data.as_ref().unwrap()))
                                .map_err(|_| {
                                    Error::from("unable to return ability data")
                                })?;
                        },
                        Either::Request(ObserverRequest::GetUpgradeData(
                            tx,
                        )) => {
                            tx.send(Rc::clone(upgrade_data.as_ref().unwrap()))
                                .map_err(|_| {
                                    Error::from("unable to return upgrade data")
                                })?;
                        },
                        Either::Request(ObserverRequest::GetBuffData(tx)) => {
                            tx.send(Rc::clone(buff_data.as_ref().unwrap()))
                                .map_err(|_| {
                                    Error::from("unable to return buff data")
                                })?;
                        },

                        _ => unreachable!(),
                    }
                },
            }
        }

        Ok(())
    }

    #[async]
    fn get_observation(
        mut self,
    ) -> Result<(Self, Rc<Observation>, Vec<Event>, bool)> {
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
                    let tag = unit.get_tag();

                    unit.set_last_seen_step(self.current_step);

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

            units: self.units
                .values()
                .map(|u| Rc::clone(u))
                .collect(),
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
            // self.spatial_actions.clear();
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

        // for action in rsp.get_observation().get_actions() {
        //     if !action.has_action_feature_layer() {
        //         continue;
        //     }

        //     let fl = action.get_action_feature_layer();

        //     if fl.has_unit_command() {
        //         self.spatial_actions
        //             .push(fl.get_unit_command().clone().into_sc2()?);
        //     } else if fl.has_camera_move() {
        //         self.spatial_actions
        //             .push(fl.get_camera_move().clone().into_sc2()?);
        //     } else if fl.has_unit_selection_point() {
        //         self.spatial_actions
        //             .push(fl.get_unit_selection_point().clone().into_sc2()?);
        //     } else if fl.has_unit_selection_rect() {
        //         self.spatial_actions
        //             .push(fl.get_unit_selection_rect().clone().into_sc2()?);
        //     }
        // }

        let mut events = vec![];

        if raw.has_event() {
            let event = raw.get_event();

            for tag in event.get_dead_units() {
                match self.previous_units.get(tag) {
                    Some(ref mut unit) => {
                        events.push(Event::UnitDestroyed(Rc::clone(unit)));
                    },
                    None => (),
                }
            }
        }

        for ref unit in self.units.values() {
            match self.previous_units.get(&unit.get_tag()) {
                Some(ref prev_unit) => {
                    if unit.get_orders().is_empty()
                        && !prev_unit.get_orders().is_empty()
                    {
                        events.push(Event::UnitIdle(Rc::clone(unit)));
                    } else if unit.get_build_progress() >= 1.0
                        && prev_unit.get_build_progress() < 1.0
                    {
                        events.push(Event::BuildingCompleted(Rc::clone(unit)));
                    }
                },
                None => {
                    if unit.get_alliance() == Alliance::Enemy
                        && unit.get_display_type() == DisplayType::Visible
                    {
                        events.push(Event::UnitDetected(Rc::clone(unit)));
                    } else {
                        events.push(Event::UnitCreated(Rc::clone(unit)));
                    }

                    events.push(Event::UnitIdle(Rc::clone(unit)));
                },
            }
        }

        let prev_upgrades =
            mem::replace(&mut self.previous_upgrades, HashSet::new());

        for upgrade in &self.upgrades {
            match prev_upgrades.get(upgrade) {
                Some(_) => (),
                None => {
                    events.push(Event::UpgradeCompleted(*upgrade));
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
            events.push(Event::NukesDetected(nukes));
        }

        if nydus_worms > 0 {
            events.push(Event::NydusWormsDetected(nydus_worms));
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

    #[async]
    fn get_game_data(
        self,
    ) -> Result<(
        Self,
        Rc<HashMap<UnitType, UnitTypeData>>,
        Rc<HashMap<Ability, AbilityData>>,
        Rc<HashMap<Upgrade, UpgradeData>>,
        Rc<HashMap<Buff, BuffData>>,
    )> {
        let mut req = sc2api::Request::new();
        req.mut_data().set_unit_type_id(true);

        let mut rsp = await!(self.client.clone().request(req))?;

        let mut unit_type_data = HashMap::new();
        let mut ability_data = HashMap::new();
        let mut upgrade_data = HashMap::new();
        let mut buff_data = HashMap::new();

        for data in rsp.mut_data().take_units().into_iter() {
            let u = UnitTypeData::from_proto(data)?;

            let unit_type = u.get_id();
            unit_type_data.insert(unit_type, u);
        }

        for data in rsp.mut_data().take_abilities().into_iter() {
            let a = AbilityData::from_proto(data)?;

            let ability = a.get_id();
            ability_data.insert(ability, a);
        }

        for data in rsp.mut_data().take_upgrades().into_iter() {
            let u = UpgradeData::from_proto(data)?;

            let upgrade = u.get_id();
            upgrade_data.insert(upgrade, u);
        }

        for data in rsp.mut_data().take_buffs().into_iter() {
            let b = BuffData::from_proto(data)?;

            let buff = b.get_id();
            buff_data.insert(buff, b);
        }

        Ok((
            self,
            Rc::from(unit_type_data),
            Rc::from(ability_data),
            Rc::from(upgrade_data),
            Rc::from(buff_data),
        ))
    }
}

#[derive(Debug)]
enum ObserverControlRequest {
    Reset(oneshot::Sender<()>),
    Step(oneshot::Sender<(Vec<Event>, bool)>),
}

#[derive(Debug)]
enum ObserverRequest {
    Observe(oneshot::Sender<Rc<Observation>>),

    GetMapInfo(oneshot::Sender<Rc<MapInfo>>),

    GetUnitData(oneshot::Sender<Rc<HashMap<UnitType, UnitTypeData>>>),
    GetAbilityData(oneshot::Sender<Rc<HashMap<Ability, AbilityData>>>),
    GetUpgradeData(oneshot::Sender<Rc<HashMap<Upgrade, UpgradeData>>>),
    GetBuffData(oneshot::Sender<Rc<HashMap<Buff, BuffData>>>),
}

enum Either {
    Control(ObserverControlRequest),
    Request(ObserverRequest),
}

#[derive(Debug, Clone)]
pub struct ObserverControlClient {
    tx: mpsc::Sender<ObserverControlRequest>,
}
impl ObserverControlClient {
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
    pub fn step(self) -> Result<(Vec<Event>, bool)> {
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

/// an interface for the observer soma
#[derive(Debug, Clone)]
pub struct ObserverClient {
    tx: mpsc::Sender<ObserverRequest>,
}

impl ObserverClient {
    /// observe the current game state
    pub fn observe(
        &self,
    ) -> impl Future<Item = Rc<Observation>, Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ObserverRequest::Observe(tx))
                    .map(|_| ())
                    .map_err(|_| Error::from("unable to send observation"))
            )?;

            await!(rx.map_err(|_| Error::from("unable to recv observation")))
        }
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

    /// get data about each unit type
    #[async]
    pub fn get_unit_data(self) -> Result<Rc<HashMap<UnitType, UnitTypeData>>> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverRequest::GetUnitData(tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send unit data request"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv unit data")))
    }

    /// get data about each ability
    #[async]
    pub fn get_ability_data(self) -> Result<Rc<HashMap<Ability, AbilityData>>> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverRequest::GetAbilityData(tx))
                .map(|_| ())
                .map_err(|_| Error::from(
                    "unable to send ability data request"
                ))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv ability data")))
    }

    /// get data about each upgrade
    #[async]
    pub fn get_upgrade_data(self) -> Result<Rc<HashMap<Upgrade, UpgradeData>>> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverRequest::GetUpgradeData(tx))
                .map(|_| ())
                .map_err(|_| Error::from(
                    "unable to send upgrade data request"
                ))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv upgrade data")))
    }

    /// get data about each buff
    #[async]
    pub fn get_buff_data(self) -> Result<Rc<HashMap<Buff, BuffData>>> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ObserverRequest::GetBuffData(tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send buff data request"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv buff data")))
    }
}
