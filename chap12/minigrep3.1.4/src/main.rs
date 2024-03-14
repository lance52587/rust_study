use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }

impl Config {
    // fn new(args: &[String]) -> Config {
    fn new(args: &[String]) -> Result<Config, &'static str>{

        // let query = args[1].clone();
        // let filename = args[2].clone();
        // Config { query, filename }

        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // Config { query, filename }
        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);

    // let query = &args[1];
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
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
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

use std::error::Error;
// fn run(config: Config){
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename)?;
    println!("With text: \n{}", contents);
    Ok(())
}