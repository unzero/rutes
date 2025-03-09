use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PipelineForm {
    pub name : String,
    pub description: String,
    /* This field is optional for creation */
    pub script: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTask {
    pub command : String, 
    pub arguments : String,
}
