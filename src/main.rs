//! “Yes, they actually wrote machine code by hand. On punched cards. Which,
//! presumably, they punched with their fists.”
//! Excerpt From
//! Crafting Interpreter
//! Robert Nystrom

use std::env::args;
use std::fs;
use std::io::stdin;

use crate::scanner::Scanner;

mod chunk;
mod value;
mod vm;
mod scanner;

fn main() {
    let args = args();
    match args.len() {
        1 => repl(),
        2 => run_file(&args.last().unwrap()),
        _ => eprintln!("Usage: lox-rs [path]")
    }
}

fn repl() {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read input");
        if let Some(line) = buf.strip_suffix("\n") {
            let mut scanner = Scanner::new(line.to_string());
            dbg!(scanner.scan_tokens());
        } else {
            break;
        }
    }
}

fn run_file(file_name: &str) {
    let source = fs::read_to_string(file_name).expect("Missing lox file");
    let mut scanner = Scanner::new(source);
    dbg!(scanner.scan_tokens());
}
