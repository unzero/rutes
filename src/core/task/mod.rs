use std::fs::File;
use std::process::Child;
use std::process::Command;
use std::collections::HashMap;

use uuid::Uuid;

use crate::core::error::RutesError;
use crate::core::user::User;

pub struct Task {
    uuid: Uuid,
    process_child: Child,
    log_filepath: String,
    command: String,
}

impl Task {
    fn new(uuid: Uuid, process_child: Child, log_filepath: String, command: String) -> Task {
        Self {
            uuid,
            process_child,
            log_filepath,
            command,
        }
    }

    fn get_status(&mut self) -> String {
        let exit_status = self.process_child.try_wait();
        match exit_status {
            Ok(Some(st)) => format!("Finished with status: {}", st),
            Ok(_) => String::from("Running"),
            _ => String::from("Unknow status"),
        }
    }

    pub fn to_string(&mut self) -> String {
        let status = self.get_status();
        format!(
            "Process {} {}:{} on {}\n{}",
            self.uuid,
            self.command,
            self.process_child.id(),
            self.log_filepath,
            status
        )
    }

    pub fn kill(&mut self) -> Result<(), RutesError> {
        self.process_child
            .kill()
            .map_err(|_e| RutesError::TaskError)
    }

    pub fn view_tail(&mut self) -> Result<(), RutesError> {
        let mut cmd = Command::new("tail");
        cmd.args(vec!["-n", "10", self.log_filepath.as_str()]);
        let _ = cmd.status().map_err( |_e| {
            RutesError::ExecutorError
        });
        Ok(())
    }

    pub fn get_tail(&mut self) -> Result<String, RutesError> {
        let mut cmd = Command::new("tail");
        cmd.args(vec!["-n", "10", self.log_filepath.as_str()]);
        let output = cmd.output().map_err(|_e| RutesError::TaskError )?;
        let logs = String::from_utf8(output.stdout).map_err(|_e| RutesError::TaskError )?;
        Ok(logs)
    }

    pub fn get_uuid(&self) -> Result<Uuid, RutesError> {
        Ok(self.uuid)
    }

    pub fn to_hash(&mut self) -> Result<HashMap<String, String>, RutesError> {
        Ok(
            HashMap::from([ 
                (String::from("uuid"), format!{"{}", self.uuid}),
                (String::from("command"), self.command.clone()),
                (String::from("log_filepath"), self.log_filepath.clone()),
                (String::from("status"), self.get_status()),
            ])
        )
    }

}

pub fn execute(active_user: &User, command: &str, args: Vec<&str>) -> Result<Task, RutesError> {
    println!("Executing {} for {}", command, active_user.get_username());
    let uuid = Uuid::new_v4();
    let filepath = format!("{}/{}.log", active_user.get_userpath(), uuid,);
    let file = File::create(filepath.clone()).map_err(|_e| {
        println!("Error opening file");
        RutesError::ExecutorError
    })?;

    let mut cmd = Command::new(command);
    cmd.args(args);
    cmd.stdout(file);
    let child = cmd.spawn().map_err(|_e| RutesError::ExecutorError)?;
    Ok(Task::new(uuid, child, filepath, String::from(command)))
}
