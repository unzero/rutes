mod core;
mod web;
mod console;
mod start_rutes;

//use crate::console;

fn main() {
    // Console execution
    let ex_mode: &str = "rutes";
    match ex_mode {
        "console" => console::run(),
        "rutes" => start_rutes::run(),
        _ => println!("no operation")
    }
}


