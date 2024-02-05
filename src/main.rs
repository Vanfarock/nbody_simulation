mod body;
mod physics;
mod simulation;
mod state;

use body::Body;
use nalgebra::Vector3;
use simulation::Simulation;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let bodies = generate_bodies();
    let simulation = Simulation::new(1_000., 10_000, 6.67430e-11, Some(1e-5), bodies, "test.json");
    simulation.run()?;

    Ok(())
}

fn generate_bodies() -> Vec<Body> {
    let bodies = vec![
        Body::new(
            Vector3::new(190.0, 200.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            1_000.0,
        ),
        Body::new(
            Vector3::new(210.0, 200.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            1_000.0,
        ),
    ];
    bodies
}
