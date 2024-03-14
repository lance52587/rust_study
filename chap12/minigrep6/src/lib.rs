use std::fs;
use std::error::Error;
use std::env;

// struct Config {
//     query: String,
//     filename: String,
// }
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // Config { query, filename }
        // Ok(Config { query, filename })
        Ok(Config { query, filename, case_sensitive })
    }
}


// fn run(config: Config){
// fn run(config: Config) -> Result<(), Box<dyn Error>>{
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive{
        println!("case_sensitive");
        search(&config.query, &contents)
    }else{
        println!("case_insensitive");
        search_case_insensitive(&config.query, &contents)
    };

    // println!("With text: \n{}", contents);
    // for line in search(&config.query, &contents){
    for line in results{
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // vec![]
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // vec![]
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        // if line.contains(query){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    
    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}