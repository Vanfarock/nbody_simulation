use std::fmt;

use nalgebra::Vector3;
use serde::Serialize;

#[derive(Serialize)]
pub struct Body {
    pub pos: Vector3<f64>,
    pub vel: Vector3<f64>,
    pub mass: f64,

    #[serde(skip_serializing)]
    pub force: Vector3<f64>,
}

impl Body {
    pub fn new(position: Vector3<f64>, velocity: Vector3<f64>, mass: f64) -> Self {
        Body {
            pos: position,
            vel: velocity,
            mass,
            force: Vector3::zeros(),
        }
    }

    pub fn apply_force(&mut self, gravity_force: Vector3<f64>) {
        self.force += gravity_force;
    }

    pub fn update_state(&mut self, time_step: f64) {
        let acceleration = if self.mass == 0. {
            Vector3::zeros()
        } else {
            self.force * (1. / self.mass)
        };
        self.vel += acceleration * time_step;
        self.pos += self.vel * time_step;
        self.force = Vector3::zeros();
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body")
            .field("position", &self.pos)
            .field("velocity", &self.vel)
            .field("mass", &self.mass)
            .field("force", &self.force)
            .finish()
    }
}
