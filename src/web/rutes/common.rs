use actix_web::HttpResponse;
use tera::Context;

pub fn render_template(
    template_name: &str,
    context: &Context,
    templates: actix_web::web::Data<tera::Tera>,
) -> HttpResponse {
    let template = templates.render(template_name, context).expect("Error");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template)
}

pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .insert_header((actix_web::http::header::LOCATION, location))
        .body("ok")
}
