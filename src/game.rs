
use std::path::PathBuf;

#[derive(Clone)]
pub enum Map {
    LocalMap(PathBuf),
    BlizzardMap(String),
}

#[derive(Copy, Clone)]
pub struct EndpointPorts {
    pub game_port:      u16,
    pub base_port:      u16
}

#[derive(Copy, Clone)]
pub struct GamePorts {
    pub shared_port:    u16,
    pub server_ports:   EndpointPorts,
    pub client_ports:   EndpointPorts
}

#[derive(Clone)]
pub struct GameSettings {
    pub map:            Map
}
