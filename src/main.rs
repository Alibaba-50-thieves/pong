use ggez;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod ball;
mod draw;
mod player;
mod state;

use state::{MainState, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("Pong", "Alibaba and the 50 Thieves")
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
