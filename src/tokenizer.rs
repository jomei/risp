use std::str::Chars;

enum Token {
    LeftParenthesis,
    RightParenthesis,
    Quote,
    Int(i64),
    String(String)
}

struct TokenWrapper {
    pub token: Token
}

impl TokenWrapper {
    fn new(token: Token) -> Self {
        TokenWrapper { token }
    }
}

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

struct TokenizerError(String);

impl <'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer{ context: Context:: input }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenWrapper>, TokenizerError> {
        let mut tokens = Vec::new();
    }
}