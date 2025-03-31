use std::fmt;

use actix_web::error::ResponseError;
use actix_web::HttpResponse;

use crate::web::rutes::common::render_template;

#[derive(Debug)]
pub enum RutesHttpError {
    Default,
    WsError,
}

impl fmt::Display for RutesHttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RutesHttpError::WsError => write!(f, "Something gone wrong with the websocket!"),
            _ => write!(f, "Something gone wrong!"),
        }
    }
}

impl ResponseError for RutesHttpError {
    fn error_response(&self) -> HttpResponse {
        let templates = actix_web::web::Data::new(crate::web::utils::get_templates_route());
        match self {
            RutesHttpError::WsError => {
                render_template("errors/404.html", crate::context!({}), templates)
            }
            _ => HttpResponse::InternalServerError().body("Something gone wrong, try again!"),
        }
    }
}

pub async fn not_found() -> HttpResponse {
    let templates = actix_web::web::Data::new(crate::web::utils::get_templates_route());
    let template = templates
        .render("errors/404.html", crate::context!({}))
        .expect("Error");
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(template)
}
