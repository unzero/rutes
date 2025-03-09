use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::web;
use futures_util::StreamExt as _;

use crate::core::task::execute;
use crate::core::user::User;
use super::errors::RutesHttpError;
use super::forms::NewTask;
use super::common;

pub async fn scheduler(templates: actix_web::web::Data<tera::Tera>) -> HttpResponse {
    common::render_template("task/scheduler.html", crate::context!({}), templates)
}

pub async fn schedule_task(
    form: actix_web::web::Form<NewTask>,
    running_tasks: actix_web::web::Data<crate::web::RunningTask>
) -> Result<HttpResponse, RutesHttpError> {
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default )?;
    let vec_args = form.arguments.split(" ").collect();
    let child = execute(&user, form.command.as_str(), vec_args).map_err( |_e| RutesHttpError::Default)?;
    running_tasks.lock().map_err(|_e| RutesHttpError::Default)?.push(child);
    Ok(common::redirect("/task"))
}

pub async fn running(
    templates: actix_web::web::Data<tera::Tera>,
    running_tasks: actix_web::web::Data<crate::web::RunningTask>,
) -> Result<HttpResponse, RutesHttpError> {
    
    let mut tasks = vec![];
    for task in running_tasks.lock().unwrap().iter_mut() {
        let hash_task = task.to_hash().map_err(|_e| RutesHttpError::Default )?;
        tasks.push(hash_task);
    }

    Ok(
        common::render_template( 
            "task/task.html",
            crate::context!({
                "metainformation": ["Id", "Log path", "Process", "Status"], 
                "running_tasks": tasks,
            }),
            templates,
        )
    )
}

pub async fn get_task_logs(
    path: web::Path<usize>,
    templates: actix_web::web::Data<tera::Tera>,
    running_tasks: actix_web::web::Data<crate::web::RunningTask>,
) -> Result<HttpResponse, RutesHttpError> {
    let index = path.into_inner();
    let tail : String = running_tasks.lock()
        .map_err(|_e| RutesHttpError::Default )?
        [index].get_tail().map_err(|_e| RutesHttpError::Default)?;
    Ok(common::render_template("task/logs.html", crate::context!({"initial_logs": tail}), templates))
}


use std::time::{Duration, Instant};
use tokio::time::interval;
use futures_util::future::{select, Either};
use std::pin::pin;
use actix_ws::AggregatedMessage;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn ws(
    req: HttpRequest, 
    body: web::Payload
) -> Result<HttpResponse, RutesHttpError> {
    let (response, mut session, stream) = actix_ws::handle(&req, body).map_err(|_e| {
        println!("Error on websocket {}", _e);
        RutesHttpError::WsError
    })?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    actix_web::rt::spawn(async move {
        //web socket logic 
        let mut last_heartbeat = Instant::now();
        let mut interval = interval(HEARTBEAT_INTERVAL);

        let close_reason = loop {

            let tick = pin!(interval.tick());
            let message = pin!(stream.next());

            match select(tick, message).await {
                Either::Left((_inst, _)) => {
                    // if no heartbeat ping/pong received recently, close the connection
                    if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                        print!("client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting");
                        break None;
                    }
    
                    // send heartbeat ping
                    let _ = session.ping(b"Hello, are you there?").await;
                },

                Either::Right((Some(Ok(msg)), _)) => {
                    println!("msg: {msg:?}");
    
                    match msg {
                        AggregatedMessage::Ping(bytes) => {
                            println!("The client is there");    
                            last_heartbeat = Instant::now();
                            // unwrap:
                            session.pong(&bytes).await.unwrap();
                        }
    
                        AggregatedMessage::Pong(_) => {
                            println!("The client is there");  
                            last_heartbeat = Instant::now();
                        }
                        AggregatedMessage::Close(reason) => break reason,
                        _ => println!("unknown message from client"),
                    }
                },

                Either::Right((_, _)) => break None,

            }

        };

        let _ = session.close(close_reason).await;

    });

    Ok(response)
}

pub async fn live_logs(templates: actix_web::web::Data<tera::Tera>) -> HttpResponse {
    common::render_template("task/live_logs.html", crate::context!({}), templates)
}

