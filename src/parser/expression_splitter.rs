use crate::lexer::Token;

pub struct ExpressionSplitter {
    tokens: Vec<Vec<Token>>,
}

impl ExpressionSplitter {
    pub fn from_vec(tokens: Vec<Token>) -> Self {
        let mut ret_vec = Vec::new();
        let mut tokens_iter = tokens.iter();

        loop {
            let mut in_vec: Vec<Token> = Vec::new();
            loop {
                match tokens_iter.next() {
                    Some(t) if t != &Token::Symbol(";".parse().unwrap()) => in_vec.push(t.clone()),
                    Some(t) if t == &Token::Symbol(";".parse().unwrap()) => break, // if it is a semi, break
                    _ => panic!("probably missing a semi."),
                }
            }
            ret_vec.push(in_vec);
            if tokens_iter.is_empty() {
                break;
            }
        }

        Self {
            tokens: ret_vec,
        }
    }

    pub fn get_token_vec(self) -> Vec<Vec<Token>> {
        self.tokens
    }
}
