use rand::Rng;
pub fn random_target_start(target_width: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..=(1.0 - target_width))
}
