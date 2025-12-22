use minigrep_tuhin::grep::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("Arguments: {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(0);
    });
    println!(
        "Searching for '{}' in file '{}':\n-----------------------------------------------------",
        config.query, config.filename
    );

    if let Err(err) = run(&config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
