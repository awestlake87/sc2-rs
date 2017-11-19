
use super::{ Result };
use frame::{ FrameData, Command };

/// trait for all entities that can handle game events
pub trait Agent {
    /// called at the beginning of a match with the inital frame data
    fn start(&mut self, _: FrameData) -> Result<Vec<Command>> {
        Ok(vec![ ])
    }
    /// called throughout the game to step the bot
    fn update(&mut self, _: FrameData) -> Result<Vec<Command>> {
        Ok(vec![ ])
    }
    /// called at the end of a match with the final frame data
    fn end(&mut self, _: FrameData) -> Result<()> {
        Ok(())
    }
}
