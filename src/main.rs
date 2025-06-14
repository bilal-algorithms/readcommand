use std::env;
use std::fs;
use std::io::ErrorKind;
use readcommand::config::Config;
use readcommand::parse_config;
use colored::Colorize;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let file = fs::read_to_string(config.file_path);
    let content = match file {
        Ok(str) => str,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("{}", "File not found.".red());
                return;
            },
            _ => panic!("Error reading the file."),
        }
    };

    println!("{}", content);

    
}


