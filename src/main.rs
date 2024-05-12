mod error;
mod model;
mod prelude;
mod rest;
mod security;
mod todo;
mod user;
mod utils;

use std::env;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    simple_logger::SimpleLogger::new().env().init().unwrap();

    rest::start_server()
}
