use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub fn clear_background(ctx: &mut Context) {
    graphics::clear(ctx, [0.2, 0.2, 0.3, 1.0].into());
}

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

pub fn get_paddle_graphics(
    ctx: &mut Context,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> GameResult<graphics::Mesh> {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(x, y, width, height),
        graphics::WHITE,
    )
}
