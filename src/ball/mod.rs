use crate::draw::get_ball_graphics;
use crate::player::{Player};
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
        let ball_vec = na::Vector2::new(self.position.0, self.position.1);

        players.iter()
            .map(|p| p.paddle_vertex())
            .for_each(|p| {
                if p.chunks(2).any(|c|
                    dist_to_segment(ball_vec, c[0], c[1]) < 25f32
                ) {
                    self.velocity = -self.velocity;
                }
            });
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
  
  fn squared_dist (v: na::Vector2<f32>, w: na::Vector2<f32>) -> f32{
    (v[0] - w[0]).powi(2) + (v[1] - w[1]).powi(2)
  }
  
  // p - point
  // v - start point of segment
  // w - end point of segment
  fn dist_to_segment (p: na::Vector2<f32>, v: na::Vector2<f32>, w: na::Vector2<f32>) -> f32{
    let paddle_points_distance = squared_dist(v, w);
    let dot = ((p[0] - v[0]) * (w[0] - v[0]) + (p[1] - v[1]) * (w[1] - v[1])) / paddle_points_distance;
    let g = 0f32.max(dot.min(1f32));
    return squared_dist(p, na::Vector2::new( v[0] + g * (w[0] - v[0]), v[1] + g * (w[1] - v[1]) )).sqrt()
  }


#[test]
fn update_moves_ball() {
    let playes = &[&Player::new(-5f32, -5f32), &Player::new(-5f32, -5f32)];
    let mut ball = Ball::new(0f32, 0f32);
    let init_pos = ball.position;
    ball.update(playes);
    let updated_pos = ball.position;
    assert!(init_pos != updated_pos);
}
