use super::util::*;

// cosine math
pub fn x_at_angle(m: f64, r: f64) -> f64 {
    m * (r * RADTODEG).cos()
}

// sine math
pub fn y_at_angle(m: f64, r: f64) -> f64 {
    m * (r * RADTODEG).sin()
}
