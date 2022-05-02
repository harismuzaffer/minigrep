use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("Searching for {} in file {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("ohhh");
    println!("contents are {}", contents);
}

#[derive(Debug)]
struct Config<'a> {
    query: &'a String,
    filename: &'a String
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough parameters");
        }
        let query = &args[1];
        let filename = &args[2];

        Config {
            query,
            filename
        }
    }
}
