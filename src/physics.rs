use nalgebra::Vector3;

use crate::body::Body;

#[derive(Clone, Copy)]
pub struct Physics {
    pub gravity_constant: f64,
    pub softening_parameter: Option<f64>,
}

impl Physics {
    pub fn get_gravity_force(self, body1: &Body, body2: &Body) -> Vector3<f64> {
        let softening_factor = match self.softening_parameter {
            Some(factor) => factor.powi(2),
            None => 0.,
        };
        let delta_position = body2.pos - body1.pos;
        let distance_squared = delta_position.norm_squared() + softening_factor;
        let distance_cubed = distance_squared.sqrt() * distance_squared;

        let force_magnitude = self.gravity_constant * (body1.mass * body2.mass) / distance_cubed;
        delta_position * force_magnitude
    }
}
