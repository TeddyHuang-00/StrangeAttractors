use rand::prelude::*;

pub fn random_number(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>() * (max - min) + min
}

pub fn random_color() -> String {
    let r = random_number(0.25, 1.0);
    let g = random_number(0.25, 1.0);
    let b = random_number(0.25, 1.0);
    let max = r.max(g).max(b);
    let r: i8 = ((r / max) * 255.0).floor() as i8;
    let g: i8 = ((g / max) * 255.0).floor() as i8;
    let b: i8 = ((b / max) * 255.0).floor() as i8;
    let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
    hex
}
