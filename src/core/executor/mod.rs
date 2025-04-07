use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use messages::ExecutorRequest;

pub mod messages;

pub struct Executor {
    receiver: Arc<Mutex<Receiver<ExecutorRequest>>>,
    rutes_state: Arc<AtomicBool>,
}

impl Executor {
    pub fn new(receiver: Receiver<ExecutorRequest>, rutes_state: Arc<AtomicBool>) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(receiver)),
            rutes_state: rutes_state,
        }
    }

    pub fn run(&mut self) {
        let state_arc = self.rutes_state.clone();
        let receiver_arc = self.receiver.clone();

        let _ = thread::spawn(move || {
            log::info!("Starting scheduler runner");
            while state_arc.load(Ordering::SeqCst) {
                let recv = receiver_arc
                    .lock()
                    .expect("Error getting receiver reference")
                    .recv();
                match recv {
                    Ok(msg) => {
                        println!("new message from ui {:?}", msg);
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
