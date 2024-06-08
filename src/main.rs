pub mod scanner;
pub mod utils;

use std::env;
use std::process;
use std::path::PathBuf;
use std::io;

use scanner::Scanner;

fn main() {
    // First arg is the executable location 
    let args: Vec<String> = env::args().collect();
    let n_args = args.len();
    dbg!(args.clone());

    if n_args > 2 {
        abort!("Usage: lox [script]");
    } else if n_args == 2 {
        run_file(&args[1])
    } else if n_args == 1 {
        run_prompt()
    }

}

#[macro_export]
macro_rules! abort {
    ( $x:literal ) => {
        eprint!($x);
        process::exit(64);
    };
    ( $x:literal, $y:literal ) => {
        eprint!($x);
        process::exit($y);
    };
}

fn run_file(file: &String) {
    let path = PathBuf::from(file.to_owned());
    let contents = std::fs::read_to_string(path).unwrap();
    run(contents);
}

fn run_prompt() {
    loop {
        print!("lox> ");
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line == "" { break };

        run(line);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let _ = scanner.scan_tokens();

    println!("{:?}", scanner.tokens);
}

