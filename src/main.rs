mod scanner;
mod token;
mod error;

use std::{env, fs, io::{self, Write}, process::exit};
use scanner::Scanner;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    match args.len() {
        0 => run_prompt(),
        1 => {
            match args.get(0) {
                Some(path) => run_file(path),
                None => panic!("Invalid command line argument.")
            }
        },
        _ => {
            println!("Usage: loxi [script file]");
            exit(64);
        },
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(script_content) => {
            run(&script_content);
        },
        Err(e) => panic!("Failed to open file: {}", e.to_string()),
    }
}
    
fn run_prompt() {
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => run(&input),
            Err(e) => panic!("Failed to read line: {}", e.to_string()),
        }
    }
}

fn run(source: &str) {
    let mut sc = Scanner::new(source);
    let tokens = sc.scan_tokens();
    println!("{:#?}", tokens);
}