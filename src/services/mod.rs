//! Contains coroutines that will asynchronously service requests from the user
//! in order to observe and affect the game state.
//!
//! This is an internal module that only exposes the structs necessary to
//! construct and communicate with these services.

pub mod action_service;
pub mod agent_service;
pub mod client_service;
pub mod computer_service;
pub mod melee_service;
pub mod observer_service;
pub mod replay_service;

/// Update scheme for the agents to use.
#[derive(Debug, Copy, Clone)]
pub enum UpdateScheme {
    /// Update as fast as possible.
    Realtime,
    /// Step the game with a fixed interval.
    Interval(u32),
}
