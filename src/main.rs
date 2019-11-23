use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

struct MainState;

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState;
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear_background(ctx);

        let circle = get_ball_graphics(ctx, 400.0, 300.0)?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult { 
    let cb = ContextBuilder::new("Pong", "Alibaba and the 50 Thieves");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}

fn get_ball_graphics(ctx: &mut Context, px: f32, py: f32) -> GameResult<graphics::Mesh> {
    graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        na::Point2::new(px, py),
        25.0,
        0.2,
        graphics::WHITE,
    )
}

fn clear_background(ctx: &mut Context) {
    graphics::clear(ctx, [0.2, 0.2, 0.3, 1.0].into());
}
