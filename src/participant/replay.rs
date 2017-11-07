
use std::mem;

use sc2_proto::sc2api::{ Request };

use super::super::{ Result };
use super::super::agent::{ Agent };
use super::super::data::{ ReplayInfo };
use super::{ Participant, Observer };

pub trait Replay {
    fn get_replay_info(&self) -> &ReplayInfo;

    fn gather_replay_info(
        &mut self, file_path: &str, download_data: bool
    )
        -> Result<()>
    ;

    fn req_start_replay(&mut self, file_path: &str)
        -> Result<()>
    ;
    fn await_replay(&mut self) -> Result<()>;
}

impl Replay for Participant {
    fn get_replay_info(&self) -> &ReplayInfo {
        match self.replay_info {
            Some(ref info) => info,
            None => panic!("replay info has not been set yet")
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
