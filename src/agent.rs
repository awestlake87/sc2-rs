
use participant::Participant;
use data::{ Unit, Upgrade };

pub trait Agent {
    fn on_game_full_start(&mut self, _: &mut Participant) {

    }
    fn on_game_start(&mut self, _: &mut Participant) {

    }
    fn on_game_end(&mut self, _: &mut Participant) {

    }
    fn on_step(&mut self, _: &mut Participant) {

    }

    fn on_unit_destroyed(&mut self, _: &mut Participant, _: &Unit) {

    }
    fn on_unit_created(&mut self, &mut Participant, _: &Unit) {

    }
    fn on_unit_idle(&mut self, &mut Participant, _: &Unit) {

    }
    fn on_upgrade_complete(&mut self, &mut Participant, _: Upgrade) {

    }
    fn on_building_complete(&mut self, &mut Participant, _: &Unit) {

    }

    fn on_nydus_detected(&mut self, &mut Participant) {

    }
    fn on_nuke_detected(&mut self, &mut Participant) {

    }
    fn on_unit_detected(&mut self, &mut Participant, _: &Unit) {

    } //param const Unit*
    //fn on_error(/*client error,protocol error */);
}
