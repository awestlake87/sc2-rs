
mod actions;
mod control;
mod debug;
mod events;
mod observer;
mod query;
mod replay;
mod spatial_actions;

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use sc2_proto::sc2api;
use sc2_proto::sc2api::{ Request, Response };

use super::{ Result, Error };
use super::agent::Agent;
use super::client::Client;
use super::data::{
    PowerSource,
    GameState,
    GameInfo,
    PlayerData,
    PlayerSetup,
    Unit,
    Tag,
    Upgrade,
    Point2,
    Action,
    SpatialAction,
    Ability,
    AbilityData,
    Score,
    ReplayInfo,
    UnitType,
    UnitTypeData,
};
use super::instance::Instance;
use super::replay_observer::ReplayObserver;

pub use self::actions::Actions;
pub use self::control::Control;
pub use self::observer::Observer;
pub use self::query::Query;
pub use self::replay::Replay;
pub use self::spatial_actions::FeatureLayerActions;

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
    base_build:                 Option<u32>,
    data_version:               Option<String>,

    observation:                sc2api::ResponseObservation,

    commands:                   Vec<Tag>,

    unit_type_data:             HashMap<UnitType, UnitTypeData>,

    previous_units:             HashMap<Tag, Rc<Unit>>,
    units:                      HashMap<Tag, Rc<Unit>>,
    power_sources:              Vec<PowerSource>,
    previous_upgrades:          Vec<Upgrade>,
    upgrades:                   Vec<Upgrade>,

    actions:                    Vec<Action>,
    requested_actions:          Vec<Action>,

    feature_layer_actions:      Vec<SpatialAction>,
    ability_data:               HashMap<Ability, AbilityData>,

    player_id:                  Option<u32>,
    camera_pos:                 Option<Point2>,
    game_state:                 GameState,
    game_info:                  GameInfo,
    player_data:                PlayerData,
    score:                      Option<Score>,

    replay_info:                Option<ReplayInfo>,

    use_generalized_ability:    bool
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
            base_build: None,
            data_version: None,

            observation: sc2api::ResponseObservation::new(),

            commands: vec![ ],

            unit_type_data: HashMap::new(),

            previous_units: HashMap::new(),
            units: HashMap::new(),
            power_sources: vec![ ],
            previous_upgrades: vec![ ],
            upgrades: vec![ ],

            actions: vec![ ],
            requested_actions: vec![ ],

            feature_layer_actions: vec![ ],

            ability_data: HashMap::new(),

            player_id: None,
            camera_pos: None,
            game_state: GameState {
                current_game_loop: 0,
                previous_game_loop: 0,
            },
            game_info: GameInfo::default(),
            player_data: PlayerData {
                minerals: 0,
                vespene: 0,
                food_cap: 0,
                food_used: 0,
                food_army: 0,
                food_workers: 0,
                idle_worker_count: 0,
                army_count: 0,
                warp_gate_count: 0,
                larva_count: 0,
            },
            score: None,

            replay_info: None,

            use_generalized_ability: true
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
            return Err(Error::Todo("app is in a bad state"))
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
            unimplemented!("errors in response");
        }
        else if pending != get_response_type(&rsp) {
            unimplemented!("unexpected response type {:#?}", rsp);
        }

        Ok(rsp)
    }

    /// ping the game instance
    pub fn ping(&mut self) -> Result<()> {
        let mut req = Request::new();

        req.mut_ping();

        self.send(req)?;
        let rsp = self.recv()?;

        self.base_build = Some(rsp.get_ping().get_base_build());
        self.data_version = Some(
            String::from(rsp.get_ping().get_data_version())
        );

        Ok(())
    }

    /// poll for an incoming message from the game instance
    pub fn poll(&self) -> bool {
        self.client.poll()
    }

    /// close the connection to the game instance
    pub fn close(&mut self) -> Result<()> {
        self.client.close()
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum AppState {
    Normal,
    Crashed,
    Timeout,
    TimeoutZombie,
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
/* put in participant
*** Cached Data ***
abilities_cached: bool;
unit_types_cached: bool;
upgrades_cached: bool;
buffs_cached: bool;
*/
