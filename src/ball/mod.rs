use crate::draw::get_ball_graphics;
use crate::math::{dist_to_segment, random_direction};
use crate::player::Player;
use crate::state::{WINDOW_HEIGHT, WINDOW_START, WINDOW_WIDTH};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub const BALL_RADIUS: f32 = 25f32;

pub struct Ball {
    pub position: na::Vector2<f32>,
    pub score1: usize,
    pub score2: usize,
    direction: na::Vector2<f32>,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: na::Vector2::new(x, y),
            direction: random_direction(),
            score1: 0,
            score2: 0,
        }
    }

    pub(crate) fn rolling_stones(&mut self) {
        self.position += self.direction;
        self.handle_score();
    }

    fn handle_score(&mut self) {
        match (self.position[0] < 1.0, self.position[0] > 799.0) {
            (true, false) => {
                self.score2 += 1;
                self.restart();
            }
            (false, true) => {
                self.score1 += 1;
                self.restart();
            }
            _ => {}
        }
    }

    fn restart(&mut self) {
        self.direction = random_direction();
        self.position = na::Vector2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
    }

    fn handle_collisions_players(&mut self, players: &[&Player; 2]) {
        let ball = self.position;

        players.iter().map(|p| p.paddle_vertex()).for_each(|p| {
            if p.chunks(2).any(|c|
                dist_to_segment(ball, c[0], c[1]) < BALL_RADIUS
                    && is_ball_in_correct_direction(self.position[0], self.direction[0])
            ) {
                self.direction[0] = -self.direction[0];
            }
        });
    }

    pub fn handle_collision_window(&mut self) {
        if self.position[1] + BALL_RADIUS > WINDOW_HEIGHT {
            self.direction[1] = -self.direction[1];
        }

        if self.position[1] - BALL_RADIUS < WINDOW_START {
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

fn is_ball_in_correct_direction(x_pos: f32, x_dir: f32) -> bool {
    (x_pos < WINDOW_WIDTH / 2.0 && x_dir < 0.0)
        || (x_pos > WINDOW_WIDTH / 2.0 && x_dir > 0.0)
}

#[test]
fn ball_position_and_direction_are_same() {
    assert!(is_ball_in_correct_direction(35.0, -4.0));
    assert!(is_ball_in_correct_direction(650.0, 4.0));
}

#[test]
fn ball_position_and_direction_are_reverse() {
    //Not
    assert!(!is_ball_in_correct_direction(35.0, 4.0));
    assert!(!is_ball_in_correct_direction(650.0, -4.0));
}

#[test]
fn update_moves_ball() {
    let players = &[&Player::new(-5f32, -5f32), &Player::new(-5f32, -5f32)];
    let mut ball = Ball::new(0f32, 0f32);
    let init_pos = ball.position;
    ball.update(players);
    let updated_pos = ball.position;
    assert!(init_pos != updated_pos);
}

#[test]
fn direction_changes_when_hitting_upper_wall() {
    let mut ball = Ball {
        position: na::Vector2::new(400f32, 26f32),
        direction: na::Vector2::new(0f32, -1.1f32),
        score1: 0,
        score2: 0,
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
        score1: 0,
        score2: 0,
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
        score1: 0,
        score2: 0,
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
        score1: 0,
        score2: 0,
    };
    let players = &[&Player::new(40.0, 225.0), &Player::new(-5f32, -5f32)];

    ball.update(players);

    assert_eq!(ball.direction, na::Vector2::new(4f32, 0f32));
    assert_eq!(ball.score2, 0);
    assert_eq!(ball.score1, 0);
}

#[test]
fn when_position_is_lower_than_left_side_score2_increases() {
    let mut ball = Ball {
        position: na::Vector2::new(2f32, 26f32),
        direction: na::Vector2::new(-3f32, 0f32),
        score1: 0,
        score2: 0,
    };

    ball.rolling_stones();

    assert_eq!(ball.score2, 1);
    assert_eq!(ball.score1, 0);
}

#[test]
fn when_position_is_greater_than_right_side_score1_increases() {
    let mut ball = Ball {
        position: na::Vector2::new(798f32, 26f32),
        direction: na::Vector2::new(3f32, 0f32),
        score1: 0,
        score2: 0,
    };

    ball.rolling_stones();

    assert_eq!(ball.score2, 0);
    assert_eq!(ball.score1, 1);
}

#[test]
fn when_score_happens_the_ball_will_restart() {
    let old_direction = na::Vector2::new(3f32, 0f32);
    let mut ball = Ball {
        position: na::Vector2::new(798f32, 26f32),
        direction: old_direction.clone(),
        score1: 0,
        score2: 0,
    };

    ball.rolling_stones();

    assert_eq!(ball.score1, 1);

    assert_eq!(ball.position, na::Vector2::new(400.0, 300.0));
    assert!(ball.direction != old_direction);
}
