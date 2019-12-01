use ggez::graphics;
use ggez::{Context, GameResult};

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