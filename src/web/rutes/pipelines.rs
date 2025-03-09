use actix_web::{HttpResponse, Result};
use actix_web::web;

use super::errors::RutesHttpError;
use super::forms::PipelineForm;
use crate::core;
use crate::core::user::User;
use crate::web::rutes::common;

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
    templates: actix_web::web::Data<tera::Tera>,
) -> Result<HttpResponse, RutesHttpError> {
    let user = User::new(String::from("tsukiko")).map_err(|_e| RutesHttpError::Default)?;
    let _ = core::pipelines::create_pipeline(&user, form.name.clone());
    get_pipelines(templates).await
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
    let pipe = core::pipelines::query_pipeline(user, uuid.clone()).map_err(|_e| RutesHttpError::Default)?;
    Ok(common::render_template(
        "pipelines/pipeline.html",
        crate::context!({"uuid": uuid, "name": pipe.name}),
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
    let pipe = core::pipelines::query_pipeline(user, uuid.clone()).map_err(|_e| RutesHttpError::Default)?;
    Ok(common::render_template(
        "pipelines/configure.html",
        crate::context!({"uuid": uuid, "name": pipe.name}),
        templates,
    ))
}
