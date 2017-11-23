
use std::collections::{ HashSet, HashMap };
use std::mem;
use std::rc::Rc;
use std::time::Duration;

use sc2_proto::common;
use sc2_proto::debug;
use sc2_proto::sc2api;
use sc2_proto::sc2api::{ Request, Response };

use super::{ Result, ErrorKind, IntoProto, IntoSc2, FromProto };
use super::agent::{ Agent };
use super::client::Client;
use super::data::{
    PlayerSetup,
    Unit,
    Tag,
    Upgrade,
    Action,
    SpatialAction,
    Map,
    GamePorts,
    GameSettings,
    ActionTarget,
    ReplayInfo,
    DisplayType,
    Alliance,
    Point2,
    UnitTypeData
};
use super::frame::{
    FrameData,
    GameState,
    MapState,
    GameEvent,
    GameData,
    Command,
    DebugTextTarget
};
use super::instance::Instance;
use super::replay_observer::ReplayObserver;

/// type that allows differentiating between agents and observers
#[allow(missing_docs)]
pub enum User {
    Agent(Box<Agent>),
    Observer(Box<ReplayObserver>),
}

/// struct in charge of synchronizing the state of a user and a game instance
///
/// this struct manages a game instance and a user and is in charge of acting
/// as the middleman between the two. all state data associated with the game
/// is stored within this class, and all actions the user wishes to perform
/// are sent to the game instance through this class via the traits defined
/// in this mod.
pub struct Participant {
    /// player type
    pub player:                 PlayerSetup,
    /// managed game instance associated with the participant
    pub instance:               Instance,

    client:                     Client,
    user:                       Option<User>,

    app_state:                  AppState,
    last_status:                AppStatus,
    response_pending:           MessageType,

    previous_step:              u32,
    current_step:               u32,
    previous_units:             HashMap<Tag, Rc<Unit>>,
    units:                      HashMap<Tag, Rc<Unit>>,

    previous_upgrades:          HashSet<Upgrade>,
    upgrades:                   HashSet<Upgrade>,

    actions:                    Vec<Action>,
    feature_layer_actions:      Vec<SpatialAction>,

    game_data:                  Option<Rc<GameData>>,

    replay_info:                Option<ReplayInfo>,

    //use_generalized_ability:    bool
}

impl Participant {
    /// construct a participant
    pub fn new(
        instance: Instance,
        client: Client,
        player: PlayerSetup,
        user: Option<User>
    )
        -> Participant
    {
        Participant {
            player: player,
            instance: instance,
            client: client,
            user: user,

            app_state: AppState::Normal,
            last_status: AppStatus::Launched,
            response_pending: MessageType::Unknown,

            previous_units: HashMap::new(),
            units: HashMap::new(),

            previous_upgrades: HashSet::new(),
            upgrades: HashSet::new(),

            actions: vec![ ],
            feature_layer_actions: vec![ ],

            previous_step: 0,
            current_step: 0,

            game_data: None,

            replay_info: None,

            //use_generalized_ability: true
        }
    }

    pub fn get_game_data(&self) -> Result<Rc<GameData>> {
        if let Some(ref data) = self.game_data {
            Ok(Rc::clone(data))
        }
        else {
            bail!("no game data")
        }
    }

    /// get the current state of the instance
    pub fn get_app_state(&self) -> AppState {
        self.app_state
    }

    /// get the last status received from the game instance
    pub fn get_last_status(&self) -> AppStatus {
        self.last_status
    }

    /// check if the participant is in a game
    pub fn is_in_game(&self) -> bool {
        if self.get_app_state() == AppState::Normal {
            match self.get_last_status() {
                AppStatus::InGame => true,
                AppStatus::InReplay => true,
                _ => false
            }
        }
        else {
            false
        }
    }

    /// check if the current game is finished
    pub fn is_finished_game(&self) -> bool {
        if self.get_app_state() != AppState::Normal {
            true
        }
        else if self.is_in_game() {
            false
        }
        else if self.has_response_pending() {
            false
        }
        else {
            true
        }
    }

    /// check if the participant is ready to create a game
    pub fn is_ready_for_create_game(&self) -> bool {
        if self.get_app_state() != AppState::Normal {
            false
        }
        else if self.has_response_pending() {
            false
        }
        else {
            match self.get_last_status() {
                AppStatus::Launched => true,
                AppStatus::Ended => true,
                _ => false
            }
        }
    }

    /// check if the participant is expecting a response from the game instance
    pub fn has_response_pending(&self) -> bool {
        self.response_pending != MessageType::Unknown
    }

    /// check if the participant should leave the current game
    pub fn poll_leave_game(&mut self) -> Result<bool> {
        if self.response_pending != MessageType::LeaveGame {
            return Ok(!self.is_in_game())
        }

        if !self.poll() {
            return Ok(true)
        }

        self.recv()?;

        Ok(true)
    }

    /// send a message to the game instance
    fn send(&mut self, req: Request) -> Result<()> {
        self.response_pending = get_request_type(&req);
        self.client.send(req)
    }

    /// receive and validate a message from the game instance
    fn recv(&mut self) -> Result<Response> {
        if self.app_state != AppState::Normal {
            bail!("app is in a bad state")
        }

        let prev_status = self.last_status;
        self.last_status = AppStatus::Unknown;

        let rsp = match self.client.recv(Duration::from_secs(30)) {
            Ok(rsp) => rsp,
            Err(e) => {
                // the game instance is not responsive
                self.app_state = AppState::Timeout;
                eprintln!("probably a timeout: {}", e);
                unimplemented!("distinguish between a crash/hang");
            }
        };

        if rsp.has_status() {
            self.last_status = AppStatus::from(rsp.get_status());

            if self.last_status != prev_status {
                println!("new status: {:?}", self.last_status);
            }
        }

        let pending = self.response_pending;
        self.response_pending = MessageType::Unknown;

        if rsp.get_error().len() != 0 {
            let mut errors = vec![ ];

            for e in rsp.get_error().iter() {
                errors.push(e.clone());
            }

            bail!(ErrorKind::GameErrors(errors))
        }
        else if pending != get_response_type(&rsp) {
            unimplemented!("unexpected response type {:#?}", rsp);
        }

        Ok(rsp)
    }

    /// ping the game instance
    // pub fn ping(&mut self) -> Result<()> {
    //     let mut req = Request::new();
    //
    //     req.mut_ping();
    //
    //     self.send(req)?;
    //     let rsp = self.recv()?;
    //
    //     self.base_build = Some(rsp.get_ping().get_base_build());
    //     self.data_version = Some(
    //         String::from(rsp.get_ping().get_data_version())
    //     );
    //
    //     Ok(())
    // }

    /// poll for an incoming message from the game instance
    pub fn poll(&self) -> bool {
        self.client.poll()
    }

    /// close the connection to the game instance
    pub fn close(&mut self) -> Result<()> {
        self.client.close()
    }

    /// create a game
    pub fn create_game(
        &mut self,
        settings: &GameSettings,
        players: &Vec<PlayerSetup>,
        is_realtime: bool
    )
        -> Result<()>
    {
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
                &PlayerSetup::Computer { ref difficulty, ref race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(difficulty.to_proto());
                    setup.set_race(race.into_proto()?);
                },
                &PlayerSetup::Player { ref race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Participant);

                    setup.set_race(race.into_proto()?);
                },
                &PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }
            }

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(is_realtime);

        self.send(req)?;
        let rsp = self.recv()?;

        println!("create game rsp: {:#?}", rsp);

        Ok(())
    }

    /// request to join a multiplayer game
    pub fn req_join_game(&mut self, ports: &Option<GamePorts>) -> Result<()> {
        let mut req = sc2api::Request::new();

        match self.player {
            PlayerSetup::Computer { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            PlayerSetup::Player { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            _ => req.mut_join_game().set_race(common::Race::NoRace)
        };

        match ports {
            &Some(ref ports) => {
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
            },
            &None => (),
        }

        {
            let options = req.mut_join_game().mut_options();

            options.set_raw(true);
            options.set_score(true);
        }

        self.send(req)?;

        Ok(())
    }

    /// await the response after a join game request
    pub fn await_join_game(&mut self) -> Result<()> {
        self.recv()?;

        Ok(())
    }

    /// leave the game
    pub fn leave_game(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();

        req.mut_leave_game();

        self.send(req)?;

        let rsp = self.recv()?;

        println!("recv: {:#?}", rsp);

        Ok(())
    }

    /// tell the game instance to step (non-realtime games)
    ///
    /// I think this has to be a collaborative effort between instances.
    /// the response should only come after all participants have requested
    /// a step.
    pub fn req_step(&mut self, count: usize) -> Result<()> {
        if self.get_app_state() != AppState::Normal {
            bail!("app is in bad state")
        }

        let mut req = sc2api::Request::new();

        req.mut_step().set_count(count as u32);

        self.send(req)?;

        Ok(())
    }

    /// await the response from the game after requesting a step
    pub fn await_step(&mut self) -> Result<FrameData> {
        let rsp = self.recv()?;

        if !rsp.has_step() || rsp.get_error().len() > 0 {
            bail!("step error")
        }

        self.update_observation()
    }

    /// quit the game
    pub fn quit(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();

        req.mut_quit();

        self.send(req)
    }

    /// call the user's start function with the inital frame data
    pub fn start(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        match self.user {
            Some(User::Agent(ref mut a)) => a.start(frame),
            Some(User::Observer(ref mut o)) => o.start(frame),
            None => Ok(vec![ ])
        }
    }

    /// call the user's update function with the latest frame data
    pub fn update(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        match self.user {
            Some(User::Agent(ref mut a)) => a.update(frame),
            Some(User::Observer(ref mut o)) => o.update(frame),
            None => Ok(vec![ ])
        }
    }

    /// call the user's stop function with the final frame data
    pub fn stop(&mut self, frame: FrameData) -> Result<()> {
        match self.user {
            Some(User::Agent(ref mut a)) => a.stop(frame),
            Some(User::Observer(ref mut o)) => o.stop(frame),
            None => Ok(())
        }
    }

    /// send the list of commands to the game instance
    pub fn send_commands(&mut self, commands: Vec<Command>) -> Result<()> {
        let mut req_actions = sc2api::Request::new();
        let mut req_debug = sc2api::Request::new();

        for cmd in commands {
            match cmd {
                Command::Action { units, ability, target } => {
                    let mut a = sc2api::Action::new();
                    {
                        let cmd = a.mut_action_raw().mut_unit_command();

                        cmd.set_ability_id(ability.into_proto()? as i32);

                        match target {
                            Some(ActionTarget::UnitTag(tag)) => {
                                cmd.set_target_unit_tag(tag);
                            }
                            Some(ActionTarget::Location(pos)) => {
                                let target = cmd.mut_target_world_space_pos();
                                target.set_x(pos.x);
                                target.set_y(pos.y);
                            },
                            None => ()
                        }

                        for u in units {
                            cmd.mut_unit_tags().push(u.tag);
                        }
                    }
                    req_actions.mut_action().mut_actions().push(a);
                },

                Command::DebugText { text, target, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_text = debug::DebugText::new();

                    debug_text.set_text(text);

                    match target {
                        Some(DebugTextTarget::Screen(p)) => {
                            debug_text.mut_virtual_pos().set_x(p.x);
                            debug_text.mut_virtual_pos().set_y(p.y);
                        },
                        Some(DebugTextTarget::World(p)) => {
                            debug_text.mut_world_pos().set_x(p.x);
                            debug_text.mut_world_pos().set_y(p.y);
                            debug_text.mut_world_pos().set_z(p.z);
                        },
                        None => ()
                    }

                    debug_text.mut_color().set_r(color.0 as u32);
                    debug_text.mut_color().set_g(color.1 as u32);
                    debug_text.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_text().push(debug_text);
                    req_debug.mut_debug().mut_debug().push(cmd);
                },
                Command::DebugLine { p1, p2, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_line = debug::DebugLine::new();

                    debug_line.mut_line().mut_p0().set_x(p1.x);
                    debug_line.mut_line().mut_p0().set_y(p1.y);
                    debug_line.mut_line().mut_p0().set_z(p1.z);

                    debug_line.mut_line().mut_p1().set_x(p2.x);
                    debug_line.mut_line().mut_p1().set_y(p2.y);
                    debug_line.mut_line().mut_p1().set_z(p2.z);

                    debug_line.mut_color().set_r(color.0 as u32);
                    debug_line.mut_color().set_g(color.1 as u32);
                    debug_line.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_lines().push(debug_line);
                    req_debug.mut_debug().mut_debug().push(cmd);
                },
                Command::DebugBox { min, max, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_box = debug::DebugBox::new();

                    debug_box.mut_min().set_x(min.x);
                    debug_box.mut_min().set_y(min.y);
                    debug_box.mut_min().set_z(min.z);

                    debug_box.mut_max().set_x(max.x);
                    debug_box.mut_max().set_y(max.y);
                    debug_box.mut_max().set_z(max.z);

                    debug_box.mut_color().set_r(color.0 as u32);
                    debug_box.mut_color().set_g(color.1 as u32);
                    debug_box.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_boxes().push(debug_box);
                    req_debug.mut_debug().mut_debug().push(cmd);
                }
                Command::DebugSphere { center, radius, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_sphere = debug::DebugSphere::new();

                    debug_sphere.mut_p().set_x(center.x);
                    debug_sphere.mut_p().set_y(center.y);
                    debug_sphere.mut_p().set_z(center.z);

                    debug_sphere.set_r(radius);

                    debug_sphere.mut_color().set_r(color.0 as u32);
                    debug_sphere.mut_color().set_g(color.1 as u32);
                    debug_sphere.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_spheres().push(debug_sphere);
                    req_debug.mut_debug().mut_debug().push(cmd);
                }
            }
        }

        if !req_actions.get_action().get_actions().is_empty() {
            self.send(req_actions)?;
            self.recv()?;
        }

        if !req_debug.get_debug().get_debug().is_empty() {
            self.send(req_debug)?;
            self.recv()?;
        }

        Ok(())
    }

    /// determine if the user should ignore the replay based on it's info
    pub fn should_ignore(&mut self) -> bool {
        //TODO: figure out how to use this value
        let player_id = 0;

        match mem::replace(&mut self.user, None) {
            Some(User::Observer(o)) => {
                let should_ignore = o.should_ignore(
                    match self.get_replay_info() {
                        Some(ref info) => info,
                        None => unimplemented!(
                            "should this be an error or a panic?"
                        )
                    },
                    player_id
                );

                self.user = Some(User::Observer(o));

                should_ignore
            },
            Some(_) => panic!("user is not an observer"),
            None => false
        }
    }

    /// get game data from the game instance
    pub fn update_data(&mut self) -> Result<()> {
        let mut req_data = sc2api::Request::new();
        req_data.mut_data().set_unit_type_id(true);

        self.send(req_data)?;
        let mut rsp_data = self.recv()?;

        let mut req_terrain_info = sc2api::Request::new();
        req_terrain_info.mut_game_info();

        self.send(req_terrain_info)?;
        let mut rsp_terrain_info = self.recv()?;

        let mut game_data = GameData {
            ability_data: HashMap::new(),
            unit_type_data: HashMap::new(),
            upgrade_data: HashMap::new(),
            buff_data: HashMap::new(),

            terrain_info: rsp_terrain_info.take_game_info().into()
        };

        for data in rsp_data.mut_data().take_units().into_iter() {
            let u = Rc::from(UnitTypeData::from_proto(data)?);

            let unit_type = u.unit_type;
            game_data.unit_type_data.insert(unit_type, u);
        }

        self.game_data = Some(Rc::from(game_data));

        Ok(())
    }

    /// get the observation from the game instance and convert it to FrameData
    pub fn update_observation(&mut self) -> Result<FrameData> {
        if self.get_app_state() != AppState::Normal {
            unimplemented!("Err - app in bad state");
        }

        let mut req = sc2api::Request::new();
        req.mut_observation();

        self.send(req)?;
        let mut rsp = self.recv()?;

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
                _ => ()
            }
        }

        self.previous_upgrades = mem::replace(
            &mut self.upgrades, HashSet::new()
        );

        for u in player_raw.take_upgrade_ids().into_iter() {
            self.upgrades.insert(Upgrade::from_proto(u)?);
        }

        let new_state = GameState {
            player_id: player_common.get_player_id(),
            previous_step: self.previous_step,
            current_step: self.current_step,
            camera_pos: {
                let camera = player_raw.get_camera();

                Point2::new(camera.get_x(), camera.get_y())
            },

            units: self.units.values().map(|u| Rc::clone(u)).collect(),
            power_sources: {
                let mut power_sources = vec![ ];

                for p in player_raw.take_power_sources().into_iter() {
                    power_sources.push(p.into());
                }

                power_sources
            },
            upgrades: self.upgrades.iter().map(|u| *u).collect(),
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
            self.actions.clear();
            self.feature_layer_actions.clear();
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
                self.feature_layer_actions.push(
                    fl.get_unit_command().clone().into_sc2()?
                );
            }
            else if fl.has_camera_move() {
                self.feature_layer_actions.push(
                    fl.get_camera_move().clone().into_sc2()?
                );
            }
            else if fl.has_unit_selection_point() {
                self.feature_layer_actions.push(
                    fl.get_unit_selection_point().clone().into_sc2()?
                );
            }
            else if fl.has_unit_selection_rect() {
                self.feature_layer_actions.push(
                    fl.get_unit_selection_rect().clone().into_sc2()?
                );
            }
        }

        let mut events = vec![ ];

        if raw.has_event() {
            let event = raw.get_event();

            for tag in event.get_dead_units() {
                match self.previous_units.get(tag) {
                    Some(ref mut unit) => {
                        events.push(GameEvent::UnitDestroyed(Rc::clone(unit)));
                    },
                    None => ()
                }
            }
        }

        for ref unit in self.units.values() {
            match self.previous_units.get(&unit.tag) {
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
            &mut self.previous_upgrades, HashSet::new()
        );

        for upgrade in &self.upgrades {
            match prev_upgrades.get(upgrade) {
                Some(_) => (),
                None => {
                    events.push(GameEvent::UpgradeCompleted(*upgrade));
                }
            }
        }

        self.previous_upgrades = prev_upgrades;

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

        Ok(
            FrameData {
                state: new_state,
                data: self.get_game_data()?,
                events: events,
                map: Rc::from(
                    MapState {
                        creep: map_state.take_creep().into_sc2()?
                    }
                )
            }
        )
    }

    fn get_replay_info(&self) -> Option<&ReplayInfo> {
        match self.replay_info {
            Some(ref info) => Some(&info),
            None => None
        }
    }

    /// gather replay information from the game instance
    pub fn gather_replay_info(&mut self, file_path: &str, download_data: bool)
        -> Result<()>
    {
        let mut req = Request::new();

        req.mut_replay_info().set_replay_path(file_path.to_string());
        req.mut_replay_info().set_download_data(download_data);

        self.send(req)?;

        let mut rsp = self.recv()?;

        self.replay_info = Some(rsp.take_replay_info().into_sc2()?);

        Ok(())
    }

    /// load a replay
    pub fn req_start_replay(&mut self, file_path: &str)
        -> Result<()>
    {
        //TODO: figure out how to use this value
        let player_id = 0;

        let mut req = Request::new();

        req.mut_start_replay().set_replay_path(file_path.to_string());
        req.mut_start_replay().set_observed_player_id(player_id as i32);

        req.mut_start_replay().mut_options().set_raw(true);
        req.mut_start_replay().mut_options().set_score(true);

        self.send(req)?;

        Ok(())
    }

    /// await the response after the replay has been loaded
    pub fn await_replay(&mut self) -> Result<()> {
        let rsp = self.recv()?;

        let replay = rsp.get_start_replay();

        if replay.has_error() {
            println!("rsp has errors: {:#?}", rsp);
            unimplemented!("errors in start replay");
        }

        assert!(self.is_in_game());

        Ok(())
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum AppState {
    Normal,
    //Crashed,
    Timeout,
    //TimeoutZombie,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum AppStatus {
    Launched,
    InitGame,
    InGame,
    InReplay,
    Ended,
    Quit,
    Unknown,
}

impl From<sc2api::Status> for AppStatus {
    fn from(status: sc2api::Status) -> AppStatus {
        match status {
            sc2api::Status::launched    => AppStatus::Launched,
            sc2api::Status::init_game   => AppStatus::InitGame,
            sc2api::Status::in_game     => AppStatus::InGame,
            sc2api::Status::in_replay   => AppStatus::InReplay,
            sc2api::Status::ended       => AppStatus::Ended,
            sc2api::Status::quit        => AppStatus::Quit,
            sc2api::Status::unknown     => AppStatus::Unknown,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum MessageType {
    Unknown,
    CreateGame,
    JoinGame,
    RestartGame,
    StartReplay,
    LeaveGame,
    QuickSave,
    QuickLoad,
    Quit,
    GameInfo,
    Observation,
    Action,
    Step,
    Data,
    Query,
    SaveReplay,
    ReplayInfo,
    AvailableMaps,
    SaveMap,
    Ping,
    Debug
}

fn get_request_type(req: &Request) -> MessageType {
    if req.has_create_game() {
        MessageType::CreateGame
    }
    else if req.has_join_game() {
        MessageType::JoinGame
    }
    else if req.has_restart_game() {
        MessageType::RestartGame
    }
    else if req.has_start_replay() {
        MessageType::StartReplay
    }
    else if req.has_leave_game() {
        MessageType::LeaveGame
    }
    else if req.has_quick_save() {
        MessageType::QuickSave
    }
    else if req.has_quick_load() {
        MessageType::QuickLoad
    }
    else if req.has_quit() {
        MessageType::Quit
    }
    else if req.has_game_info() {
        MessageType::GameInfo
    }
    else if req.has_observation() {
        MessageType::Observation
    }
    else if req.has_action() {
        MessageType::Action
    }
    else if req.has_step() {
        MessageType::Step
    }
    else if req.has_data() {
        MessageType::Data
    }
    else if req.has_query() {
        MessageType::Query
    }
    else if req.has_save_replay() {
        MessageType::SaveReplay
    }
    else if req.has_replay_info() {
        MessageType::ReplayInfo
    }
    else if req.has_available_maps() {
        MessageType::AvailableMaps
    }
    else if req.has_save_map() {
        MessageType::SaveMap
    }
    else if req.has_ping() {
        MessageType::Ping
    }
    else if req.has_debug() {
        MessageType::Debug
    }
    else {
        MessageType::Unknown
    }
}

fn get_response_type(rsp: &Response) -> MessageType {
    if rsp.has_create_game() {
        MessageType::CreateGame
    }
    else if rsp.has_join_game() {
        MessageType::JoinGame
    }
    else if rsp.has_restart_game() {
        MessageType::RestartGame
    }
    else if rsp.has_start_replay() {
        MessageType::StartReplay
    }
    else if rsp.has_leave_game() {
        MessageType::LeaveGame
    }
    else if rsp.has_quick_save() {
        MessageType::QuickSave
    }
    else if rsp.has_quick_load() {
        MessageType::QuickLoad
    }
    else if rsp.has_quit() {
        MessageType::Quit
    }
    else if rsp.has_game_info() {
        MessageType::GameInfo
    }
    else if rsp.has_observation() {
        MessageType::Observation
    }
    else if rsp.has_action() {
        MessageType::Action
    }
    else if rsp.has_step() {
        MessageType::Step
    }
    else if rsp.has_data() {
        MessageType::Data
    }
    else if rsp.has_query() {
        MessageType::Query
    }
    else if rsp.has_save_replay() {
        MessageType::SaveReplay
    }
    else if rsp.has_replay_info() {
        MessageType::ReplayInfo
    }
    else if rsp.has_available_maps() {
        MessageType::AvailableMaps
    }
    else if rsp.has_save_map() {
        MessageType::SaveMap
    }
    else if rsp.has_ping() {
        MessageType::Ping
    }
    else if rsp.has_debug() {
        MessageType::Debug
    }
    else {
        MessageType::Unknown
    }
}
