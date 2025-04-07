use actix_web::web;
use actix_web::{HttpResponse, Result};

use super::errors::RutesHttpError;
use super::forms::NewSchedule;
use super::forms::PipelineCode;
use super::forms::PipelineForm;
use crate::core;
use crate::core::executor::messages::ExecutorRequest;
use crate::core::user::User;
use crate::web::rutes::common;
use crate::web::ExecutorSender;

pub async fn get_pipelines(
    templates: actix_web::web::Data<tera::Tera>,
) -> Result<HttpResponse, RutesHttpError> {
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default)?;
    let pipelines =
        core::pipelines::get_pipelines_metadata(&user).map_err(|_e| RutesHttpError::Default)?;
    Ok(common::render_template(
        "pipelines/pipelines.html",
        crate::context!({"pipelines": pipelines}),
        templates,
    ))
}

pub async fn create_pipeline(
    form: actix_web::web::Form<PipelineForm>,
) -> Result<HttpResponse, RutesHttpError> {
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default)?;
    let _ = core::pipelines::create_pipeline(&user, form.name.as_str(), form.description.as_str());
    Ok(common::redirect("/"))
}

pub async fn new_pipeline(
    templates: actix_web::web::Data<tera::Tera>,
) -> Result<HttpResponse, RutesHttpError> {
    Ok(common::render_template(
        "pipelines/new_pipeline.html",
        crate::context!({}),
        templates,
    ))
}

pub async fn pipeline_front(
    path: web::Path<String>,
    templates: actix_web::web::Data<tera::Tera>,
) -> Result<HttpResponse, RutesHttpError> {
    let uuid = path.into_inner();
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default)?;
    let pipeline = core::pipelines::query_pipeline(user, uuid.clone())
        .map_err(|_e| RutesHttpError::Default)?;
    Ok(common::render_template(
        "pipelines/pipeline.html",
        crate::context!({"uuid": pipeline.uuid, "name": pipeline.name, "description": pipeline.description}),
        templates,
    ))
}

pub async fn drop_pipeline(
    path: web::Path<String>,
    templates: actix_web::web::Data<tera::Tera>,
) -> Result<HttpResponse, RutesHttpError> {
    let uuid = path.into_inner();
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default)?;
    core::pipelines::drop_pipeline(user, uuid).map_err(|_e| RutesHttpError::Default)?;
    get_pipelines(templates).await
}

pub async fn configure_pipeline(
    path: web::Path<String>,
    templates: actix_web::web::Data<tera::Tera>,
) -> Result<HttpResponse, RutesHttpError> {
    let uuid = path.into_inner();
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default)?;
    let pipeline = core::pipelines::query_pipeline(user, uuid.clone())
        .map_err(|_e| RutesHttpError::Default)?;
    Ok(common::render_template(
        "pipelines/update.html",
        crate::context!({"pipeline": pipeline}),
        templates,
    ))
}

pub async fn update_pipeline(
    form: actix_web::web::Form<PipelineForm>,
) -> Result<HttpResponse, RutesHttpError> {
    let user = User::new(String::from("tsukiko")).unwrap();
    let uuid = core::pipelines::update_pipeline(
        user,
        form.get_uuid()?.as_str(),
        form.name.as_str(),
        form.description.as_str(),
        form.get_script()?.as_str(),
    )
    .unwrap();
    Ok(common::redirect(
        format!("/pipeline/view/{}", uuid).as_str(),
    ))
}

pub async fn check_pipeline_code(
    form: actix_web::web::Json<PipelineCode>,
) -> Result<HttpResponse, RutesHttpError> {
    println!("checking syntax");
    crate::core::pipelines::parser::check_syntax(form.script.as_str())
        .map_err(|_e| RutesHttpError::Default)?;
    println!("checking syntax done");
    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn execute(
    path: web::Path<String>,
    templates: actix_web::web::Data<tera::Tera>,
) -> Result<HttpResponse, RutesHttpError> {
    let uuid = path.into_inner();
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default)?;
    let pipeline = core::pipelines::query_pipeline(user, uuid.clone())
        .map_err(|_e| RutesHttpError::Default)?;
    let parameters = core::pipelines::parser::get_script_parameters(pipeline.script.as_str())
        .map_err(|_e| RutesHttpError::Default)?;
    Ok(common::render_template(
        "pipelines/execute.html",
        crate::context!({"uuid": pipeline.uuid, "name": pipeline.name, "description": pipeline.description, "parameters": parameters }),
        templates,
    ))
}

pub async fn schedule(
    executor_sender: actix_web::web::Data<ExecutorSender>,
    form: actix_web::web::Json<NewSchedule>,
) -> Result<HttpResponse, RutesHttpError> {
    let executor_request =
        ExecutorRequest::new(String::from("ping 8.8.8.8"), String::from("tsukiko"), form.uuid.clone());

    executor_sender
        .lock()
        .map_err(|_e| RutesHttpError::Default)?
        .send(executor_request)
        .map_err(|_e| RutesHttpError::Default)?;

    Ok(HttpResponse::Ok().body(""))
}
