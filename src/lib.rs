use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents, !config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str, to_lower: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    for line in contents.lines() {
        if to_lower {
            if line.to_lowercase().contains(&query_lower) {
                results.push(line);
            }
        } else {
            if line.contains(query) {
                results.push(line)
            }
        }
    }

    results
}

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_sensitive: bool,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: &args[1],
            filename: &args[2],
            case_sensitive,
        })
    }
}
