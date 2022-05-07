use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);

    Ok(())
}

