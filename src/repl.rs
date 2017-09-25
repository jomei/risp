use std::io::{self, Write};

pub fn run() {
    println!("Rust Lisp interpreter");
    println!("Ctr + D to exit");
    let io = io::stdin();
    let lisp = Lisp::new();
    loop {
        println!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let result = lisp.eval_line(input.as_str());
    }
}

fn process_line(input: String) {

}