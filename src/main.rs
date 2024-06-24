mod body;
mod physics;
mod simulation;
mod state;

use body::Body;
use nalgebra::Vector3;
use simulation::Simulation;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let files = vec![
        "lagrange.txt",
    ];
    for file in files {
        let bodies = generate_bodies()?;
        let simulation = Simulation::new(0.1, 1_000, 1., Some(1e-5), bodies, file);
        simulation.run(true)?;
    }

    Ok(())
}

fn generate_bodies() -> Result<Vec<Body>, Box<dyn Error>> {
    Ok(vec![
        Body::new(
            Vector3::new(1., 0., 0.),
            Vector3::new(0., f64::sqrt(3.), 0.),
            1.,
        ),
        Body::new(
            Vector3::new(-1., 0., 0.),
            Vector3::new(0., -f64::sqrt(3.)/2., 0.),
            1.,
        ),
        Body::new(
            Vector3::new(0., 2., 0.),
            Vector3::new(-f64::sqrt(3.), 0., 0.),
            1.,
        ),
    ])
}
