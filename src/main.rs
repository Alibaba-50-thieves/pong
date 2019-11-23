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

        let paddle_width = 30.0;
        let paddle_height = 150.0;
        let paddle_x_offset = (paddle_width / 2.0) * -1.0;
        let paddle_y_offset = (paddle_height / 2.0) * -1.0;

        let player_paddle = get_paddle_graphics(ctx, 55.0, 300.0, paddle_width, paddle_height)?;
        graphics::draw(
            ctx,
            &player_paddle,
            (na::Point2::new(paddle_x_offset, paddle_y_offset),),
        )?;
        graphics::draw(
            ctx,
            &player_paddle,
            (na::Point2::new(paddle_x_offset, paddle_y_offset),),
        )?;

        let enemy_paddle =
            get_paddle_graphics(ctx, 800.0 - 55.0, 300.0, paddle_width, paddle_height)?;
        graphics::draw(
            ctx,
            &enemy_paddle,
            (na::Point2::new(paddle_x_offset, paddle_y_offset),),
        )?;

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

fn get_paddle_graphics(
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
