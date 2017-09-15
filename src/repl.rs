use std::io;

pub fn run() {
    println!("Rust Lisp interpreter");
    println!("Ctr + D to exit");
    let io = io::stdin();
    loop {
        println!(">");
        let mut input = String::new();
        match io.read_line(&mut input) {
            Ok(_) => process_line(input),
            _ => {
                println!("Bye");
                break;
            }

        }
    }
}

fn process_line(input: String) {

}