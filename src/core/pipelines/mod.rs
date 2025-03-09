use std::fs;
use std::path::Path;

use uuid::Uuid;

mod pipeline;

use super::error::RutesError;
use super::user::User;
use pipeline::Pipeline;

pub fn get_pipelines_metadata(user: &User) -> Result<Vec<Pipeline>, RutesError> {
    let entries =
        fs::read_dir(user.get_pipelines_path()).map_err(|_e| RutesError::ConfigurationError)?;
    let mut pipelines: Vec<Pipeline> = vec![];
    for entry in entries {
        /* TODO: you may have a folder here */
        let e = entry.map_err(|_e| RutesError::ConfigurationError)?.path();
        let path = e.as_path();
        pipelines.push(Pipeline::try_from(path).map_err(|e| e)?);
    }
    Ok(pipelines)
}

pub fn create_pipeline(user: &User, name: &str, description: &str) -> Result<(), RutesError> {
    let p = Pipeline::new(name, description);
    p.store(user.get_pipelines_path())
}

pub fn query_pipeline(user: User, uuid: String) -> Result<Pipeline, RutesError> {
    let path = format!("{}/{}.json", user.get_pipelines_path(), uuid);
    let pipeline_path = Path::new(path.as_str());
    Pipeline::try_from(pipeline_path)
}

pub fn drop_pipeline(user: User, uuid: String) -> Result<(), RutesError> {
    let path = format!("{}/{}.json", user.get_pipelines_path(), uuid);
    let pipeline_path = Path::new(path.as_str());
    let pipeline = Pipeline::try_from(pipeline_path).map_err(|e| e)?;
    pipeline.drop().map_err(|e| e)?;
    fs::remove_file(pipeline_path).map_err(|_e| RutesError::IOError)?;
    Ok(())
}

pub fn update_pipeline(
    user: User,
    uuid: &str,
    name: &str,
    description: &str,
    script: &str,
) -> Result<Uuid, RutesError> {
    let userpath = user.get_pipelines_path();
    let pipeline = query_pipeline(user, String::from(uuid))?;
    let pipeline = pipeline
        .update(userpath, name, description, script)
        .map_err(|e| e)?;
    Ok(pipeline.uuid)
}
