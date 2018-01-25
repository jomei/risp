#![feature(try_from)]
use std::io::{self, Write};

mod parser;
mod tokenizer;

pub struct Lisp {
}

impl Lisp {
    pub fn new() -> Self {
        Lisp {
        }
    }

    pub fn repl() {
        println!("Rust Lisp interpreter");
        println!("Ctr + C to exit");
        let io = io::stdin();
        loop {
            println!(">");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let result = Lisp::eval_line(input.as_str());
            println!("{}", result);
        }
    }

    pub fn eval_line(s: &str) -> String {
        s.to_owned()
    }
}