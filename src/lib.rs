use std::error::Error;
use std::fs;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_constructor_returns_error() {
        let empty: Vec<String> = vec!();

        assert!(Config::new(&empty).is_err());
    }

    #[test]
    fn can_construct_config() {
        let args =  &[String::from("some/path/minigrep"), String::from("query"), String::from("filename")];
        let expected = Ok(Config { query: String::from("query"), filename: String::from("filename") });
        let actual = Config::new(args);

        assert_eq!(expected, actual);
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}