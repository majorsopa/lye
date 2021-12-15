use crate::tokenizer::token::Token;
use std::slice::Iter;
use crate::tokenizer::token::Token::Literal;

pub struct Tokenizer {
    key_lexemes: Vec<String>,
}

impl Tokenizer {
    pub fn new(key_lexemes: Box<[&str]>) -> Self {
        Self {
            key_lexemes: {
                let mut ret_vec = Vec::new();
                for lex_str in key_lexemes.to_vec() {
                    ret_vec.push(lex_str.to_string());
                }
                ret_vec
            }
        }
    }

    pub fn tokenize(&self, input_string: String) -> Vec<Token> {
        let mut input_string = input_string.chars().peekable();
        let mut ret_vec = Vec::new();

        while input_string.peek().is_some() {
            let mut token_string = String::new();

            match input_string.peek().unwrap() {
                '\"' => {
                    // this deals with string literals
                    input_string.next().unwrap();
                    while input_string.peek().unwrap() != &'\"' {
                        token_string.push(input_string.next().unwrap());
                    }
                    ret_vec.push(Token::new_string_literal(token_string));
                    input_string.next().unwrap();
                }
                ' ' => { input_string.next().unwrap(); }  // this will not be reached inside of a string literal
                '0'..='9' | '-' => {
                    'i32_loop: loop {
                        match input_string.peek() {
                            None => break 'i32_loop,
                            Some(i32_char) => match i32_char {
                                '0'..='9' => token_string.push(input_string.next().unwrap()),
                                _ => break 'i32_loop,
                            }
                        }
                    }
                    ret_vec.push(Token::from_i32(token_string.parse::<i32>().unwrap()));
                }
                _ => {
                    let mut is_key_lexeme = false;
                    'key_lexeme_char_loop: for key_lexeme in &self.key_lexemes {
                        'lexeme_char_loop: for character in key_lexeme.chars() {
                            match input_string.peek() {
                                None => break 'key_lexeme_char_loop,
                                Some(next_char) => {
                                    if next_char == &character {
                                        token_string.push(input_string.next().unwrap());
                                        is_key_lexeme = true;
                                    } else {
                                        is_key_lexeme = false;
                                        break 'lexeme_char_loop;
                                    }
                                },
                            }
                        }
                    }
                    if !is_key_lexeme {  // not a key lexeme
                        'custom_lexeme_loop: while input_string.peek().is_some() {
                            match input_string.peek().unwrap() {
                                ' ' => break 'custom_lexeme_loop,
                                _ => token_string.push(input_string.next().unwrap()),
                            }
                        }
                    }
                    ret_vec.push(Token::new_lexeme(token_string));
                }
            }
        }

        ret_vec
    }
}
