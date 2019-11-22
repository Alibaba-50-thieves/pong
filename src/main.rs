use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

struct MainState;

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState;
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.2, 0.2, 0.3, 1.0].into());

        let circle = get_ball_graphics(ctx, 400.0, 300.0)?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("Pong", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}

fn get_ball_graphics(ctx: &mut ggez::Context, px: f32, py: f32) -> Result<ggez::graphics::Mesh, ggez::error::GameError> {
    graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        na::Point2::new(px, py),
        25.0,
        0.2,
        graphics::WHITE,
    )
}
