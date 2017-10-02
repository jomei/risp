use std::str::Chars;

enum Token {
    LeftParenthesis,
    RightParenthesis,
    Quote,
    Int(i32),
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

    pub fn next(&mut self) -> Option<char> {
        return self.chars.next();
    }
}
pub struct Tokenizer<'a> {
    context: Context<'a>
}

struct TokenizerError(String);

impl TryFrom<char> for Token {
    type Error = TokenizerError;
    fn try_from(some_char: char) -> Result<Token, Self::Error> {
        match some_char {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '\'' => Token::Quote,
             _ => TokenizerError(String::from("abc"))
        }
    }
}


impl <'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer{ context: Context:: input }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenWrapper>, TokenizerError> {
        let mut tokens = Vec::new();
        while let Some(c) = self.context.next() {
            if c.is_alphanumeric() {
                tokens.push(TokenWrapper{token: Token::Int()})
            }
        }
    }

    fn parse_int(first: char, chars: Context) -> i32 {
        let mut s = String::new();
        s.push(first);
        while let Some(c) = chars.next() {

        }
    }

    fn token_hash() -> HashMap<char, Token> {
        let mut h: HashMap<char, Token> = HashMap::new();
        h.insert('(', Token::LeftParenthesis);
        h.insert(')', Token::RightParenthesis);
        h.insert('\'', Token::Quote);
        return h;
    }
}