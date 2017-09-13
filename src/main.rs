use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let str: &str = args[0];
    println!(str);
}
