use std::error::Error;
use std::fs;

use crate::minigrep::data::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let search_results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in search_results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests_minigrep {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\n\
            Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents))
    }
}
