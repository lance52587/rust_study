use std::env;
use std::fs;

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);

    // let query = &args[1];
    // let filename = &args[2];
    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text: \n{}", contents);
}
