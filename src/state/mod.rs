use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::ball::get_ball_graphics;
use crate::draw::clear_background;
use crate::paddle::get_paddle_graphics;
use crate::player::{Player, PADDLE_HEIGHT, PADDLE_WIDTH};

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub struct MainState {
    pub ball_position: (f32, f32),
    player: Player,
    enemy_position: (f32, f32),
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let s = MainState {
            ball_position: (400.0, 300.0),
            player: Player::new(40.0, 225.0),
            enemy_position: (800.0 - 70.0, 225.0),
        };
        Ok(s)
    }

    pub fn update(&mut self) {
        self.ball_position.0 += 3.0;
        self.ball_position.0 = self.ball_position.0 % 800.0;

        self.player.update();
    }

    fn draw_ball(&mut self, ctx: &mut Context) -> GameResult {
        let circle = get_ball_graphics(ctx, self.ball_position.0, self.ball_position.1)?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_player(&mut self, ctx: &mut Context) -> GameResult {
        let player_paddle = get_paddle_graphics(
            ctx,
            self.player.position.0,
            self.player.position.1,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )?;
        graphics::draw(ctx, &player_paddle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_enemy(&mut self, ctx: &mut Context) -> GameResult {
        let enemy_paddle = get_paddle_graphics(
            ctx,
            self.enemy_position.0,
            self.enemy_position.1,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )?;
        graphics::draw(ctx, &enemy_paddle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }
}

impl EventHandler for MainState {
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

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => {
                self.player.move_up();
            }
            KeyCode::S => {
                self.player.move_down();
            }
            _ => (),
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::W | KeyCode::S => {
                self.player.stop_moving();
            }
            _ => (),
        };
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
