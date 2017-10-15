
use data::{ Unit, Upgrade };

pub trait Agent {
    fn on_game_full_start(&mut self) {

    }
    fn on_game_start(&mut self) {

    }
    fn on_game_end(&mut self) {

    }
    fn on_step(&mut self) {

    }

    fn on_unit_destroyed(&mut self, _: &Unit) {

    }
    fn on_unit_created(&mut self, _: &Unit) {

    }
    fn on_unit_idle(&mut self, _: &Unit) {

    }
    fn on_upgrade_complete(&mut self, _: Upgrade) {

    }
    fn on_building_complete(&mut self, _: &Unit) {

    }

    fn on_nydus_detected(&mut self) {

    }
    fn on_nuke_detected(&mut self) {

    }
    fn on_unit_detected(&mut self, _: &Unit) {

    } //param const Unit*
    //fn on_error(/*client error,protocol error */);
}
