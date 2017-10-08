use super::{ Client };

pub trait Agent {
    fn on_game_full_start(&mut self) {

    }
    fn on_game_start(&mut self) {

    }
    fn on_game_end(&mut self) {

    }
    fn on_step(&mut self) {

    }
    //param const Unit*
    fn on_unit_destroyed(&mut self) {

    }
    //param const Unit*
    fn on_unit_created(&mut self) {

    }
    //param const Unit*
    fn on_unit_idle(&mut self) {

    }
    //param upgrade ID
    fn on_upgrade_complete(&mut self) {

    }
    //param const Unit*
    fn on_building_complete(&mut self) {

    }

    fn on_nydus_detected(&mut self) {

    }
    fn on_nuke_detected(&mut self) {

    }
    //param const Unit*
    fn on_unit_detected(&mut self) {

    } //param const Unit*
    //fn on_error(/*client error,protocol error */);
}
