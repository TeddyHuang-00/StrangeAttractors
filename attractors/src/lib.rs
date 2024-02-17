mod utils;

extern crate console_error_panic_hook;

use strum::EnumString;
use utils::{random_color, random_number};
use wasm_bindgen::prelude::*;

type AttractorSystem = dyn Fn([f64; 3]) -> [f64; 3];

fn lorenz() -> Box<AttractorSystem> {
    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;
    Box::new(move |[x, y, z]| [sigma * (y - x), x * (rho - z) - y, x * y - beta * z])
}

fn rossler() -> Box<AttractorSystem> {
    let a = 0.1;
    let b = 0.1;
    let c = 14.0;
    Box::new(move |[x, y, z]| [-y - z, x + a * y, b + z * (x - c)])
}

fn thomas() -> Box<AttractorSystem> {
    let beta = 0.208186;
    Box::new(move |[x, y, z]| {
        [
            -beta * x + y.sin(),
            -beta * y + z.sin(),
            -beta * z + x.sin(),
        ]
    })
}

fn lu_chen() -> Box<AttractorSystem> {
    let a = 36.0;
    let b = 3.0;
    let c = 20.0;
    let u = -15.15;
    Box::new(move |[x, y, z]| [a * (y - x), x * (1.0 - z) + c * y + u, x * y - b * z])
}

fn dequan_li() -> Box<AttractorSystem> {
    let a = 40.0;
    let b = 11.0 / 6.0;
    let c = 0.16;
    let d = 0.65;
    let e = 55.0;
    let f = 20.0;
    Box::new(move |[x, y, z]| {
        [
            a * (y - x) + c * x * z,
            e * x + f * y - x * z,
            b * z + x * y - d * x * x,
        ]
    })
}

fn newton_leipnik() -> Box<AttractorSystem> {
    let a = 0.4;
    let b = 0.175;
    Box::new(move |[x, y, z]| {
        [
            y - a * x + 10.0 * y * z,
            -x - a * y + 5.0 * x * z,
            b * z - 5.0 * x * y,
        ]
    })
}

fn nose_hoover() -> Box<AttractorSystem> {
    let a = 1.5;
    Box::new(move |[x, y, z]| [y, -x + z * y, a - y * y])
}

fn halvorsen() -> Box<AttractorSystem> {
    let a = 1.4;
    Box::new(move |[x, y, z]| {
        [
            -a * x - 4.0 * y - 4.0 * z - y * y,
            -a * y - 4.0 * z - 4.0 * x - z * z,
            -a * z - 4.0 * x - 4.0 * y - x * x,
        ]
    })
}

fn chen_lee() -> Box<AttractorSystem> {
    let a = 5.0;
    let b = -10.0;
    let c = -0.38;
    Box::new(move |[x, y, z]| [a * x - y * z, b * y + x * z, c * z + x * y / 3.0])
}

fn bouali() -> Box<AttractorSystem> {
    let a = 0.1;
    let b = -0.1;
    let u = 1.0;
    Box::new(move |[x, y, z]| {
        [
            a * z - x * (1.0 - y),
            y * (1.0 - x * x),
            b * x - u * z * (1.0 - y),
        ]
    })
}

fn finance() -> Box<AttractorSystem> {
    let a = 0.001;
    let b = 0.2;
    let c = 1.1;
    Box::new(move |[x, y, z]| [(1.0 / b - a) * x + z + x * y, -b * y - x * x, -c * z - x])
}

fn arneodo() -> Box<AttractorSystem> {
    let a = -5.5;
    let b = 3.5;
    let c = -1.0;
    Box::new(move |[x, y, z]| [y, z, -a * x - b * y - z + c * x.powi(3)])
}

fn sprott_b() -> Box<AttractorSystem> {
    let a = 0.4;
    let b = 1.2;
    let c = 1.0;
    Box::new(move |[x, y, z]| [a * y * z, x - b * y, c - x * y])
}

fn sprott_linz_f() -> Box<AttractorSystem> {
    let a = 0.5;
    Box::new(move |[x, y, z]| [y + z, -x + a * y, x * x - z])
}

fn dadras() -> Box<AttractorSystem> {
    let a = 3.0;
    let b = 2.7;
    let c = 1.7;
    let d = 2.0;
    let e = 9.0;
    Box::new(move |[x, y, z]| [y - a * x + b * y * z, c * y - x * z + z, d * x * y - e * z])
}

#[wasm_bindgen]
#[derive(PartialEq, EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum AttractorName {
    Lorenz = "lorenz",
    Rossler = "rossler",
    Thomas = "thomas",
    LuChen = "lu_chen",
    DequanLi = "dequan_li",
    NewtonLeipnik = "newton_leipnik",
    NoseHoover = "nose_hoover",
    Halvorsen = "halvorsen",
    ChenLee = "chen_lee",
    Bouali = "bouali",
    Finance = "finance",
    Arneodo = "arneodo",
    SprottB = "sprott_b",
    SprottLinzF = "sprott_linz_f",
    Dadras = "dadras",
}

#[wasm_bindgen]
pub struct Attractor {
    points: Vec<f64>,
    colors: Vec<String>,
    system: Box<AttractorSystem>,
}

impl Default for Attractor {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Attractor {
    fn solver(&self, v: [f64; 3], dt: f64) -> [f64; 3] {
        let [x, y, z] = v;
        let k1 = (self.system)([x, y, z]);
        let k2 = (self.system)([
            x + 0.5 * dt * k1[0],
            y + 0.5 * dt * k1[1],
            z + 0.5 * dt * k1[2],
        ]);
        let k3 = (self.system)([
            x + 0.5 * dt * k2[0],
            y + 0.5 * dt * k2[1],
            z + 0.5 * dt * k2[2],
        ]);
        let k4 = (self.system)([x + dt * k3[0], y + dt * k3[1], z + dt * k3[2]]);
        [
            x + dt / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
            y + dt / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
            z + dt / 6.0 * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]),
        ]
    }

    pub fn new() -> Attractor {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        Attractor {
            points: Vec::with_capacity(30_000),
            colors: Vec::with_capacity(10_000),
            system: lorenz(),
        }
    }

    pub fn init_points(&mut self, n: usize, range: f64) {
        self.points.clear();
        for _ in 0..n {
            for __ in 0..3 {
                self.points.push(random_number(-range, range));
            }
        }
    }
    pub fn init_colors(&mut self, n: usize) {
        self.colors.clear();
        for _ in 0..n {
            self.colors.push(random_color());
        }
    }
    pub fn set_system(&mut self, name: &str) {
        self.system = match AttractorName::from_str(name) {
            Some(att) => match att {
                AttractorName::Lorenz => lorenz(),
                AttractorName::Rossler => rossler(),
                AttractorName::Thomas => thomas(),
                AttractorName::LuChen => lu_chen(),
                AttractorName::DequanLi => dequan_li(),
                AttractorName::NewtonLeipnik => newton_leipnik(),
                AttractorName::NoseHoover => nose_hoover(),
                AttractorName::Halvorsen => halvorsen(),
                AttractorName::ChenLee => chen_lee(),
                AttractorName::Bouali => bouali(),
                AttractorName::Finance => finance(),
                AttractorName::Arneodo => arneodo(),
                AttractorName::SprottB => sprott_b(),
                AttractorName::SprottLinzF => sprott_linz_f(),
                AttractorName::Dadras => dadras(),
                _ => lorenz(),
            },
            None => lorenz(),
        }
    }
    pub fn step(&mut self, dt: f64) {
        let dt = dt.min(0.02); // limit the maximum dt to avoid numerical instability
        for i in 0..self.points.len() / 3 {
            let mut v = [
                self.points[3 * i],
                self.points[3 * i + 1],
                self.points[3 * i + 2],
            ];
            v = self.solver(v, dt);
            self.points[3 * i] = v[0];
            self.points[3 * i + 1] = v[1];
            self.points[3 * i + 2] = v[2];
        }
    }
    pub fn points(&self) -> Vec<f64> {
        self.points.clone()
    }
    pub fn colors(&self) -> Vec<String> {
        self.colors.clone()
    }
}
