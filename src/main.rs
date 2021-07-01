#![feature(exact_size_is_empty)]

use crate::lexer::{Lexer, Token};
use crate::parser::{Parser, Expression};
use crate::expression_splitter::ExpressionSplitter;
use crate::compiler::Compiler;
use std::path::Path;
use std::fs::{File, create_dir};
use std::io::{Read, Write};

//todo make the modules actual modules instead of random files
#[path="./lexer/lexer.rs"]
mod lexer;
#[path="./parser/expression_splitter.rs"]
mod expression_splitter;
#[path="./parser/parser.rs"]
mod parser;
#[path= "compiler/compiler.rs"]
mod compiler;
#[path= "compiler/runner.rs"]
mod runner;

const SOURCE_EXTENSION: &str = "balls"; // funny haha
const INTERMEDIATE_EXTENSION: &str = "bi"; // balls intermediate


fn main() {
    let program_dir = "./program/";
    let filename = program_dir.to_owned() + "src/main." + SOURCE_EXTENSION;
    let intermediate_asm_dir = program_dir.to_owned() + "intermediate/";
    let intermediate_asm_file = intermediate_asm_dir.clone() + "inter." + INTERMEDIATE_EXTENSION;

    let tokens = Lexer::from_file(&*filename).unwrap().produce_tokens();
    let expression_splitter = ExpressionSplitter::from_vec(tokens);
    let split_token_vec = expression_splitter.get_token_vec();

    let mut expression_vec: Vec<Vec<Expression>> = Vec::new();
    for token_vec in split_token_vec {
        expression_vec.push(Parser::from_vec(token_vec).produce_expressions());
    }



    println!("{:?}", expression_vec);



    let mut constants: Vec<Vec<Expression>> = Vec::new();
    for vec_of_expressions in &expression_vec {
        // 0 is the type of expression
        match vec_of_expressions.get(0).unwrap() {
            Expression::Declaration(expr) if match expr.get(0).unwrap() {
                Token::Symbol(value) if Token::Symbol(value.clone()) == Token::Symbol("const".parse().unwrap()) => true,
                _ => false,
            } => constants.push(vec_of_expressions.to_vec()),
            _ => {},
        }
    }

    let intermediate_asm_dir_path = Path::new(intermediate_asm_file.as_str());
    if !intermediate_asm_dir_path.exists() {
        create_dir(intermediate_asm_dir).unwrap();
        File::create(intermediate_asm_file.clone()).unwrap();
    }
    let mut compiler = Compiler::new(expression_vec, intermediate_asm_dir_path);
    compiler.do_boilerplate();
    compiler.do_text_section();
    // add instructions
    //compiler.add_asm();
    compiler.do_end_program();
    compiler.do_data_section();
    for constant_expression in constants {
        compiler.add_constants(constant_expression);
    }
}
