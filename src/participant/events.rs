
use std::mem;
use std::rc::Rc;

use super::super::{ GameEvents, Result };
use super::super::data::{ Unit, Upgrade };
use super::{ Participant, Replay, User };

/*impl GameEvents for Participant {
    fn on_game_full_start(&mut self) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_game_full_start(self)?,
                &mut User::Observer(ref mut o) => o.on_game_full_start(self)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }

    fn on_game_start(&mut self) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_game_start(self)?,
                &mut User::Observer(ref mut o) => o.on_game_start(self)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_game_end(&mut self) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_game_end(self)?,
                &mut User::Observer(ref mut o) => o.on_game_end(self)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_step(&mut self) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_step(self)?,
                &mut User::Observer(ref mut o) => o.on_step(self)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }

    fn on_unit_destroyed(&mut self, u: &Rc<Unit>) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_unit_destroyed(self, u)?,
                &mut User::Observer(ref mut o) => {
                    o.on_unit_destroyed(self, u)?
                },
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_unit_created(&mut self, u: &Rc<Unit>) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_unit_created(self, u)?,
                &mut User::Observer(ref mut o) => o.on_unit_created(self, u)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_unit_idle(&mut self, u: &Rc<Unit>) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_unit_idle(self, u)?,
                &mut User::Observer(ref mut o) => o.on_unit_idle(self, u)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_upgrade_complete(&mut self, u: Upgrade) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_upgrade_complete(self, u)?,
                &mut User::Observer(ref mut o) => {
                    o.on_upgrade_complete(self, u)?
                },
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_building_complete(&mut self, u: &Rc<Unit>) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => {
                    a.on_building_complete(self, u)?
                },
                &mut User::Observer(ref mut o) => {
                    o.on_building_complete(self, u)?
                },
            }
            self.user = Some(user);
        }

        Ok(())
    }

    fn on_nydus_detected(&mut self) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_nydus_detected(self)?,
                &mut User::Observer(ref mut o) => o.on_nydus_detected(self)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_nuke_detected(&mut self) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_nuke_detected(self)?,
                &mut User::Observer(ref mut o) => o.on_nuke_detected(self)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }
    fn on_unit_detected(&mut self, u: &Rc<Unit>) -> Result<()> {
        if let Some(mut user) = mem::replace(&mut self.user, None) {
            match &mut user {
                &mut User::Agent(ref mut a) => a.on_unit_detected(self, u)?,
                &mut User::Observer(ref mut o) => o.on_unit_detected(self, u)?,
            }
            self.user = Some(user);
        }

        Ok(())
    }

    
}*/
