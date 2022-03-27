use std::{env, process};

use grep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // we don't need to use the match keyword, instead we have the 'if let' syntax.
    if let Err(e) = run(config) {
        eprintln!("Application Error : {}", e);
        process::exit(1)
    }
}
