use std::{env, fs, process, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing the arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);

    Ok(())
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
