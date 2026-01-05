// Constants

use rand::{rngs::ThreadRng, Rng};

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    let mut rng: ThreadRng = rand::rng();
    // Returns a random real in [0,1).
    return rng.random::<f64>();
}

pub fn bounded_random_double(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    return min + (max - min) * random_double();
}

#[allow(dead_code)]
pub fn bounded_random_int(min: i64, max: i64) -> i64 {
    // Returns a random integer in [min,max].
    return bounded_random_double(min as f64, (max + 1) as f64) as i64;
}
