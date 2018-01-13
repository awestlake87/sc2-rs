

use std::collections::{ HashMap, HashSet };
use std::mem;
use std::rc::Rc;

use organelle;
use organelle::{ ResultExt, Handle, Neuron, Sheath, Impulse, Dendrite };
use sc2_proto::{ sc2api };

use super::{
    Result,
    FromProto,
    IntoSc2,

    Signal,
    Synapse,
    Axon,

    FrameData,
    GameData,
    GameState,
    GameEvent,
    MapState,

    Buff,
    BuffData,
    Upgrade,
    UpgradeData,
    Ability,
    AbilityData,
    UnitType,
    UnitTypeData,
    Action,
    SpatialAction,
    Unit,
    Tag,
    Alliance,
    Point2,
    DisplayType,
};
use client::{ ClientSignal, ClientRequest, ClientResult, Transactor };

pub enum ObserverSoma {
    Init(Init),

    Started(Started),

    GameDataReady(GameDataReady),

    FetchGameData(FetchGameData),
    FetchTerrainData(FetchTerrainData),

    Observe(Observe),
}

impl ObserverSoma {
    pub fn sheath() -> Result<Sheath<Self>> {
        Ok(
            Sheath::new(
                ObserverSoma::Init(Init { }),
                vec![
                    Dendrite::RequireOne(Synapse::Observer),
                ],
                vec![
                    Dendrite::RequireOne(Synapse::Client),
                ],
            )?
        )
    }
}

impl Neuron for ObserverSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> organelle::Result<Self>
    {
        match self {
            ObserverSoma::Init(state) => state.update(axon, msg),

            ObserverSoma::Started(state) => state.update(axon, msg),

            ObserverSoma::FetchGameData(state) => state.update(axon, msg),
            ObserverSoma::FetchTerrainData(state) => state.update(axon, msg),

            ObserverSoma::GameDataReady(state) => state.update(axon, msg),

            ObserverSoma::Observe(state) => state.update(axon, msg),
        }.chain_err(
            || organelle::ErrorKind::SomaError
        )
    }
}

pub struct Init;

impl Init {
    fn update(self, _axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ObserverSoma>
    {
        match msg {
            Impulse::Start => Started::start(),

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message"),
        }
    }
}

pub struct Started;

impl Started {
    fn start() -> Result<ObserverSoma> {
        Ok(ObserverSoma::Started(Started { }))
    }

    fn restart(axon: &Axon) -> Result<ObserverSoma> {
        axon.send_req_input(Synapse::Observer, Signal::GameEnded)?;

        Ok(ObserverSoma::Started(Started { }))
    }

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ObserverSoma>
    {
        match msg {
            Impulse::Signal(_, Signal::Client(ClientSignal::Ready))
            | Impulse::Signal(_, Signal::Client(ClientSignal::Closed))
            | Impulse::Signal(_, Signal::Client(ClientSignal::Error(_))) => {
                Ok(ObserverSoma::Started(self))
            },
            Impulse::Signal(src, Signal::FetchGameData) => {
                self.on_fetch_game_data(axon, src)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }

    fn on_fetch_game_data(self, axon: &Axon, src: Handle)
        -> Result<ObserverSoma>
    {
        assert_eq!(src, axon.req_input(Synapse::Observer)?);

        FetchGameData::fetch(axon)
    }
}

pub struct FetchGameData {
    transactor:     Transactor,
}

impl FetchGameData {
    fn fetch(axon: &Axon) -> Result<ObserverSoma> {
        let mut req = sc2api::Request::new();
        req.mut_data().set_unit_type_id(true);

        let req = ClientRequest::new(req);
        let client = axon.req_output(Synapse::Client)?;

        let transactor = req.transactor(client);

        axon.effector()?.send(client, ClientSignal::Request(req).into());

        Ok(
            ObserverSoma::FetchGameData(
                FetchGameData { transactor: transactor }
            )
        )
    }

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ObserverSoma>
    {
        match msg {
            Impulse::Signal(
                src, Signal::Client(ClientSignal::Result(result))
            ) => {
                self.on_game_data(axon, src, result)
            }

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }

    fn on_game_data(self, axon: &Axon, src: Handle, result: ClientResult)
        -> Result<ObserverSoma>
    {
        let mut rsp = self.transactor.expect(src, result)?;

        let mut unit_type_data = HashMap::new();
        let mut ability_data = HashMap::new();
        let mut upgrade_data = HashMap::new();
        let mut buff_data = HashMap::new();

        for data in rsp.mut_data().take_units().into_iter() {
            let u = UnitTypeData::from_proto(data)?;

            let unit_type = u.unit_type;
            unit_type_data.insert(unit_type, u);
        }

        for data in rsp.mut_data().take_abilities().into_iter() {
            let a = AbilityData::from_proto(data)?;

            let ability = a.ability;
            ability_data.insert(ability, a);
        }

        for data in rsp.mut_data().take_upgrades().into_iter() {
            let u = UpgradeData::from_proto(data)?;

            let upgrade = u.upgrade;
            upgrade_data.insert(upgrade, u);
        }

        for data in rsp.mut_data().take_buffs().into_iter() {
            let b = BuffData::from_proto(data)?;

            let buff = b.buff;
            buff_data.insert(buff, b);
        }

        FetchTerrainData::fetch(
            axon, unit_type_data, ability_data, upgrade_data, buff_data
        )
    }
}

pub struct FetchTerrainData {
    transactor:             Transactor,

    unit_type_data:         HashMap<UnitType, UnitTypeData>,
    ability_data:           HashMap<Ability, AbilityData>,
    upgrade_data:           HashMap<Upgrade, UpgradeData>,
    buff_data:              HashMap<Buff, BuffData>,
}

impl FetchTerrainData {
    fn fetch(
        axon: &Axon,
        unit_type_data: HashMap<UnitType, UnitTypeData>,
        ability_data: HashMap<Ability, AbilityData>,
        upgrade_data: HashMap<Upgrade, UpgradeData>,
        buff_data: HashMap<Buff, BuffData>
    )
        -> Result<ObserverSoma>
    {
        let mut req = sc2api::Request::new();
        req.mut_game_info();

        let req = ClientRequest::new(req);
        let client = axon.req_output(Synapse::Client)?;
        let transactor = req.transactor(client);

        axon.effector()?.send(client, ClientSignal::Request(req).into());

        Ok(
            ObserverSoma::FetchTerrainData(
                FetchTerrainData {
                    transactor: transactor,

                    unit_type_data: unit_type_data,
                    ability_data: ability_data,
                    upgrade_data: upgrade_data,
                    buff_data: buff_data,
                }
            )
        )
    }

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ObserverSoma>
    {
        match msg {
            Impulse::Signal(
                src, Signal::Client(ClientSignal::Result(rsp))
            ) => {
                self.on_terrain_info(axon, src, rsp)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }

    fn on_terrain_info(self, axon: &Axon, src: Handle, result: ClientResult)
        -> Result<ObserverSoma>
    {
        let mut rsp = self.transactor.expect(src, result)?;

        let game_data = Rc::from(
            GameData {
                unit_type_data: self.unit_type_data,
                ability_data: self.ability_data,
                upgrade_data: self.upgrade_data,
                buff_data: self.buff_data,

                terrain_info: rsp.take_game_info().into_sc2()?
            }
        );

        GameDataReady::start(axon, game_data)
    }
}

struct ObserverData {
    previous_step:      u32,
    current_step:       u32,
    previous_units:     HashMap<Tag, Rc<Unit>>,
    units:              HashMap<Tag, Rc<Unit>>,

    previous_upgrades:  HashSet<Upgrade>,
    upgrades:           HashSet<Upgrade>,

    actions:            Vec<Action>,
    spatial_actions:    Vec<SpatialAction>,

    game_data:          Rc<GameData>,
}

pub struct GameDataReady {
    data:               ObserverData,
}

impl GameDataReady {
    fn start(axon: &Axon, game_data: Rc<GameData>) -> Result<ObserverSoma> {
        axon.send_req_input(Synapse::Observer, Signal::GameDataReady)?;

        Ok(
            ObserverSoma::GameDataReady(
                GameDataReady {
                    data: ObserverData {
                        previous_step: 0,
                        current_step: 0,
                        previous_units: HashMap::new(),
                        units: HashMap::new(),

                        previous_upgrades: HashSet::new(),
                        upgrades: HashSet::new(),

                        actions: vec![ ],
                        spatial_actions: vec![ ],
                        game_data: game_data,
                    }
                }
            )
        )
    }

    fn ready(data: ObserverData) -> Result<ObserverSoma> {
        Ok(ObserverSoma::GameDataReady(GameDataReady { data: data }))
    }

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ObserverSoma>
    {
        match msg {
            Impulse::Signal(src, Signal::Observe) => {
                assert_eq!(src, axon.req_input(Synapse::Observer)?);
                Observe::observe(axon, self.data)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message"),
        }
    }
}

pub struct Observe {
    transactor:     Transactor,

    data:           ObserverData,
}

impl Observe {
    fn observe(axon: &Axon, data: ObserverData) -> Result<ObserverSoma> {
        let mut req = sc2api::Request::new();
        req.mut_observation();

        let req = ClientRequest::new(req);
        let client = axon.req_output(Synapse::Client)?;
        let transactor = req.transactor(client);

        axon.effector()?.send(client, ClientSignal::Request(req).into());

        Ok(
            ObserverSoma::Observe(
                Observe {
                    transactor: transactor,

                    data: data
                }
            )
        )
    }

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ObserverSoma>
    {
        match msg {
            Impulse::Signal(
                src, Signal::Client(ClientSignal::Result(result))
            ) => {
                self.on_observe(axon, src, result)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message"),
        }
    }

    fn on_observe(self, axon: &Axon, src: Handle, result: ClientResult)
        -> Result<ObserverSoma>
    {
        let mut rsp = self.transactor.expect(src, result)?;

        if rsp.get_status() != sc2api::Status::in_game {
            return Started::restart(axon)
        }

        let mut observation = rsp.take_observation().take_observation();

        let mut data = self.data;

        data.previous_step = data.current_step;
        data.current_step = observation.get_game_loop();
        let is_new_frame = data.current_step != data.previous_step;

        let player_common = observation.take_player_common();
        let mut raw = observation.take_raw_data();
        let mut player_raw = raw.take_player();

        data.previous_units = mem::replace(&mut data.units, HashMap::new());
        for unit in raw.take_units().into_iter() {
            match Unit::from_proto(unit) {
                Ok(mut unit) => {
                    let tag = unit.tag;

                    unit.last_seen_game_loop = data.current_step;

                    data.units.insert(tag, Rc::from(unit));
                },
                _ => ()
            }
        }

        data.previous_upgrades = mem::replace(
            &mut data.upgrades, HashSet::new()
        );

        for u in player_raw.take_upgrade_ids().into_iter() {
            data.upgrades.insert(Upgrade::from_proto(u)?);
        }

        let new_state = GameState {
            player_id: player_common.get_player_id(),
            previous_step: data.previous_step,
            current_step: data.current_step,
            camera_pos: {
                let camera = player_raw.get_camera();

                Point2::new(camera.get_x(), camera.get_y())
            },

            units: data.units.values().map(|u| Rc::clone(u)).collect(),
            power_sources: {
                let mut power_sources = vec![ ];

                for p in player_raw.take_power_sources().into_iter() {
                    power_sources.push(p.into());
                }

                power_sources
            },
            upgrades: data.upgrades.iter().map(|u| *u).collect(),
            effects: vec![ ],

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

            score: observation.take_score().into_sc2()?,
        };

        if is_new_frame {
            data.actions.clear();
            data.spatial_actions.clear();
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

            data.actions.push(cmd.clone().into_sc2()?);
        }

        for action in rsp.get_observation().get_actions() {
            if !action.has_action_feature_layer() {
                continue;
            }

            let fl = action.get_action_feature_layer();

            if fl.has_unit_command() {
                data.spatial_actions.push(
                    fl.get_unit_command().clone().into_sc2()?
                );
            }
            else if fl.has_camera_move() {
                data.spatial_actions.push(
                    fl.get_camera_move().clone().into_sc2()?
                );
            }
            else if fl.has_unit_selection_point() {
                data.spatial_actions.push(
                    fl.get_unit_selection_point().clone().into_sc2()?
                );
            }
            else if fl.has_unit_selection_rect() {
                data.spatial_actions.push(
                    fl.get_unit_selection_rect().clone().into_sc2()?
                );
            }
        }

        let mut events = vec![ ];

        if raw.has_event() {
            let event = raw.get_event();

            for tag in event.get_dead_units() {
                match data.previous_units.get(tag) {
                    Some(ref mut unit) => {
                        events.push(GameEvent::UnitDestroyed(Rc::clone(unit)));
                    },
                    None => ()
                }
            }
        }

        for ref unit in data.units.values() {
            match data.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if unit.orders.is_empty() && !prev_unit.orders.is_empty() {
                        events.push(GameEvent::UnitIdle(Rc::clone(unit)));
                    }
                    else if unit.build_progress >= 1.0
                        && prev_unit.build_progress < 1.0
                    {
                        events.push(
                            GameEvent::BuildingCompleted(Rc::clone(unit))
                        );
                    }
                },
                None => {
                    if unit.alliance == Alliance::Enemy &&
                        unit.display_type == DisplayType::Visible
                    {
                        events.push(GameEvent::UnitDetected(Rc::clone(unit)));
                    }
                    else {
                        events.push(GameEvent::UnitCreated(Rc::clone(unit)));
                    }

                    events.push(GameEvent::UnitIdle(Rc::clone(unit)));
                }
            }
        }

        let prev_upgrades = mem::replace(
            &mut data.previous_upgrades, HashSet::new()
        );

        for upgrade in &data.upgrades {
            match prev_upgrades.get(upgrade) {
                Some(_) => (),
                None => {
                    events.push(GameEvent::UpgradeCompleted(*upgrade));
                }
            }
        }

        data.previous_upgrades = prev_upgrades;

        let mut nukes = 0;
        let mut nydus_worms = 0;

        for alert in observation.get_alerts() {
            match *alert {
                sc2api::Alert::NuclearLaunchDetected => nukes += 1,
                sc2api::Alert::NydusWormDetected => nydus_worms += 1
            }
        }

        if nukes > 0 {
            events.push(GameEvent::NukesDetected(nukes));
        }

        if nydus_worms > 0 {
            events.push(GameEvent::NydusWormsDetected(nydus_worms));
        }

        let mut map_state = raw.take_map_state();

        let frame = Rc::from(
            FrameData {
                state: new_state,
                data: Rc::clone(&data.game_data),
                events: events,
                map: Rc::from(
                    MapState {
                        creep: map_state.take_creep().into_sc2()?,
                        visibility: map_state.take_visibility().into_sc2()?
                    }
                )
            }
        );

        axon.send_req_input(Synapse::Observer, Signal::Observation(frame))?;

        GameDataReady::ready(data)
    }
}
