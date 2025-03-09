mod core;
mod web;
mod console;

//use crate::console;

fn main() {
    // Console execution
    let ex_mode: &str = "web";
    match ex_mode {
        "console" => console::run(),
        _ => web::main().expect("hello"),
    }
}


