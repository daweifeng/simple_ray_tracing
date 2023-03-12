// constants

use std::f64::consts::PI;

pub const INFINITY: f64 = f64::INFINITY;

// Utility Functions

pub fn degree_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
