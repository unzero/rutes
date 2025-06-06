use std::convert::TryFrom;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::error::RutesError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub script: String,
}

impl TryFrom<&Path> for Pipeline {
    type Error = RutesError;
    fn try_from(file: &Path) -> Result<Self, RutesError> {
        let serialized = fs::read_to_string(file).map_err(|_e| RutesError::IOError)?;
        Ok(serde_json::from_str(&serialized).map_err(|_e| RutesError::SerializationError)?)
    }
}

impl Pipeline {
    pub fn new(name: &str, description: &str) -> Self {
        Pipeline {
            uuid: Uuid::new_v4(),
            name: String::from(name),
            description: String::from(description),
            script: String::from(""),
        }
    }

    pub fn store(&self, userpath: String) -> Result<(), RutesError> {
        let path = format!("{}/{}.json", userpath, self.uuid);
        let contents = serde_json::to_string(&self).map_err(|_e| RutesError::SerializationError)?;
        fs::write(path, contents).map_err(|_e| RutesError::IOError)?;
        Ok(())
    }

    pub fn drop(self) -> Result<(), RutesError> {
        Ok(())
    }

    pub fn update(
        mut self,
        userpath: String,
        name: &str,
        description: &str,
        script: &str,
    ) -> Result<Pipeline, RutesError> {
        self.name = String::from(name);
        self.description = String::from(description);
        self.script = String::from(script);
        self.store(userpath).map_err(|e| e)?;
        Ok(self)
    }
}
