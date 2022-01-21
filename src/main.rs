mod lexer;
mod scanner;
use std::{env, fs, io};

use crate::scanner::scanner::Scanner;

// TODO Move to struct
static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let contents = fs::read_to_string(path).expect("Error reading file");
    run(contents);
}

fn run_prompt() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Could not read line");
        run(buffer);
        unsafe {
            HAD_ERROR = false;
        }
    }
}

fn run(source: String) {
    print!("Running: {}", source);
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
}

fn error(line: i32, message: &str) {
    report(line, message);
    unsafe {
        HAD_ERROR = true;
    }
}

fn report(line: i32, message: &str) {
    println!("[line {}] Error: {}", line, message);
}
