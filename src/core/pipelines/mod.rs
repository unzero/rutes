use std::fs;
use std::path::Path;

mod pipeline;

use super::error::RutesError;
use super::user::User;
use pipeline::Pipeline;

pub fn get_pipelines_metadata(user: &User) -> Result<Vec<Pipeline>, RutesError> {
    let entries = fs::read_dir(user.get_pipelines_path()).map_err(|_e| RutesError::ConfigurationError )?;
    let mut pipelines : Vec<Pipeline> = vec![];
    for entry in entries {
        let e = entry.map_err(|_e| RutesError::ConfigurationError)?.path();
        let path = e.as_path();
        pipelines.push(Pipeline::from(path));
    }
    Ok(pipelines)
}

pub fn create_pipeline(user: &User, name: &str, description: &str) -> Result<(), RutesError> {
    let p = Pipeline::new(name, description);
    p.store(user.get_pipelines_path())
}

pub fn query_pipeline(user: User, uuid: String) -> Result<Pipeline, RutesError> {
    let path = format!("{}/{}.json", user.get_pipelines_path(), uuid);
    let pipeline_path  = Path::new(path.as_str());
    Ok(Pipeline::from(pipeline_path))
}

pub fn drop_pipeline(user: User, uuid: String) -> Result<(), RutesError> {
    let path = format!("{}/{}.json", user.get_pipelines_path(), uuid);
    let pipeline_path  = Path::new(path.as_str());
    let pipeline = Pipeline::from(pipeline_path);
    pipeline.drop().map_err( |_e| RutesError::PipelineError )?;
    fs::remove_file(pipeline_path).map_err( |_e| RutesError::PipelineError )?;
    Ok(())
}
