use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = parse_config(&args);

    println!("Searching for {} in file {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("ohhh");
    println!("contents are {}", contents);
}

#[derive(Debug)]
struct Config<'a> {
    query: &'a String,
    filename: &'a String
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let filename = &args[2];
    
    Config {
        query,
        filename
    }
}
