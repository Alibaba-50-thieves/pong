#[cfg(test)]
mod tests;

pub const PLAYER_SPEED: f32 = 6.5;

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
    }
}
