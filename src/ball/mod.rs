use crate::draw::get_ball_graphics;
use crate::player::Player;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub struct Ball {
    pub position: (f32, f32),
    velocity: f32,
    // pub direction: (f32, f32),
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: (x, y), velocity: 3f32 }
    }

    pub(crate) fn rolling_stones(&mut self) {
        self.position.0 += self.velocity;
        self.position.0 = self.position.0 % 800.0;
    }

    fn handle_collisions(&mut self, players: &[&Player; 2]) {
        players
            .iter()
            .map(|p| p.paddle_vertex())
            .for_each(|player_paddle| {
                let left_upper_corner = player_paddle[0];
                let left_bottom_corner = player_paddle[1];
                let right_upper_corner = player_paddle[2];
                let right_bottom_corner = player_paddle[3];

                // if self.position.0 + 25 <= 
            })

        // let p = na::Vector2::new(self.position.0, self.position.1);
        // players
        //     .iter()
        //     .for_each(|player| player.paddle_vertex().chunks_exact(2).for_each(|slice| {
        //         let a = slice[0];
        //         let b = slice[1];
        //
        //         let n = b - a;
        //         let pa = a - p;
        //
        //         let c = n.dot(&pa);
        //
        //         // Closest point is between a and b
        //         let e = pa - n * (c / n.dot(&n));
        //
        //         if e.dot(&e) < 25f32 {
        //             self.velocity = -self.velocity;
        //         }
        //     }))
    }

    pub fn update(&mut self, players: &[&Player; 2]) {
        self.rolling_stones();
        self.handle_collisions(players);
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
