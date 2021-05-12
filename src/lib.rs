//! ## rust_grep
//! rsut_grep contains implementation functions for file searches.
use std::fs;
use std::env;
use std::error::Error;

/// Config is the base structure for retrieving input from the user.
#[derive(Debug)]
pub struct Config {
    /// The string you want to search/query for in a given file.
    pub query: String,
    /// The filepath you want to run the query against.
    pub filename: String,
    /// Taken from the environment variable `CASE_INSENSITIVE`.
    /// If this environment variable exists the search is case insensitive.
    pub case_insensitive: bool,
}

impl Config {
    /// Creating a new config from standard input.
    /// # Example
    /// ```
    /// use std::env;
    /// use std::process;
    /// use rust_grep::Config;
    ///
    /// let config = Config::new(env::args()).unwrap_or_else(|error| {
    ///     eprintln!("Problem parsing arguments: {}", error);
    ///     process::exit(1);
    /// });
    /// ```
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // Skip filename as it its the first item in the `Args` vector
        // iterator.
        args.next();

        // The query we want to search for.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string."),
        };

        // The filename we want to query.
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a filename."),
        };

        // See if case sensitivity is enabled.
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

/// The entrypoint that executes the search based on the `Config` recieved.
/// If `CASE_INSENSITIVE` environment variable is defined run executes
/// `search_case_insensitive`. Otherwise it executes `search`.
///
/// # Example
/// ```
/// use std::env;
/// use std::process;
///
/// use rust_grep::Config;
///
/// fn main() {
///
///     let config = Config::new(env::args()).unwrap_or_else(|error| {
///         eprintln!("Problem parsing arguments: {}", error);
///         process::exit(1);
///     });
///
///     if let Err(err) = rust_grep::run(config) {
///         eprintln!("Application error: {}", err);
///         process::exit(1);
///     };
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file to string.
    let contents = fs::read_to_string(config.filename)?;

    // Get the results of the search based in case sensitivity.
    let results = if config.case_insensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Print the results asociated by the line location in the file.
    for (i, line) in results {
        println!("{}: {}", i, line);
    }

    Ok(())
}

/// Search for `query` in the `contents` of the file recieved by `Config`.
/// Returns a Vector containing tuples of usize and &str which represent the
/// line number and line where the query matched.
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        // Enumerate to get line numbers.
        .enumerate()
        // Filter to later collect line and number into a Vec.
        .filter(|(_, line)| line.contains(query))
        .collect()
}

/// Search for `query` in the `contents` of the file recieved by `Config`.
/// Returns a Vector containing tuples of usize and &str which represent the
/// line number and line where the query matched case insensitively.
pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str)
    -> Vec<(usize, &'a str)> {
    contents
        .lines()
        // Enumerate to get line numbers.
        .enumerate()
        // Filter to later collect line and number into a Vec.
        .filter(|(_, line)| {
            // Do the search insensitively.
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
        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search(query, contents)
        );
        let query = "fast";
        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust
safe, fast, productive.
Pic three.";
        assert_eq!(
            vec![(0, "Rust")],
            search_case_insensitive(query, contents)
        );

        let query = "FaSt";
        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search_case_insensitive(query, contents)
        );
    }
}
