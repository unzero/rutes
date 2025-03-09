use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPipeline {
    pub name : String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTask {
    pub command : String, 
    pub arguments : String,
}
