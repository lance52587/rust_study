use io::Config;
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err|{
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = io::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
