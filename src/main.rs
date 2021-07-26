#![feature(exact_size_is_empty)]

use crate::lexer::{Lexer, Token, Literal};
use crate::parser::{Parser, Expression, Call};
use crate::expression_splitter::ExpressionSplitter;
use crate::compiler::Compiler;
use std::path::Path;
use std::fs::File;


//todo make error messages constants so easy to edit

//todo make errors if you put random tokens in unchecked spaces.
// preferably when making expressions so it is not hard to maintain

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

const SOURCE_EXTENSION: &str = "lye";
const INTERMEDIATE_EXTENSION: &str = "lasm";

macro_rules! make_in_order {
    ($in_vec:expr, $index:expr) => {
        match $in_vec.get($index) {
            Some(expr) => match expr {
                Expression::Declaration(decl_vec) => decl_vec.1,
                Expression::Call(call) => match call {
                    Call::StdCall(my_call) => my_call.1,
                    Call::CustomCall(my_call) => my_call.1,
                }
            }

            None => {
                18446744073709551615
            },
        }
    }
}

fn main() {
    let program_dir = "./program/";

    let file_dir = program_dir.to_owned() + "src/";
    let filename = file_dir.clone() + "main." + SOURCE_EXTENSION;

    let intermediate_asm_dir = program_dir.to_owned() + "intermediate/";
    let intermediate_asm_file = intermediate_asm_dir.clone() + "inter." + INTERMEDIATE_EXTENSION;


    let source_file_path = Path::new(filename.as_str());
    if !source_file_path.exists() {
        //File::create(source_file_path).unwrap();
        panic!("No source file! A source file should be located in `<workspace>/src/main.{}`", SOURCE_EXTENSION)
    }



    let tokens = Lexer::from_file(&*filename).unwrap().produce_tokens();
    let expression_splitter = ExpressionSplitter::from_vec(tokens);
    let split_token_vec = expression_splitter.get_token_vec();

    let mut expression_vec: Vec<Vec<Expression>> = Vec::new();
    for token_vec in split_token_vec {
        expression_vec.push(Parser::from_vec(token_vec).produce_expressions());
    }



    println!("{:?}", expression_vec);



    let mut constants: Vec<Expression> = Vec::new();
    let mut mutables: Vec<(String, String, i32)> = Vec::new();
    let mut mutable_reassign: Vec<Expression> = Vec::new();
    let mut mutable_sizes: Vec<String> = Vec::new();
    let mut imports: Vec<Expression> = Vec::new();
    let mut functions: Vec<Expression> = Vec::new();
    for vec_of_expressions in &expression_vec {
        // 0 is the type of expression
        let my_expr = vec_of_expressions.get(0).unwrap();
        match my_expr {
            Expression::Declaration(expr) => match expr.0.get(0).unwrap() {
                Token::Symbol(value) => match value.as_str() {
                    "const" => constants.push(my_expr.clone()),
                    "mutable" => {
                        // 1 is the second token, the name
                        let name = match expr.0.get(1).unwrap() {
                            Token::Literal(val) => match val {
                                Literal::Str(my_str) => format!("{}_0123456789", my_str),
                                _ => panic!("error with assigning a mutable variable."),
                            },
                            _ => panic!("error with assigning a mutable variable.")
                        };

                        // 3rd index is the size (resb, resw, resd)
                        let byte_size = match expr.0.get(3).unwrap() {
                            Token::Symbol(my_str_literal) => my_str_literal.to_owned(),
                            _ => panic!("currently can only make it either `resb` (1 byte), `resw` (2 bytes), or `resd` (4 bytes)."),
                        };

                        // the 5th index is the amount
                        let amount = match expr.0.get(5).unwrap() {
                            Token::Literal(my_int_literal) => match my_int_literal {
                                Literal::Integer(my_int) if &0 < my_int && &5 > my_int => *my_int,
                                _ => panic!("currently can only make it a size that is a integer literal 1-4, inclusive.")
                            }
                            _ => panic!("currently can only make it a size that is a integer literal.")
                        };


                        mutables.push((name, byte_size.clone(), amount));
                        mutable_sizes.push(byte_size);
                    },
                    "import" => imports.push(my_expr.clone()),
                    "let" => mutable_reassign.push(my_expr.clone()),
                    _ => {}
                },
                _ => {}
            },
            Expression::Call(call) => match call {
                Call::StdCall(_) => functions.push(my_expr.clone()),
                Call::CustomCall(e) => panic!("unrecognized token: {:?}", e.0.get(0))
            }
        }
    }


    // make the intermediate file
    let intermediate_asm_dir_path = Path::new(intermediate_asm_file.as_str());
    if !intermediate_asm_dir_path.exists() {
        File::create(intermediate_asm_file.clone()).unwrap();
    }

    let mut compiler = Compiler::new(expression_vec, intermediate_asm_dir_path);



    // extern imports
    for import in imports {
        compiler.add_import(import);
    }
    // boilerplate declaring linker entry etc
    compiler.do_boilerplate();
    // declare the text section and program start
    compiler.do_text_section();
    // add the instructions for function calls
    let mut index0: usize = 0;
    let mut index1: usize = 0;
    let mut instructions_bool = false;
    let mut variable_change_bool = false;
    loop {

        if instructions_bool && variable_change_bool {
            break;
        }

        // if mutable reassign comes first
        // >= so when it gets to max it will trigger and break
        if (make_in_order!(functions, index1) >= make_in_order!(mutable_reassign, index0)) && !variable_change_bool {
            match mutable_reassign.get(index0) {
                Some(v) => {
                    compiler.add_instruction(v.clone(), mutable_sizes.get(index0).unwrap());
                    index0 += 1;
                },
                None => {
                    variable_change_bool = true;
                    continue;
                },
            };
        } else {
            match functions.get(index1) {
                Some(v) => {
                    compiler.add_instruction(v.clone(), "nothing to see here.");
                    index1 += 1;
                },
                None => {
                    instructions_bool = true;
                    continue;
                },
            };
        }
    }
    // add jump to end_program
    compiler.do_jmp_end_program();
    // add the functions
    for function in functions.clone() {
        compiler.add_function(function);
    };
    // gracefully end the program
    compiler.do_end_program();
    // make constants (i do it at the end, i do not know why but i like it here better)
    compiler.do_data_section();
    for constant_expression in constants {
        compiler.add_constant(constant_expression);
    }
    // allocate space for mutable variables
    compiler.do_bss_section();
    for mutable_expression in mutables {
        compiler.add_mutable(mutable_expression);
    }
}
