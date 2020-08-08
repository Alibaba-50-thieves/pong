#[cfg(test)]
mod tests;

use crate::draw::get_paddle_graphics;
use crate::state::WINDOW_HEIGHT;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub const PLAYER_SPEED: f32 = 6.5;
pub const PADDLE_WIDTH: f32 = 30.0;
pub const PADDLE_HEIGHT: f32 = 150.0;

#[derive(Default)]
pub struct Player {
    pub position: (f32, f32),
    is_moving: bool,
    velocity: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            is_moving: false,
            velocity: 0.0,
        }
    }

    pub fn move_down(&mut self) {
        self.is_moving = true;
        self.velocity = PLAYER_SPEED;
    }

    pub fn move_up(&mut self) {
        self.is_moving = true;
        self.velocity = -PLAYER_SPEED;
    }

    pub fn stop_moving(&mut self) {
        self.is_moving = false;
    }

    pub fn update(&mut self) {
        if self.is_moving {
            self.position.1 += self.velocity;
        }

        self.handle_collisions();
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let player_paddle = get_paddle_graphics(
            ctx,
            self.position.0,
            self.position.1,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )?;
        graphics::draw(ctx, &player_paddle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn handle_collisions(&mut self) {
        self.window_collision();
    }

    fn window_collision(&mut self) {
        if self.position.1 + PADDLE_HEIGHT > WINDOW_HEIGHT {
            self.position.1 = WINDOW_HEIGHT - PADDLE_HEIGHT;
        }

        if self.position.1 < 0.0 {
            self.position.1 = 0.0;
        }
    }

    pub fn paddle_vertex(&self) -> Vec<(f32, f32)> {
        let (x, y) = self.position;

        vec![
            self.position,
            (x, y + PADDLE_HEIGHT),
            (x + PADDLE_WIDTH, y),
            (x + PADDLE_WIDTH, y + PADDLE_HEIGHT),
        ]
    }
}
