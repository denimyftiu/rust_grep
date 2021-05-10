use std::fs;
use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a filename."),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{ query, filename, case_insensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for (i, line) in results {
        println!("{}: {}", i, line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str)
    -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            line.to_lowercase().contains(&query.to_lowercase())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust 
safe, fast, productive.
Pic three.";
        assert_eq!(vec![(1, "safe, fast, productive.")], search(query, contents));
        let query = "fast";
        assert_eq!(vec![(1, "safe, fast, productive.")], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust
safe, fast, productive.
Pic three.";
        assert_eq!(vec![(0, "Rust")], search_case_insensitive(query, contents));
        let query = "FaSt";
        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search_case_insensitive(query, contents)
        );
    }
}
