use ggez;
use ggez::event;
use ggez::{ContextBuilder, GameResult};


mod state;
mod draw;
mod ball;
mod paddle;

use state::MainState;

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("Pong", "Alibaba and the 50 Thieves");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
