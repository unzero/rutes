use std::sync::{Arc, Mutex};

use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;
use env_logger::Env;
//use actix_identity::IdentityMiddleware;
//use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
//use actix_web::cookie::time::Duration;

mod utils;
mod rutes;

use crate::web::rutes::errors::not_found;
use crate::core::task::Task;

pub type RunningTask = Arc<Mutex< Vec<Task> >>;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    /* debug flag */
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init_from_env(Env::default().default_filter_or("trace"));
    //let private_key = actix_web::cookie::Key::generate();
    let running_task : RunningTask = Arc::new(Mutex::new(Vec::new()));
    let running_task_data = actix_web::web::Data::new(running_task);

    HttpServer::new( move || {
        let tera = crate::web::utils::get_templates_route();
        App::new()
            //login 
            .wrap(Logger::default())
            //session management 
            //.wrap(IdentityMiddleware::default())
            //.wrap(
            //    SessionMiddleware::builder( CookieSessionStore::default(), private_key.clone() )
            //    .cookie_name("red".to_owned())
            //    .cookie_secure(false)
            //    .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_LIFETIME))
            //    .build(),
            //)
            .app_data(actix_web::web::Data::new(tera))
            .app_data(running_task_data.clone())
            .configure(rutes::get_configuration)
            //.service(actix_files::Files::new("/static", "./static/").show_files_listing())
            .default_service( actix_web::web::route().to( not_found ) )
    })
    .bind(("192.168.174.128", 8080))?
    .run()
    .await
}
