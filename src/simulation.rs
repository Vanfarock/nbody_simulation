use std::error::Error;

use crate::{body::Body, physics::Physics, state::State};

pub struct Simulation {
    time_step: f64,
    total_steps: u32,
    gravity_constant: f64,
    softening_parameter: Option<f64>,
    bodies: Vec<Body>,
    simulation_file: &'static str,
}

impl Simulation {
    pub fn new(
        time_step: f64,
        total_steps: u32,
        gravity_constant: f64,
        softening_parameter: Option<f64>,
        bodies: Vec<Body>,
        simulation_file: &'static str,
    ) -> Self {
        Simulation {
            time_step,
            total_steps,
            gravity_constant,
            softening_parameter,
            bodies,
            simulation_file,
        }
    }

    pub fn run(mut self, save_forces: bool) -> Result<(), Box<dyn Error>> {
        let physics = Physics {
            gravity_constant: self.gravity_constant,
            softening_parameter: self.softening_parameter,
        };

        let mut state = State::new(self.simulation_file)?;
        state.save_bodies(&self.bodies)?;

        for _ in 0..self.total_steps {
            self.run_step(physics, &mut state, save_forces)?;
        }

        Ok(())
    }

    fn run_step(
        &mut self,
        physics: Physics,
        state: &mut State,
        save_forces: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut forces = vec![];
        for i in 0..self.bodies.len() {
            for j in i + 1..self.bodies.len() {
                let gravity_force = physics.get_gravity_force(&self.bodies[i], &self.bodies[j]);
                self.bodies[i].apply_force(gravity_force);
                self.bodies[j].apply_force(-gravity_force);
            }
            forces.push(self.bodies[i].force);
            self.bodies[i].update_state(self.time_step);
        }
        if save_forces {
            state.save_forces(&forces)?;
        }
        state.save_bodies(&self.bodies)?;

        Ok(())
    }
}
