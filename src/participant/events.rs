
use std::mem;
use std::rc::Rc;

use super::super::{ GameEvents };
use super::super::data::{ Unit, Upgrade };
use super::{ Participant, Replay, User };

impl GameEvents for Participant {
    fn on_game_full_start(&mut self) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => a.on_game_full_start(self),
                    &mut User::Observer(ref mut o) => {
                        o.on_game_full_start(self)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }

    fn on_game_start(&mut self) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => a.on_game_start(self),
                    &mut User::Observer(ref mut o) => o.on_game_start(self),
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_game_end(&mut self) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => a.on_game_end(self),
                    &mut User::Observer(ref mut o) => o.on_game_end(self),
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_step(&mut self) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => a.on_step(self),
                    &mut User::Observer(ref mut o) => o.on_step(self),
                }
                self.user = Some(user);
            },
            None => ()
        }
    }

    fn on_unit_destroyed(&mut self, u: &Rc<Unit>) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => {
                        a.on_unit_destroyed(self, u)
                    },
                    &mut User::Observer(ref mut o) => {
                        o.on_unit_destroyed(self, u)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_unit_created(&mut self, u: &Rc<Unit>) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => a.on_unit_created(self, u),
                    &mut User::Observer(ref mut o) => {
                        o.on_unit_created(self, u)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_unit_idle(&mut self, u: &Rc<Unit>) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => a.on_unit_idle(self, u),
                    &mut User::Observer(ref mut o) => o.on_unit_idle(self, u),
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_upgrade_complete(&mut self, u: Upgrade) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => {
                        a.on_upgrade_complete(self, u)
                    },
                    &mut User::Observer(ref mut o) => {
                        o.on_upgrade_complete(self, u)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_building_complete(&mut self, u: &Rc<Unit>) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => {
                        a.on_building_complete(self, u)
                    },
                    &mut User::Observer(ref mut o) => {
                        o.on_building_complete(self, u)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }

    fn on_nydus_detected(&mut self) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => {
                        a.on_nydus_detected(self)
                    },
                    &mut User::Observer(ref mut o) => {
                        o.on_nydus_detected(self)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_nuke_detected(&mut self) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => {
                        a.on_nuke_detected(self)
                    },
                    &mut User::Observer(ref mut o) => {
                        o.on_nuke_detected(self)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }
    fn on_unit_detected(&mut self, u: &Rc<Unit>) {
        match mem::replace(&mut self.user, None) {
            Some(mut user) => {
                match &mut user {
                    &mut User::Agent(ref mut a) => {
                        a.on_unit_detected(self, u)
                    },
                    &mut User::Observer(ref mut o) => {
                        o.on_unit_detected(self, u)
                    },
                }
                self.user = Some(user);
            },
            None => ()
        }
    }

    fn should_ignore(&mut self) -> bool {
        //TODO: figure out how to use this value
        let player_id = 0;

        match mem::replace(&mut self.user, None) {
            Some(user) => {
                let should_ignore = match &user {
                    &User::Observer(ref o) => o.should_ignore(
                        match self.get_replay_info() {
                            Some(ref info) => info,
                            None => unimplemented!(
                                "should this be an error or a panic?"
                            )
                        },
                        player_id
                    ),
                    _ => {
                        // indicates an internal error in the library
                        panic!("user is not a replay observer")
                    }
                };

                self.user = Some(user);

                should_ignore
            },
            None => false
        }
    }
}
