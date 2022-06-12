#![allow(unused_variables, unused_imports, dead_code, unused_must_use)]
use std::{env, process, io::{Read, prelude}, error::Error, fs::File};

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{return Err("Command requires more arguments");}
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, filename, case_sensitive})
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let results = if config.case_sensitive{
        sensitive_search(&config.query, &contents)
    }
    else{
        insensitive_search(&config.query, &contents)
    };
    for line in results {
        println!("{0}", line);
    }
    Ok(())
}
pub fn sensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}