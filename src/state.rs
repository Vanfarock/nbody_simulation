use std::io::Write;
use std::{
    fs::{self, File, OpenOptions},
    io::Error,
};

use crate::body::Body;

pub struct State {
    file: File,
}

impl State {
    pub fn new(filename: &str) -> Result<Self, Error> {
        if State::file_exists(filename) {
            fs::remove_file(filename)?;
        }

        File::create(filename)?;

        let state = State {
            file: OpenOptions::new().write(true).append(true).open(filename)?,
        };
        Ok(state)
    }

    fn file_exists(filename: &str) -> bool {
        if let Ok(metadata) = fs::metadata(filename) {
            return metadata.is_file();
        }
        false
    }

    pub fn save(&mut self, bodies: &Vec<Body>) -> Result<(), serde_json::Error> {
        let bodies_as_json = serde_json::to_string(&bodies)?;
        let json_data = format!("{}\n", bodies_as_json);

        let _ = self.file.write_all(json_data.as_bytes());

        Ok(())
    }
}
