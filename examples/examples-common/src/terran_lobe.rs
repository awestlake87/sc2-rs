
use std::rc::Rc;

use cortical;
use cortical::{ Lobe, Protocol, ResultExt, Constraint };
use rand::random;
use sc2::{
    Result,
    Message,
    Role,
    Soma,
    FrameData,
    Command,
    Ability,
    Point2,
    Tag,
    ActionTarget,
    Vector2,
    Alliance,
    UnitType,
    PlayerSetup,
    Race,
};

const TARGET_SCV_COUNT: usize = 15;

pub enum TerranLobe {
    Init(Init),
    Setup(Setup),

    InGame(InGame),
}

impl TerranLobe {
    pub fn cortex(interval: u32) -> Result<Self> {
        Ok(
            TerranLobe::Init(
                Init {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::Agent),
                        ],
                        vec![ ],
                    )?,
                    interval: interval,
                }
            )
        )
    }
}

impl Lobe for TerranLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
        -> cortical::Result<TerranLobe>
    {
        match self {
            TerranLobe::Init(state) => state.update(msg),
            TerranLobe::Setup(state) => state.update(msg),

            TerranLobe::InGame(state) => state.update(msg),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}

pub struct Init {
    soma:           Soma,
    interval:       u32,
}

impl Init {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<TerranLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Start => Setup::setup(self.soma, self.interval),

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(TerranLobe::Init(self))
        }
    }
}

pub struct Setup {
    soma:           Soma,
    interval:       u32,
}

impl Setup {
    fn setup(soma: Soma, interval: u32) -> Result<TerranLobe> {
        Ok(TerranLobe::Setup(Setup { soma: soma, interval: interval }))
    }

    fn update(mut self, msg: Protocol<Message, Role>)-> Result<TerranLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::RequestPlayerSetup(_)) => {
                    self.soma.send_req_input(
                        Role::Agent,
                        Message::PlayerSetup(
                            PlayerSetup::Player {
                                race: Race::Terran
                            }
                        )
                    )?;

                    Ok(TerranLobe::Setup(self))
                },
                Protocol::Message(_, Message::RequestUpdateInterval) => {
                    self.soma.send_req_input(
                        Role::Agent, Message::UpdateInterval(self.interval)
                    )?;

                    Ok(TerranLobe::Setup(self))
                },
                Protocol::Message(_, Message::GameStarted) => {
                    InGame::start(self.soma, self.interval)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(TerranLobe::Setup(self))
        }
    }
}

pub struct InGame {
    soma:           Soma,
    interval:       u32,
}

impl InGame {
    fn start(soma: Soma, interval: u32) -> Result<TerranLobe> {
        Ok(TerranLobe::InGame(InGame { soma: soma, interval: interval }))
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<TerranLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::Observation(frame)) => {
                    self.on_frame(frame)
                },

                Protocol::Message(_, Message::GameEnded) => {
                    Setup::setup(self.soma, self.interval)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(TerranLobe::InGame(self))
        }
    }

    fn on_frame(self, frame: Rc<FrameData>) -> Result<TerranLobe> {
        let commands = self.create_commands(&*frame)?;

        let agent = self.soma.req_input(Role::Agent)?;

        let mut messages: Vec<Message> = commands.into_iter()
            .map(|cmd| Message::Command(cmd))
            .collect()
        ;

        messages.push(Message::UpdateComplete);

        self.soma.effector()?.send_in_order(agent, messages);

        Ok(TerranLobe::InGame(self))
    }

    fn create_commands(&self, frame: &FrameData) -> Result<Vec<Command>> {
        let mut commands = vec![ ];
        // if there are marines and the command center is not found, send them
        // scouting.
        if let Some(cmd) = self.scout_with_marines(&frame) {
            commands.push(cmd);
        }

        // build supply depots if they are needed
        if let Some(cmd) = self.try_build_supply_depot(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        // build terran SCV's if they are needed
        if let Some(cmd) = self.try_build_scv(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        // build barracks if they are ready to be built
        if let Some(cmd) = self.try_build_barracks(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        // just keep building marines if possible
        if let Some(cmd) = self.try_build_marine(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        Ok(commands)
    }

    fn find_enemy_structure(&self, frame: &FrameData) -> Option<Tag> {
        let units = frame.state.filter_units(
            |u| u.alliance == Alliance::Enemy && (
                u.unit_type == UnitType::TerranCommandCenter ||
                u.unit_type == UnitType::TerranSupplyDepot ||
                u.unit_type == UnitType::TerranBarracks
            )
        );

        if !units.is_empty() {
            Some(units[0].tag)
        }
        else {
            None
        }
    }

    fn find_enemy_pos(&self, frame: &FrameData) -> Option<Point2> {
        if frame.data.terrain_info.enemy_start_locations.is_empty() {
            None
        }
        else {
            //TODO: should be random I think
            Some(frame.data.terrain_info.enemy_start_locations[0])
        }
    }

    fn scout_with_marines(&self, frame: &FrameData) -> Option<Command> {
        let units = frame.state.filter_units(
            |u| u.alliance == Alliance::Domestic &&
                u.unit_type == UnitType::TerranMarine &&
                u.orders.is_empty()
        );

        for ref u in units {
            match self.find_enemy_structure(frame) {
                Some(enemy_tag) => {
                    return Some(
                        Command::Action {
                            units: vec![ Rc::clone(u) ],
                            ability: Ability::Attack,
                            target: Some(ActionTarget::UnitTag(enemy_tag))
                        }
                    )
                },
                None => ()
            }

            match self.find_enemy_pos(frame) {
                Some(target_pos) => {
                    return Some(
                        Command::Action {
                            units: vec![ Rc::clone(u) ],
                            ability: Ability::Smart,
                            target: Some(ActionTarget::Location(target_pos))
                        }
                    )
                },
                None => ()
            }
        }

        None
    }

    fn try_build_supply_depot(&self, frame: &FrameData) -> Option<Command> {
        // if we are not supply capped, don't build a supply depot
        if frame.state.food_used + 2 <= frame.state.food_cap {
            return None
        }

        // find a random SVC to build a depot
        self.try_build_structure(frame, Ability::BuildSupplyDepot)
    }

    fn try_build_scv(&self, frame: &FrameData) -> Option<Command> {
        let scv_count = frame.state.filter_units(
            |u| u.unit_type == UnitType::TerranScv
        ).len();

        if scv_count < TARGET_SCV_COUNT {
            self.try_build_unit(
                frame, Ability::TrainScv, UnitType::TerranCommandCenter
            )
        }
        else {
            None
        }
    }

    fn try_build_barracks(&self, frame: &FrameData) -> Option<Command> {
        let scv_count = frame.state.filter_units(
            |u| u.unit_type == UnitType::TerranScv
        ).len();
        // wait until we have our quota of SCVs
        if scv_count < TARGET_SCV_COUNT {
            return None
        }

        let barracks_count = frame.state.filter_units(
            |u| u.unit_type == UnitType::TerranBarracks
        ).len();

        if barracks_count > 0 {
            return None
        }

        self.try_build_structure(frame, Ability::BuildBarracks)
    }

    fn try_build_marine(&self, frame: &FrameData) -> Option<Command> {
        self.try_build_unit(
            frame, Ability::TrainMarine, UnitType::TerranBarracks
        )
    }

    fn try_build_unit(
        &self, frame: &FrameData, ability: Ability, unit_type: UnitType
    )
        -> Option<Command>
    {
        let units = frame.state.filter_units(
            |u| u.unit_type == unit_type && u.orders.is_empty()
        );

        if units.is_empty() {
            None
        }
        else {
            Some(
                Command::Action {
                    units: vec![ Rc::clone(&units[0]) ],
                    ability: ability,
                    target: None
                }
            )
        }
    }

    fn try_build_structure(&self, frame: &FrameData, ability: Ability)
        -> Option<Command>
    {
        let units = frame.state.filter_units(
            |u| u.alliance == Alliance::Domestic
        );

        // if a unit is already building this structure, do nothing
        for u in &units {
            for o in &u.orders {
                if o.ability == ability {
                    return None
                }
            }
        }

        if !units.is_empty() {
            let r = Vector2::new(random(), random());

            let u = random::<usize>() % units.len();

            Some(
                Command::Action {
                    units: vec![ Rc::clone(&units[u]) ],
                    ability: ability,
                    target: Some(
                        ActionTarget::Location(
                            Point2::new(units[u].pos.x, units[u].pos.y)
                            + r * 5.0
                        )
                    )
                }
            )
        }
        else {
            None
        }
    }
}
