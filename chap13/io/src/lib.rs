use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    // pub fn new(args: &[String]) -> Result<Config, &'static str>{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        // if args.len() < 3{
        //     return Err("not enough arguments");
        // }

        // let query = args[1].clone();
        // let filename = args[2].clone();
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }
}

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
    // let mut results = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // vec![]
    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines(){
    //     // if line.contains(query){
    //     if line.to_lowercase().contains(&query){
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}
