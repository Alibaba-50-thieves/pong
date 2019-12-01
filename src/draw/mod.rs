use ggez::graphics;
use ggez::{Context};


pub fn clear_background(ctx: &mut Context) {
    graphics::clear(ctx, [0.2, 0.2, 0.3, 1.0].into());
}
