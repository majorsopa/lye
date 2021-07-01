use crate::parser::Expression;
use std::iter::Peekable;
use std::vec::IntoIter;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use crate::lexer::{Token, Literal};

const BOILERPLATE_ASM: &str =
"extern _GetStdHandle@4
extern _WriteFile@20
extern _ExitProcess@4

global Start

";

const TEXT_SECTION_ASM: &str =
"section .text

Start:

";

const END_PROGRAM_ASM: &str =
"end_program:
    push 0
    call _ExitProcess@4

";

const DATA_SECTION_ASM: &str =
"section .data
";


pub struct Compiler {
    expressions: Peekable<IntoIter<Peekable<IntoIter<Expression>>>>,
    file: File,
}

impl Compiler {
    pub fn new(expression_vec: Vec<Vec<Expression>>, output: &Path) -> Self {
        let output_file = File::create(output).unwrap();

        let mut ret_vec = Vec::new();

        for expression in expression_vec {
            ret_vec.push(expression.into_iter().peekable());
        }

        Compiler {
            expressions: ret_vec.into_iter().peekable(),
            file: output_file,
        }
    }


    pub fn do_boilerplate(&mut self) {
        self.file.write_all(BOILERPLATE_ASM.as_bytes()).expect("Failed to write boilerplate to file.");
    }

    pub fn do_text_section(&mut self) {
        self.file.write_all(TEXT_SECTION_ASM.as_bytes()).expect("Failed to write text header to file.");
    }

    pub fn do_end_program(&mut self) {
        self.file.write_all(END_PROGRAM_ASM.as_bytes()).expect("Failed to write end code to file.");
    }

    pub fn do_data_section(&mut self) {
        self.file.write_all(DATA_SECTION_ASM.as_bytes()).expect("Failed to write data section to file.");
    }


    pub fn add_constants(&mut self, expression_vec: Vec<Expression>) {
        // 4 spaces for a tab in
        let mut code: String = String::from("    ");
        let expression = match expression_vec.get(0).unwrap() {
            Expression::Declaration(var) if var.get(0).unwrap() ==
                &Token::Symbol("const".parse().unwrap()) => var,
            _ => panic!("something else passed to add_constant method.")
        };

        // index 1 to get the variable name
        match expression.get(1).unwrap() {
            Token::Literal(Literal::Str(name)) => {
                code.push_str(name);
                code.push(' ');
            },
            _ => panic!("incorrect constant syntax."),
        }

        let mut is_literal: bool = true;
        // index 3 to get the value
        let mut cont_value_literal: &Literal = &Literal::Str("rust compiler is dumb".parse().unwrap());

        match expression.get(3).expect("incorrect constant syntax.") {
            Token::Literal(val) => cont_value_literal = val,
            // 0 is false 1 is true
            Token::Symbol(val) if val == &"true".to_string() => {
                code.push_str("equ ");
                code.push_str("1");
                is_literal = false;
            },
            Token::Symbol(val) if val == &"false".to_string() => {
                code.push_str("equ ");
                code.push_str("0");
                is_literal = false;
            },
            _ => panic!("incorrect constant syntax."),
        };

        if is_literal {
            match cont_value_literal {
                Literal::Str(val) => {
                    code.push_str("db ");
                    code.push('"');
                    code.push_str(val);
                    code.push('"');
                },
                Literal::Integer(val) => {
                    code.push_str("equ ");
                    code.push_str(val.to_string().as_str());
                },
            }
        }

        code.push('\n');
        self.file.write_all(code.as_bytes()).expect("Failed to write code to file.");
    }

    /*pub fn add_asm(&mut self, expression_vec: Vec<Expression>) {
        let code: &str = "";

        self.file.write_all(code.as_bytes()).expect("Failed to write code to file.");
    }*/
}
