#[macro_export]
macro_rules! context {
    ( $( $x:tt )+ ) => {
        &tera::Context::from_value(serde_json::json!( $($x)+ )).unwrap()
    }
}

#[macro_export]
macro_rules! json_response {
    ( $( $x:tt )+ ) => {
        serde_json::json!( $($x)+ )
    }
}

pub fn get_templates_route() -> tera::Tera {
    tera::Tera::new("src/web/templates/**/*.html").unwrap()
}
