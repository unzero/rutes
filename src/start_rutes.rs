use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use crate::core::executor;
use crate::core::executor::messages::ExecutorRequest;
use crate::web;

pub fn run() {
    let rutes_state = Arc::new(AtomicBool::new(true));
    let (web_sender, executor_receiver) = mpsc::channel::<ExecutorRequest>();
    let mut scheduler = executor::Executor::new(executor_receiver, rutes_state.clone());

    let t1 = thread::spawn(move || {
        log::info!("Starting web server runner");
        web::main(web_sender).expect("hello");
        log::info!("Closing web server runner");
        rutes_state.store(false, Ordering::SeqCst)
    });

    scheduler.run();

    let _ = t1
        .join()
        .map_err(|e| log::error!("Error joining web server thread: {:?}", e));
    scheduler.close();
}
