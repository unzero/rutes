/// This macro serialize a given structure
/// I use it to serialize data within  HttpResponse
#[macro_export]
macro_rules! context {
    ( $( $x:tt )+ ) => {
        &tera::Context::from_value(serde_json::json!( $($x)+ )).unwrap()
    }
}

/// This is the default location of Tera templates
pub fn get_templates_route() -> tera::Tera {
    tera::Tera::new("src/web/templates/**/*.html").unwrap()
}
