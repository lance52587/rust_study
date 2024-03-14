use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);

    // let query = &args[1];
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err);
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    // println!("With text: \n{}", contents);
    // run(config);
    // if let Err(e) = run(config) {
    if let Err(e) = minigrep::run(config){
        // println!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
