use uuid::Uuid;

#[derive(Debug)]
pub struct ExecutorRequest {
    pub cmd: String,
    pub user: String,
    pub pipeline_uuid: String,
}

impl ExecutorRequest {
    pub fn new(cmd: String, user: String, pipeline_uuid: String) -> Self {
        Self {
            cmd: cmd,
            user: user,
            pipeline_uuid: pipeline_uuid,
        }
    }
}
