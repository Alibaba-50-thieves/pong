use crate::player::{Player, PLAYER_SPEED};

#[test]
fn when_player_is_moving_stops_it() {
    let mut player = Player::default();
    player.is_moving = true;

    player.stop_moving();

    assert_eq!(player.is_moving, false);
}

#[test]
fn move_down_makes_velocity_positive() {
    let mut player = Player::default();
    player.move_down();

    assert_eq!(player.is_moving, true);
    assert_eq!(player.velocity, PLAYER_SPEED);
}

#[test]
fn move_up_makes_velocity_negative() {
    let mut player = Player::default();
    player.move_up();

    assert_eq!(player.is_moving, true);
    assert_eq!(player.velocity, -PLAYER_SPEED);
}

#[test]
fn update_when_player_is_moving() {
    let mut player = Player::default();
    player.move_down();

    let old_y_position = player.position.1;

    player.update();

    assert!(player.position.1 > old_y_position);
}

#[test]
fn update_when_player_is_not_moving() {
    let mut player = Player::default();

    let old_y_position = player.position.1;

    player.update();

    assert_eq!(player.position.1, old_y_position);
}
