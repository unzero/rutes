use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PipelineForm {
    pub name : String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTask {
    pub command : String, 
    pub arguments : String,
}
