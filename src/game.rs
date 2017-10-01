
use std::path::PathBuf;

pub enum Map {
    LocalMap(PathBuf)
}

pub struct GameSettings {
    pub map:            Map
}
