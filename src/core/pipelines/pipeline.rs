use std::convert::From;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::error::RutesError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
    uuid: Uuid,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineMetadata {
    pub uuid: Uuid,
    pub name: String,
}

impl From<&Pipeline> for PipelineMetadata {
    fn from(pipeline: &Pipeline) -> Self {
        PipelineMetadata {
            uuid: pipeline.uuid,
            name: pipeline.name.clone(),
        }
    }
}

impl From<&Path> for Pipeline {
    fn from(file: &Path) -> Self {
        let serialized = fs::read_to_string(file).unwrap();
        serde_json::from_str(&serialized).unwrap()
    }
}

impl Pipeline {
    pub fn new(name: String) -> Self {
        Pipeline {
            uuid: Uuid::new_v4(),
            name: name,
        }
    }

    pub fn store(self, userpath: String) -> Result<(), RutesError> {
        let path = format!("{}/{}.json", userpath, self.uuid);
        let contents = serde_json::to_string(&self).map_err(|_e| RutesError::PipelineError)?;
        fs::write(path, contents).map_err(|_e| RutesError::PipelineError)?;
        Ok(())
    }

    pub fn drop(self) -> Result<(), RutesError> {
        Ok(())
    }
}
