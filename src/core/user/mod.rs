use std::fs;

use crate::core::error::RutesError;

const RUTES_HOME: &str = "/tmp/rutes/users";

#[derive(Debug)]
pub struct User {
    username: String,
}

impl User {
    pub fn new(username: String) -> Result<User, RutesError> {
        let userpaths = vec![
            format!("{}/{}", RUTES_HOME, username),
            format!("{}/{}/pipelines", RUTES_HOME, username),
        ];
        for path in userpaths{
            fs::create_dir_all(path).map_err(|_e| RutesError::UserCreationError)?;
        }
        Ok(Self { username })
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_userpath(&self) -> String {
        format!("{}/{}", RUTES_HOME, self.username)
    }

    pub fn get_pipelines_path(&self) -> String {
        format!("{}/{}/pipelines", RUTES_HOME, self.username)
    }
}
