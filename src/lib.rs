use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(filename: &String) -> Result<(), Box<Error>> {
    let mut f = File::open(&filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
