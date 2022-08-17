//use std::env;
use std::process;

use elevators::Config;

fn main() {
    let config = Config {
        n_elevators: 2,
        n_floors: 4,
    };

    if let Err(e) = elevators::run(config) {
        eprintln!("Critical error: {}", e);

        process::exit(1);
    }
}