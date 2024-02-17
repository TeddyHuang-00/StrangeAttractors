use rand::prelude::*;

pub fn random_number(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>() * (max - min) + min
}
