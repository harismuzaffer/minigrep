use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let result = Config::new(&args);

    match result {
        Ok(c) => {
            println!("we have made it {}", c.query);
        },
        Err(message) => {
            println!("sorry found an error {}", message);
        }
    }

    // println!("Searching for {} in file {}", config.query, config.filename);

    // let contents = fs::read_to_string(config.filename).expect("ohhh");
    // println!("contents are {}", contents);
}

#[derive(Debug)]
struct Config<'a> {
    query: &'a String,
    filename: &'a String
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough params");
        }
        let query = &args[1];
        let filename = &args[2];

        let c  = Config {
            query,
            filename
        };

        Ok(c)
    }
}
