use crate::draw::get_ball_graphics;
use crate::math::{dist_to_segment, random_direction};
use crate::player::Player;
use crate::state::WINDOW_HEIGHT;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub struct Ball {
    pub position: na::Vector2<f32>,
    direction: na::Vector2<f32>,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: na::Vector2::new(x, y),
            direction: random_direction(),
        }
    }

    pub(crate) fn rolling_stones(&mut self) {
        self.position += self.direction;
        self.position[0] = self.position[0] % 800.0;
    }

    fn handle_collisions_players(&mut self, players: &[&Player; 2]) {
        let ball = self.position;

        players.iter().map(|p| p.paddle_vertex()).for_each(|p| {
            if p.chunks(2)
                .any(|c| dist_to_segment(ball, c[0], c[1]) < 25f32)
            {
                self.direction[0] = -self.direction[0];
            }
        });
    }

    pub fn handle_collision_window(&mut self) {
        if self.position[1] + 25f32 > WINDOW_HEIGHT {
            self.direction[1] = -self.direction[1];
        }

        if self.position[1] - 25f32 < 0.0f32 {
            self.direction[1] = -self.direction[1];
        }
    }

    pub fn update(&mut self, players: &[&Player; 2]) {
        self.rolling_stones();
        self.handle_collisions_players(players);
        self.handle_collision_window();
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let circle = get_ball_graphics(ctx, self.position[0], self.position[1])?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }
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

#[test]
fn direction_changes_when_hitting_upper_wall() {
    let mut ball = Ball {
        position: na::Vector2::new(400f32, 26f32),
        direction: na::Vector2::new(0f32, -1.1f32),
    };

    ball.rolling_stones();
    ball.handle_collision_window();

    assert!(ball.direction[1] > 0f32);
}

#[test]
fn direction_changes_when_hitting_lower_wall() {
    let mut ball = Ball {
        position: na::Vector2::new(400f32, 573f32),
        direction: na::Vector2::new(0f32, 2.1f32),
    };

    ball.rolling_stones();
    ball.handle_collision_window();

    assert!(ball.direction[1] < 0f32);
}

#[test]
fn direction_doesnt_changes_when_away_from_wall() {
    let mut ball = Ball {
        position: na::Vector2::new(400f32, 523f32),
        direction: na::Vector2::new(0f32, 2.1f32),
    };

    ball.rolling_stones();
    ball.handle_collision_window();

    assert!(ball.direction[1] > 2f32);
}

#[test]
fn direction_changes_when_hitting_pad() {
    let mut ball = Ball {
        position: na::Vector2::new(67f32, 250f32),
        direction: na::Vector2::new(-4f32, 0f32),
    };
    let players = &[&Player::new(40.0, 225.0), &Player::new(-5f32, -5f32)];

    ball.update(players);

    assert_eq!(ball.direction, na::Vector2::new(4f32, 0f32));
}
