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

    for l in search(&config.query, &contents) {
        println!("{}", l);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            v.push(line);
        }
    }

    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "rom";
        let contents = "\
writing in Rust:
working from home
and enjoying the coffee.
";

        assert_eq!(vec!["working from home"], search(query, contents));
    }
}
