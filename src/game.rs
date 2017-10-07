
use std::path::PathBuf;

pub enum Map {
    LocalMap(PathBuf),
    BlizzardMap(String),
}

pub struct GameSettings {
    pub map:            Map
}
