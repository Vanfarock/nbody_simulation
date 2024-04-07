mod body;
mod physics;
mod simulation;
mod state;

use body::Body;
use nalgebra::Vector3;
use rand::prelude::*;
use simulation::Simulation;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let files = vec![
        // "random_0.txt",
        "random_1.txt",
        "random_2.txt",
        "random_3.txt",
        "random_4.txt",
        "random_5.txt",
        "random_6.txt",
        "random_7.txt",
        "random_8.txt",
        "random_9.txt",
        "random_10.txt",
        "random_11.txt",
        "random_12.txt",
        "random_13.txt",
        "random_14.txt",
        "random_15.txt",
        "random_16.txt",
        "random_17.txt",
        "random_18.txt",
        "random_19.txt",
        "random_20.txt",
        "random_21.txt",
        "random_22.txt",
        "random_23.txt",
        "random_24.txt",
        "random_25.txt",
        "random_26.txt",
        "random_27.txt",
        "random_28.txt",
        "random_29.txt",
        "random_30.txt",
    ];
    for file in files {
        // let bodies = generate_bodies()?;
        // let simulation = Simulation::new(10., 1_000, 6.67430e-11, Some(1e-5), bodies, "test.json");

        let mut rng = thread_rng();
        let min_pos = -1.;
        let max_pos = 1.;

        let min_vel = -1.;
        let max_vel = 1.;

        let min_mass = 0.;
        let max_mass = 1.;

        let bodies = vec![
            Body::new(
                Vector3::new(
                    rng.gen_range(min_pos..=max_pos),
                    rng.gen_range(min_pos..=max_pos),
                    rng.gen_range(min_pos..=max_pos),
                ),
                Vector3::new(
                    rng.gen_range(min_vel..=max_vel),
                    rng.gen_range(min_vel..=max_vel),
                    rng.gen_range(min_vel..=max_vel),
                ),
                rng.gen_range(min_mass..=max_mass),
            ),
            Body::new(
                Vector3::new(
                    rng.gen_range(min_pos..=max_pos),
                    rng.gen_range(min_pos..=max_pos),
                    rng.gen_range(min_pos..=max_pos),
                ),
                Vector3::new(
                    rng.gen_range(min_vel..=max_vel),
                    rng.gen_range(min_vel..=max_vel),
                    rng.gen_range(min_vel..=max_vel),
                ),
                rng.gen_range(min_mass..=max_mass),
            ),
            Body::new(
                Vector3::new(
                    rng.gen_range(min_pos..=max_pos),
                    rng.gen_range(min_pos..=max_pos),
                    rng.gen_range(min_pos..=max_pos),
                ),
                Vector3::new(
                    rng.gen_range(min_vel..=max_vel),
                    rng.gen_range(min_vel..=max_vel),
                    rng.gen_range(min_vel..=max_vel),
                ),
                rng.gen_range(min_mass..=max_mass),
            ),
        ];
        let simulation = Simulation::new(0.1, 300, 1., Some(1e-5), bodies, file);
        simulation.run(true)?;
    }

    Ok(())
}

fn generate_bodies() -> Result<Vec<Body>, Box<dyn Error>> {
    Ok(vec![
        Body::new(Vector3::new(2., 0.5, 0.), Vector3::new(-1., 0., 0.), 0.),
        Body::new(Vector3::new(-3., -2., 0.), Vector3::new(0., 1., 0.), 0.),
        Body::new(Vector3::new(1., 0., 0.), Vector3::new(0., -1., 0.), 0.),
    ])
    // let file = fs::read_to_string("c_0000.csv")?;
    // let mut bodies: Vec<Body> = vec![];
    // for (i, line) in file.lines().enumerate() {
    //     if i == 0 {
    //         continue;
    //     }

    //     let values: Vec<&str> = line.split(",").collect();

    //     bodies.push(Body::new(
    //         Vector3::new(
    //             values[0].parse::<f64>()?,
    //             values[1].parse::<f64>()?,
    //             values[2].parse::<f64>()?,
    //         ),
    //         Vector3::new(
    //             values[3].parse::<f64>()?,
    //             values[4].parse::<f64>()?,
    //             values[5].parse::<f64>()?,
    //         ),
    //         values[6].parse::<f64>()?,
    //     ))
    // }
    // Ok(bodies)
}
