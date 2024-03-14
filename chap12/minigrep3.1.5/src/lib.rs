use std::fs;
use std::error::Error;

// struct Config {
//     query: String,
//     filename: String,
// }
pub struct Config {
    pub query: String,
    pub filename: String,
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
    // fn new(args: &[String]) -> Result<Config, &'static str>{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{

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


// fn run(config: Config){
// fn run(config: Config) -> Result<(), Box<dyn Error>>{
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename)?;
    println!("With text: \n{}", contents);
    Ok(())
}