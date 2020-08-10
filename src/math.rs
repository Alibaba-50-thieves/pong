use ggez::nalgebra::Vector2;
use rand::Rng;

fn squared_dist(v: Vector2<f32>, w: Vector2<f32>) -> f32 {
    (v[0] - w[0]).powi(2) + (v[1] - w[1]).powi(2)
}

pub fn dist_to_segment(p: Vector2<f32>, v: Vector2<f32>, w: Vector2<f32>) -> f32 {
    let paddle_points_distance = squared_dist(v, w);
    let dot =
        ((p[0] - v[0]) * (w[0] - v[0]) + (p[1] - v[1]) * (w[1] - v[1])) / paddle_points_distance;
    let g = 0f32.max(dot.min(1f32));
    squared_dist(
        p,
        Vector2::new(v[0] + g * (w[0] - v[0]), v[1] + g * (w[1] - v[1])),
    )
    .sqrt()
}

pub fn random_direction() -> Vector2<f32> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let x = vec![rng.gen_range(-1f32, -0.3f32), rng.gen_range(0.3f32, 1f32)];

    4f32 * Vector2::new(*x.choose(&mut rng).unwrap(), rng.gen_range(-0.5f32, 0.5f32))
}

#[test]
fn rand_x_not_eq_zero() {
    (0..100).for_each(|_| assert!(random_direction()[0] != 0f32));
}

#[test]
fn distante_to_segment_in_segment_zone() {
    let a = Vector2::new(0f32, 3f32);
    let b = Vector2::new(0f32, 12f32);
    let p = Vector2::new(5f32, 7f32);

    let dist = dist_to_segment(p, a, b);

    assert_eq!(dist, 5f32)
}

#[test]
fn distante_to_segment_in_segment_border() {
    let a = Vector2::new(0f32, 3f32);
    let b = Vector2::new(0f32, 12f32);
    let p = Vector2::new(5f32, 12f32);

    let dist = dist_to_segment(p, a, b);

    assert_eq!(dist, 5f32)
}

#[test]
fn distante_to_segment_out_of_segment_zone() {
    let a = Vector2::new(0f32, 3f32);
    let b = Vector2::new(0f32, 12f32);
    let p = Vector2::new(5f32, 20f32);

    let dist = dist_to_segment(p, a, b);

    assert!(dist > 9f32)
}
