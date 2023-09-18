use std::env;
use std::process;   

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching in file \"{}\" for \"{}\"",  config.file_path, config.query);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}