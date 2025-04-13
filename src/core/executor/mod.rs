use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::core::task::execute_for_pipeline;
use messages::ExecutorRequest;

use super::task::Task;

pub mod messages;

pub struct Executor {
    receiver: Arc<Mutex<Receiver<ExecutorRequest>>>,
    rutes_state: Arc<AtomicBool>,
    running_task: Arc<Mutex<Vec<Task>>>,
}

impl Executor {
    pub fn new(receiver: Receiver<ExecutorRequest>, rutes_state: Arc<AtomicBool>) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(receiver)),
            rutes_state: rutes_state,
            running_task: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&mut self) {
        let state_arc = self.rutes_state.clone();
        let receiver_arc = self.receiver.clone();
        let running_task_arc = self.running_task.clone();

        let _ = thread::spawn(move || {
            log::info!("Starting scheduler runner");
            while state_arc.load(Ordering::SeqCst) {
                let recv = receiver_arc
                    .lock()
                    .expect("Error getting receiver reference")
                    .recv();
                match recv {
                    Ok(msg) => {
                        log::debug!("New run for pipeline {:?}", msg.pipeline_uuid.clone());
                        let task =
                            execute_for_pipeline(&msg.user, msg.cmd.as_str(), msg.pipeline_uuid);
                        match task {
                            Ok(task) => {
                                running_task_arc.lock().expect("Ee").push(task);
                            }
                            Err(e) => {
                                log::error!("Error ")
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Scheduler receiver error: {:?}", e)
                    }
                }
            }
        })
        .join()
        .map_err(|e| {
            log::error!("Error on joining scheduler runner: {:?}", e);
            self.close();
        });
    }

    pub fn close(&mut self) {
        log::info!("Cleaning scheduler runner");
        self.rutes_state.store(false, Ordering::SeqCst)
    }
}
