use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub fn get_ball_graphics(ctx: &mut Context, px: f32, py: f32) -> GameResult<graphics::Mesh> {
    graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        na::Point2::new(px, py),
        25.0,
        0.2,
        graphics::WHITE,
    )
}
