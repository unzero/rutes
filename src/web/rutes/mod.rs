mod common;
pub mod errors;
mod task;
mod pipelines;
mod forms;

//within this file I put all logic for web application
use actix_web::web;

pub fn get_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(pipelines::get_pipelines))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("pipeline/new")
            .route(web::get().to(pipelines::new_pipeline))
            .route(web::post().to(pipelines::create_pipeline))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("pipeline/view/{uuid}")
            .route(web::get().to(pipelines::pipeline_front))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("pipeline/update")
            .route(web::post().to(pipelines::update_pipeline))
            //.default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("pipeline/drop/{uuid}")
            .route(web::post().to(pipelines::drop_pipeline))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("pipeline/update/{uuid}")
            .route(web::get().to(pipelines::configure_pipeline))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );




    /* task configuration */
    cfg.service(
        web::resource("task")
            .route(web::get().to(task::running))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("task/scheduler")
            .route(web::get().to(task::scheduler))
            .route(web::post().to(task::schedule_task))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("task/logs/{uuid}")
            .route(web::get().to(task::get_task_logs))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
    cfg.service(
        web::resource("task/ws")
            .route(web::get().to(task::ws))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );

    cfg.service(
        web::resource("task/live_logs")
            .route(web::get().to(task::live_logs))
            .default_service(actix_web::web::route().to(errors::not_found)),
    );
}
