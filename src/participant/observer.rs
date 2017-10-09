
use super::{ Participant };
//use super::super::{ Result, Error };

pub trait Observer {
    fn get_game_loop(&self) -> u32;
}

impl Observer for Participant {
    fn get_game_loop(&self) -> u32 {
        self.game_state.current_game_loop
    }
}
