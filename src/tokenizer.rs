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
    fn try_from(c: char) -> Result<Token, Self::Error> {
        match c {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '\'' => Token::Quote,
             _ => TokenizerError(format!("Unexpected token: {}", c))
        }
    }
}


impl <'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer{ context: Context:: input }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenWrapper>, TokenizerError> {
        let mut tokens = Vec::new();
        while let Some(c) = self.context.peek() {
            if c.is_alphanumeric() {
                tokens.push(TokenWrapper{token: Token::Int(parse_int(self.context))})
            }

            let c1 = self.context.next();
            tokens.push(c1.try_into::<Token>().unwrap())

        }
    }

    fn parse_int(&mut chars: Context) -> i32 {
        let mut s = String::new();
        while let Some(c) = chars.peek() && c.is_alphanumeric() {
            let Some(c1) = chars.next();
            s.push(с1)
        }

        return s.parse::<i32>().unwrap();
    }
}