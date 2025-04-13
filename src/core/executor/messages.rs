use crate::core::user::User;

#[derive(Debug)]
pub struct ExecutorRequest {
    pub cmd: String,
    pub user: User,
    pub pipeline_uuid: String,
}

impl ExecutorRequest {
    pub fn new(cmd: String, user: User, pipeline_uuid: String) -> Self {
        Self {
            cmd: cmd,
            user: user,
            pipeline_uuid: pipeline_uuid,
        }
    }
}
