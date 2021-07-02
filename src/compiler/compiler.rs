use crate::parser::{Expression, Call};
use std::iter::Peekable;
use std::vec::IntoIter;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use crate::lexer::{Token, Literal};



const NASM_IMPORT: &str =
"extern ";


// imports
const MALLOC_IMPORT: &str =
"malloc";

const WINDOWS_STD_HANDLE_IMPORT: &str =
"_GetStdHandle@4";

const WRITEFILE: &str =
"_WriteFile@20";




// if extern changes this has to change as well
const BOILERPLATE_ASM: &str =
"extern _ExitProcess@4

global Start

";

const TEXT_SECTION_ASM: &str =
"section .text

Start:
";

const END_PROGRAM_JMP: &str =
"    jmp end_program

";

const END_PROGRAM_ASM: &str =
"end_program:
    push 0
    call _ExitProcess@4

";

const DATA_SECTION_ASM: &str =
"section .data
";



macro_rules! non_doubling_import {
    ($str_to_push_to:expr, $import_str_to_push:expr, $set_true:expr) => {
        if !$set_true {
            $set_true = true;

            $str_to_push_to.push_str(NASM_IMPORT);
            $str_to_push_to.push_str($import_str_to_push);
            $str_to_push_to.push('\n');
        }
    }
}

macro_rules! string_length_getter {
    ($str_to_push_to:expr, $variable_name:expr) => {
        $str_to_push_to.push_str(format!(
"    mov edx, {var}
    push edx
    mov ecx,0
    dec edx
    count:
        inc ecx
        inc edx
        cmp byte[edx], 0
        jnz count
    dec ecx
    pop edx

", var = format!("{}_0123456789", $variable_name)
        ).as_str()
        );
    }
}

macro_rules! std_print_function {
    ($str_to_push_to:expr, $variable_name:expr) => {
        $str_to_push_to.push_str(format!(
"    push    -11
    call    _GetStdHandle@4
    mov     ebx, eax

    push    0
    lea     eax, [ebp-4]
    push    eax
    push    ecx
    push    {var}
    push    ebx
    call    _WriteFile@20
", var = format!("{}_0123456789", $variable_name)
        ).as_str()
        );
    }
}



pub struct Compiler {
    expressions: Peekable<IntoIter<Peekable<IntoIter<Expression>>>>,
    file: File,

    // bools so double imports don't happen
    windows_std_handle_input_bool: bool,
    writefile_bool: bool,
    malloc_import_bool: bool,
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

            windows_std_handle_input_bool: false,
            writefile_bool: false,
            malloc_import_bool: false,
        }
    }


    //todo buffered writer
    pub fn do_boilerplate(&mut self) {
        self.file.write_all(BOILERPLATE_ASM.as_bytes()).expect("Failed to write boilerplate to file.");
    }

    pub fn do_text_section(&mut self) {
        self.file.write_all(TEXT_SECTION_ASM.as_bytes()).expect("Failed to write text header to file.");
    }

    pub fn do_jmp_end_program(&mut self) {
        self.file.write_all(END_PROGRAM_JMP.as_bytes()).expect("Failed to write end program jump code to file.");
    }

    pub fn do_end_program(&mut self) {
        self.file.write_all(END_PROGRAM_ASM.as_bytes()).expect("Failed to write end code to file.");
    }

    pub fn do_data_section(&mut self) {
        self.file.write_all(DATA_SECTION_ASM.as_bytes()).expect("Failed to write data section to file.");
    }


    pub fn add_import(&mut self, expression_vec: Vec<Expression>) {
        let mut code: String = String::from("");
        let import = match expression_vec.get(0).unwrap() {
            Expression::Declaration(var) if var.get(0).unwrap() ==
                &Token::Symbol("import".parse().unwrap()) => var,
            _ => panic!("something else passed to add_import method.")
        };

        // index 1 to get the import name
        match import.get(1).unwrap() {
            Token::Symbol(in_import) if in_import == "std_print" => {
                non_doubling_import!(code, WINDOWS_STD_HANDLE_IMPORT, self.windows_std_handle_input_bool);
                non_doubling_import!(code, WRITEFILE, self.writefile_bool);
                non_doubling_import!(code, MALLOC_IMPORT, self.malloc_import_bool);
            },
            _ => panic!("incorrect import syntax."),
        }

        code.push('\n');
        self.file.write_all(code.as_bytes()).expect("Failed to write import to file.");
    }

    pub fn add_constant(&mut self, expression_vec: Vec<Expression>) {
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
                code.push_str(&*format!("{}_0123456789 ", name));
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
                    code.push_str("'");
                    code.push_str(val);
                    code.push_str("'");
                },
                Literal::Integer(val) => {
                    code.push_str("equ ");
                    code.push_str(val.to_string().as_str());
                },
            }
        }

        code.push('\n');
        self.file.write_all(code.as_bytes()).expect("Failed to write constant to file.");
    }

    pub fn add_instruction(&mut self, expression_vec: Vec<Expression>) {
        // will have this be recursive so you have define custom functions


        // 0 is the index of the function token
        let function = match expression_vec.get(0) {


            Some(expr) => match expr {
                Expression::Call(call) => match call {
                    Call::StdCall(std_call) => std_call,
                    Call::CustomCall(custom_call) => custom_call,
                },
                _ => panic!("something else passed to add_instruction method.")
            },
            None => panic!("function compilation failed"),
        };


        if match function.get(0).unwrap() {
            Token::Symbol(function_name) => function_name,
            _ => panic!("something else passed to add_instruction method."),
        } == "print" {

            // index 2 is the value contained in the parentheses
            let variable_name = match function.get(2)
                .expect("please pass an argument to your function.") {
                Token::Literal(var_name) => match var_name {
                    Literal::Str(val) => val,
                    Literal::Integer(_) => panic!("pass a string to print.")
                },
                _ => panic!("probably using a taken keyword as a variable name."),
            };


            let mut code = String::from("");
            string_length_getter!(code, variable_name);
            std_print_function!(code, variable_name);


            code.push('\n');
            self.file.write_all(code.as_bytes()).expect("Failed to write code to file.");
        }
    }
}
