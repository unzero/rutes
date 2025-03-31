use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::errors::RutesHttpError;

#[derive(Debug, Deserialize, Serialize)]
pub struct PipelineForm {
    pub name: String,
    pub description: String,
    /* This field is optional for creation */
    pub script: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTask {
    pub command: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PipelineCode {
    pub script: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSchedule {
    pub uuid: String,
    pub parameters: HashMap<String, String>,
}

impl PipelineForm {
    pub fn get_uuid(&self) -> Result<String, RutesHttpError> {
        match &self.uuid {
            Some(s) => Ok(s.to_string()),
            _ => Err(RutesHttpError::Default),
        }
    }

    pub fn get_script(&self) -> Result<String, RutesHttpError> {
        match &self.script {
            Some(s) => Ok(s.to_string()),
            _ => Err(RutesHttpError::Default),
        }
    }
}
