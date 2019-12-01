use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::draw::clear_background;
use crate::ball::get_ball_graphics;
use crate::paddle::get_paddle_graphics;

const PADDLE_WIDTH : f32 = 30.0;
const PADDLE_HEIGHT: f32 = 150.0;

pub struct MainState {
    pub ball_position: (f32, f32),
    player_position: (f32,f32),
    enemy_position: (f32, f32),
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let s = MainState {
            ball_position: (400.0, 300.0),
            player_position: (40.0, 225.0),
            enemy_position: (800.0 - 70.0, 225.0),
        };
        Ok(s)
    }

    pub fn update(&mut self) {
        self.ball_position.0 += 3.0;
        self.ball_position.0 = self.ball_position.0 % 800.0;
    }

    fn draw_ball(&mut self, ctx: &mut Context) -> GameResult {
        let circle = get_ball_graphics(ctx, self.ball_position.0, self.ball_position.1)?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_player(&mut self, ctx: &mut Context) -> GameResult {
        let player_paddle = get_paddle_graphics(ctx, self.player_position.0, self.player_position.1, PADDLE_WIDTH, PADDLE_HEIGHT)?;
        graphics::draw(ctx, &player_paddle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_enemy(&mut self, ctx: &mut Context) -> GameResult {
        let enemy_paddle = get_paddle_graphics(ctx, self.enemy_position.0, self.enemy_position.1, PADDLE_WIDTH, PADDLE_HEIGHT)?;
        graphics::draw(ctx, &enemy_paddle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear_background(ctx);

        self.draw_ball(ctx)?;
        self.draw_player(ctx)?;
        self.draw_enemy(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

#[test]
fn update_moves_ball() {
    let mut state = MainState::new().unwrap();
    let init_pos = state.ball_position;
    state.update();
    let updated_pos = state.ball_position;
    assert!(init_pos != updated_pos);
}