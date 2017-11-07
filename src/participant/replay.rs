
use std::mem;

use sc2_proto::sc2api::{ Request };

use super::super::{ Result, GameEvents };
use super::super::data::{ ReplayInfo };
use super::{ Participant, Observer };

/// UNSTABLE replay interface
pub trait Replay {
    /// get replay info that was fetched by gather replay info
    fn get_replay_info(&self) -> Option<&ReplayInfo>;

    /// ask the game instance for info about the given replay file
    fn gather_replay_info(
        &mut self, file_path: &str, download_data: bool
    )
        -> Result<()>
    ;

    /// send a start replay request to the game instance
    fn req_start_replay(&mut self, file_path: &str)
        -> Result<()>
    ;
    /// await the response after requesting to start a replay
    fn await_replay(&mut self) -> Result<()>;
}

impl Replay for Participant {
    fn get_replay_info(&self) -> Option<&ReplayInfo> {
        match self.replay_info {
            Some(ref info) => Some(&info),
            None => None
        }
    }

    fn gather_replay_info(&mut self, file_path: &str, download_data: bool)
        -> Result<()>
    {
        let mut req = Request::new();

        req.mut_replay_info().set_replay_path(file_path.to_string());
        req.mut_replay_info().set_download_data(download_data);

        self.send(req)?;

        let rsp = self.recv()?;

        self.replay_info = Some(
            ReplayInfo::from_proto(rsp.get_replay_info())
        );

        Ok(())
    }

    fn req_start_replay(&mut self, file_path: &str)
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

    fn await_replay(&mut self) -> Result<()> {
        let rsp = self.recv()?;

        let replay = rsp.get_start_replay();

        if replay.has_error() {
            println!("rsp has errors: {:#?}", rsp);
            unimplemented!("errors in start replay");
        }

        assert!(self.is_in_game());

        self.update_observation()?;

        self.on_game_start();

        Ok(())
    }
}
