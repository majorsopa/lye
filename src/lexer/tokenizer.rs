use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lexer::literal::Literal;
use crate::lexer::token::Token;

const ALL_SYMBOLS: [&str; 24] = [
    // symbol symbols
    ",",
    ":",
    ";",
    "=",
    "!=",
    "==",
    "<",
    ">",
    "<=",
    ">=",
    "(",
    ")",
    "+",
    "-",
    "*",
    "/",
    "//",

    // keywords
    "const",
    "import",
    "function",
    "mutable",
    "let",
    // bools
    "true",
    "false",
];

pub const BINARY_OPERATORS: [&str; 11] = [
    "=",
    "!=",
    "==",
    "<",
    ">",
    "<=",
    ">=",
    "+",
    "-",
    "*",
    "/",
];

pub const STD_LIB: [&str; 1] = [
    "print",
];

pub const COMMENT_SYMBOLS: [&str; 1] = [
    "//",
];

pub const KEYWORDS: [&str; 7] = [
    "const",
    "import",
    "function",
    "mutable",
    "let",
    // bools
    "true",
    "false",
];


pub struct Tokenizer {
    raw_data: Peekable<IntoIter<char>>,
}

impl Tokenizer {
    pub fn from_text(text: &str) -> Self {
        Tokenizer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    pub fn from_file(file_path: &str) -> std::io::Result<Self> {
        //todo buffered reader
        Ok(Self::from_text(&std::fs::read_to_string(file_path)?))
    }

    pub fn produce_tokens(&mut self) -> Vec<Token> {
        let mut ret_token_vec = Vec::<Token>::new();
        for token in self {
            match token {
                Ok(t) => ret_token_vec.push(t),
                Err(e) => panic!("something weird passed to the token producer: {}.", e),
            }
        }
        ret_token_vec
    }


    fn is_literal(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }


    fn grab_next_char_while(&mut self, raw_token: &mut String, cond: fn(char) -> bool) {
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    raw_token.push(*c);
                    // consume next because it was peeked
                    self.raw_data.next();
                }
                _ => break,
            }
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let token: Result<Token, String>;

        let first_char: char;
        loop {
            match self.raw_data.next() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) => {
                    first_char = c;
                    break;
                }
                None => return None,
            }
        }

        if Self::is_literal(first_char) && !first_char.is_numeric() {
            let mut name = first_char.to_string();
            self.grab_next_char_while(&mut name, Self::is_literal);

            if ALL_SYMBOLS.contains(&&*name) {
                token = Ok(Token::Symbol(name));
            } else {
                token = Ok(Token::Literal(Literal::Str(name)));
            }
        } else if first_char.is_numeric() {
            let mut value = first_char.to_string();
            self.grab_next_char_while(&mut value, |c| c.is_numeric());

            token = match value.parse() {
                Ok(i) => Ok(Token::Literal(Literal::Integer(i))),
                Err(_) => Err(format!("Integer literal {} is invalid", value)),
            }
        } else if first_char == '"' {
            let mut value = String::new();
            self.grab_next_char_while(&mut value, |c| c != '"');

            self.raw_data.next();

            token = Ok(Token::Literal(Literal::Str(value)));
        } else {
            let mut raw = first_char.to_string();
            loop {
                if let Some(peek) = self.raw_data.peek() {
                    raw.push(*peek);
                } else {
                    break;
                }

                if ALL_SYMBOLS.contains(&&raw[..]) {
                    self.raw_data.next();
                } else {
                    raw.pop();
                    break;
                }
            }

            token = match &raw[..] {
                // comments, C-style
                s if s == "//" => {
                    self.grab_next_char_while(&mut String::new(), |c| c != '\n');

                    self.next()?
                }
                s if ALL_SYMBOLS.contains(&s) => Ok(Token::Symbol(raw)),
                _ => Err(format!("Unknown token: {}", raw)),
            }
        }

        Some(token)
    }
}
