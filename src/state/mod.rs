use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::ball::Ball;
use crate::draw::clear_background;
use crate::player::{Player, PADDLE_WIDTH};

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub const GOAL_DISTANCE: f32 = 40.0;

pub struct MainState {
    ball: Ball,
    player1: Player,
    player2: Player,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let s = MainState {
            ball: Ball::new(400.0, 300.0),
            player1: Player::new(40.0, 225.0),
            player2: Player::new(WINDOW_WIDTH - (PADDLE_WIDTH + GOAL_DISTANCE), 225.0),
        };
        Ok(s)
    }

    pub fn update(&mut self) {
        self.ball.update();
        self.player1.update();
        self.player2.update();
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear_background(ctx);

        self.ball.draw(ctx)?;
        self.player1.draw(ctx)?;
        self.player2.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => {
                self.player1.move_up();
            }
            KeyCode::S => {
                self.player1.move_down();
            }
            KeyCode::Up => {
                self.player2.move_up();
            }
            KeyCode::Down => {
                self.player2.move_down();
            }
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::W | KeyCode::S => {
                self.player1.stop_moving();
            }
            KeyCode::Up | KeyCode::Down => {
                self.player2.stop_moving();
            }
            _ => (),
        };
    }
}
