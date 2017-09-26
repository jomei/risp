use std::str::Chars;

struct Context<'a> {
    chars: Chars<'a>
}

impl <'a> Context <'a> {
    pub fn new(str: &'a str) -> Self {
        Context { chars: str.chars() }
    }
}
pub struct Tokenizer<'a> {
    context: Context<'a>
}

impl <'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer{ context: Context:: input }
    }
}