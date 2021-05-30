use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1)
    });
    if let Err(e) = run(&config.filename) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

fn run(filename: &String) -> Result<(), Box<Error>> {
    let mut f = File::open(&filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
