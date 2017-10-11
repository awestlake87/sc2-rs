
mod control;
mod observer;

use sc2_proto::sc2api;
use sc2_proto::sc2api::{ Request, Response };

use super::{ Result, Error };
use super::agent::Agent;
use super::client::Client;
use super::instance::Instance;
use super::player::Player;
use super::game::{ GameState };

pub use self::control::{ Control };
pub use self::observer::{ Observer };

pub struct Participant {
    pub player:                 Player,
    pub instance:               Instance,
    client:                     Client,
    pub agent:                  Box<Agent>,

    pub app_state:              AppState,
    pub last_status:            AppStatus,
    response_pending:           MessageType,
    base_build:                 Option<u32>,
    data_version:               Option<String>,

    pub player_id:              Option<u32>,
    pub game_state:             GameState,
}

impl Participant {
    pub fn new(
        instance: Instance, client: Client, player: Player, agent: Box<Agent>
    )
        -> Participant
    {
        Participant {
            player: player,
            instance: instance,
            client: client,
            agent: agent,

            app_state: AppState::Normal,
            last_status: AppStatus::Launched,
            response_pending: MessageType::Unknown,
            base_build: None,
            data_version: None,

            player_id: None,
            game_state: GameState {
                current_game_loop: 0
            }
        }
    }

    fn send(&mut self, req: Request) -> Result<()> {
        self.response_pending = get_request_type(&req);
        self.client.send(req)
    }

    fn recv(&mut self) -> Result<Response> {
        if self.app_state != AppState::Normal {
            return Err(Error::Todo("app is in a bad state"))
        }

        self.last_status = AppStatus::Unknown;

        let rsp = match self.client.recv() {
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
            println!("last_status: {:?}", self.last_status)
        }

        let pending = self.response_pending;
        self.response_pending = MessageType::Unknown;

        if rsp.get_error().len() != 0 {
            unimplemented!("errors in response");
        }
        else if pending != get_response_type(&rsp) {
            unimplemented!("unexpected response type");
        }

        Ok(rsp)
    }

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