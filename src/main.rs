mod body;
mod physics;
mod simulation;
mod state;

use body::Body;
use nalgebra::Vector3;
use rand::Rng;
use simulation::Simulation;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let files = vec![
        "exemplo.txt",
    ];
    for file in files {
        let bodies = generate_random_bodies()?;
        let simulation = Simulation::new(0.1, 1_000, 1., Some(1e-5), bodies, file);
        simulation.run(true)?;
    }

    Ok(())
}

fn generate_random_bodies() -> Result<Vec<Body>, Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    Ok(vec![
        Body::new(
            Vector3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.)),
            Vector3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.)),
            rng.gen_range(0.0..1.),
        ),
        Body::new(
            Vector3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.)),
            Vector3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.)),
            rng.gen_range(0.0..1.),
        ),
        Body::new(
            Vector3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.)),
            Vector3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.)),
            rng.gen_range(0.0..1.),
        ),
    ])
}

fn generate_bodies() -> Result<Vec<Body>, Box<dyn Error>> {
    Ok(vec![
        Body::new(
            Vector3::new(-0.97000436, 0.24308753, 0.),
            Vector3::new(0.4662036850, 0.4323657300, 0.),
            1.,
        ),
        Body::new(
            Vector3::new(0., 0., 0.),
            Vector3::new(-0.93240737, -0.86473146, 0.),
            1.,
        ),
        Body::new(
            Vector3::new(0.97000436, -0.24308753, 0.),
            Vector3::new(0.4662036850, 0.4323657300, 0.),
            1.,
        ),
    ])
}

