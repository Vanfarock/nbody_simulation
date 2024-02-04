use nalgebra::Vector3;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::{error::Error, fmt, fs::File};

const GRAVITY_CONSTANT: f64 = 6.67430e-11;

#[derive(Serialize)]
pub struct Body {
    pos: Vector3<f64>,
    vel: Vector3<f64>,
    mass: f64,

    #[serde(skip_serializing)]
    force: Vector3<f64>,
}

impl Body {
    fn new(position: Vector3<f64>, velocity: Vector3<f64>, mass: f64) -> Self {
        Body {
            pos: position,
            vel: velocity,
            mass,
            force: Vector3::zeros(),
        }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body")
            .field("position", &self.pos)
            .field("velocity", &self.vel)
            .field("mass", &self.mass)
            .finish()
    }
}

fn calculate_gravity_force(body1: &Body, body2: &Body, softening_parameter: f64) -> Vector3<f64> {
    let delta_position = body2.pos - body1.pos;
    let distance_squared = delta_position.norm_squared() + softening_parameter.powi(2);
    let distance_cubed = distance_squared.sqrt() * distance_squared;

    let force_magnitude = GRAVITY_CONSTANT * (body1.mass * body2.mass) / distance_cubed;
    delta_position * force_magnitude
}

fn update_body_velocity_position(body: &mut Body, acceleration: Vector3<f64>, time_step: f64) {
    body.vel += acceleration * time_step;
    body.pos += body.vel * time_step;
}

fn file_exists(filename: &str) -> bool {
    if let Ok(metadata) = fs::metadata(filename) {
        return metadata.is_file();
    }
    return false;
}

fn open_file(filename: &str) -> Result<File, Box<dyn Error>> {
    if file_exists(filename) {
        fs::remove_file(filename)?;
    }

    File::create(filename)?;
    let file = OpenOptions::new().write(true).append(true).open(filename)?;

    return Ok(file);
}

fn save_to_json(mut file: &File, bodies: &Vec<Body>) -> Result<(), Box<dyn Error>> {
    let bodies_as_json = serde_json::to_string(&bodies)?;
    let json_data = format!("{}\n", bodies_as_json);

    let _ = file.write_all(json_data.as_bytes());

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let softening_parameter = 1e-5;
    let mut bodies = vec![
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

    let time_step = 1_000.;
    let num_steps = 10_000;

    let filename = "test.json";
    let file = open_file(filename)?;

    let _ = save_to_json(&file, &bodies);

    for _ in 0..num_steps {
        for i in 0..bodies.len() {
            for j in i + 1..bodies.len() {
                let force = calculate_gravity_force(&bodies[i], &bodies[j], softening_parameter);
                bodies[i].force += force;
                bodies[j].force -= force;
            }
            let acceleration_i = bodies[i].force * (1. / bodies[i].mass);
            update_body_velocity_position(&mut bodies[i], acceleration_i, time_step);
            bodies[i].force = Vector3::zeros();
        }
        let _ = save_to_json(&file, &bodies);
    }

    Ok(())
}
