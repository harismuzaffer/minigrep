use std::{fs, env};
use std::error::Error;

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough params");
        }
        let query = &args[1];
        let filename = &args[2];

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        let c  = Config {
            query,
            filename,
            case_sensitive,
        };

        Ok(c)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for l in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            v.push(line);
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "rom";
        let contents = "\
writing in Rust Rom:
working from home
and enjoying the coffee.
";

        assert_eq!(vec!["working from home"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rom";
        let contents = "\
writing in Rust Rom:
working from home
and enjoying the coffee.
";

        assert_eq!(vec!["writing in Rust Rom:", "working from home"], search_case_insensitive(query, contents));
    }
}
