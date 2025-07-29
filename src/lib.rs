use core::{
    error::Error,
    result::Result::{self, Err},
};
use std::fs;

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    // .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    Ok(())
}
