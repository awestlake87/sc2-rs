use std::path::PathBuf;

pub use services::replay_service::{ReplayBuilder, ReplaySink};

#[derive(Debug, Clone)]
pub enum Replay {
    LocalReplay(PathBuf),
}
