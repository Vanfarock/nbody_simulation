use std::io::Write;
use std::{
    fs::{self, File, OpenOptions},
    io::Error,
};

use nalgebra::Vector3;

use crate::body::Body;

pub struct State {
    bodies_file: File,
    forces_file: File,
}

impl State {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let forces_filename = String::from("forces_") + filename;

        if State::file_exists(filename) {
            fs::remove_file(filename)?;
        }
        if State::file_exists(&forces_filename) {
            fs::remove_file(&forces_filename)?;
        }

        File::create(filename)?;
        File::create(&forces_filename)?;

        let state = State {
            bodies_file: OpenOptions::new().write(true).append(true).open(filename)?,
            forces_file: OpenOptions::new()
                .write(true)
                .append(true)
                .open(&forces_filename)?,
        };
        Ok(state)
    }

    fn file_exists(filename: &str) -> bool {
        if let Ok(metadata) = fs::metadata(filename) {
            return metadata.is_file();
        }
        false
    }

    pub fn save_bodies(&mut self, bodies: &Vec<Body>) -> Result<(), serde_json::Error> {
        let bodies_as_json = serde_json::to_string(&bodies)?;
        let json_data = format!("{}\n", bodies_as_json);

        let _ = self.bodies_file.write_all(json_data.as_bytes());

        Ok(())
    }

    pub fn save_forces(&mut self, forces: &Vec<Vector3<f64>>) -> Result<(), serde_json::Error> {
        let forces_as_json = serde_json::to_string(&forces)?;
        let json_data = format!("{}\n", forces_as_json);

        let _ = self.forces_file.write_all(json_data.as_bytes());

        Ok(())
    }
}
