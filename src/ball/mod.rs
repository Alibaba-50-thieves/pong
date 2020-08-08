use crate::draw::get_ball_graphics;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const VELOCITY: f32 = 3f32;

pub struct Ball {
    pub position: (f32, f32),
    //TODO: pub direction: (f32, f32),
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: (x, y) }
    }

    pub(crate) fn rolling_stones(&mut self) {
        self.position.0 += VELOCITY;
        self.position.0 = self.position.0 % 800.0;
    }

    pub fn update(&mut self) {
        self.rolling_stones();
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let circle = get_ball_graphics(ctx, self.position.0, self.position.1)?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }
}

#[test]
fn update_moves_ball() {
    let mut ball = Ball::new(0f32, 0f32);
    let init_pos = ball.position;
    ball.update();
    let updated_pos = ball.position;
    assert!(init_pos != updated_pos);
}
