use crate::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

const STD: [&str; 1] = [
    "print",
];


#[derive(Debug, Clone)]
pub enum Expression {
    Declaration(Vec<Token>),

    // still could be an error
    Call(Call),
}

#[derive(Debug, Clone)]
pub enum Call {
    StdCall(Vec<Token>),
    CustomCall(Vec<Token>),
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn from_vec(token_vec: Vec<Token>) -> Self {
        Parser {
            tokens: token_vec.into_iter().peekable(),
        }
    }

    pub fn produce_expressions(&mut self) -> Vec<Expression> {
        let mut ret_expressions: Vec<Expression> = Vec::<Expression>::new();
        for expression in self {
            match expression {
                Ok(expr) => ret_expressions.push(expr),
                Err(e) => panic!("{}", e),
            }
        }
        ret_expressions
    }


    fn is_std_call(symbol: &Token) -> bool {
        let mut found = false;
        for keyword in STD {
            if &Token::Symbol(keyword.parse().unwrap()) == symbol {
                found = true;
            }
        }
        found
    }


    fn get_next_token_while(&mut self, raw_expression: &mut Vec<Token>) {
        loop {
            match self.tokens.next() {
                Some(t) => {
                    raw_expression.push(t.clone());
                },
                _ => break,
            }
        }
    }
}

impl Iterator for Parser {
    type Item = Result<Expression, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let expression: Result<Expression, String>;
        let mut token_vec: Vec<Token> = Vec::<Token>::new();

        let first_token: Token;
        match self.tokens.next() {
            Some(t) => match t {
                Token::Symbol(s) => first_token = Token::Symbol(s),
                Token::Literal(l) => first_token = Token::Literal(l),
                Token::Error(_) => {
                    //todo error messages
                    panic!()
                },
            },
            None => return None,
        }


        if Self::is_std_call(&first_token) {
            token_vec.push(first_token);
            self.get_next_token_while(&mut token_vec);
            expression = Ok(Expression::Call(Call::StdCall(token_vec)))
        } else {
            token_vec.push(first_token);
            self.get_next_token_while(&mut token_vec);
            expression = Ok(Expression::Declaration(token_vec))
        }


        Some(expression)
    }
}
