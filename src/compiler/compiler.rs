use crate::parser::{Expression, Call};
use std::fs::File;
use std::path::Path;
use std::io::Write;
use crate::lexer::{Token, Literal};


const NASM_IMPORT: &str =
"extern ";


// imports
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
"    jmp    end_program

";

const END_PROGRAM_ASM: &str =
"end_program:
    push    0
    call    _ExitProcess@4

";

const DATA_SECTION_ASM: &str =
"section .data

";

const BSS_SECTION_ASM: &str =
"section .bss
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

macro_rules! call_std_print_function {
    ($str_to_push_to:expr, $variable_name:expr) => {
        $str_to_push_to.push_str(format!(
"    push {var}
    call std_print_function
    pop ecx
", var = format!("{}_0123456789", $variable_name)
        ).as_str()
        );
    }
}

macro_rules! std_string_length_getter {
    ($str_to_push_to:expr, $variable_name:expr, $function_bool:expr) => {
        if !$function_bool {
            $str_to_push_to.push_str(
"std_string_length_getter:
    push    edx

    xor     ecx, ecx
    dec     edx
    count:
        inc     ecx
        inc     edx
        cmp     byte[edx], 0
        jnz     count
    dec     ecx

    pop     edx
    ret

"
        );
        $function_bool = true;
        }
    }
}

macro_rules! std_print_function {
    ($str_to_push_to:expr, $variable_name:expr, $function_bool:expr, $string_len_getter_bool:expr) => {
        if !$function_bool {
            $str_to_push_to.push_str(
"std_print_function:
    push    ebp
    mov     ebp, esp
    and     esp, 0xfffffff0
    mov     edx, [ebp+8]

    push    -11
    call    _GetStdHandle@4
    mov     ebx, eax

    push    0
    lea     eax, [ebp-4]
    push    eax

    call    std_string_length_getter
    push    ecx

    push    edx
    push    ebx
    call    _WriteFile@20


    mov     esp, ebp
    pop     ebp
    ret

"
        );
            std_string_length_getter!($str_to_push_to, $variable_name, $string_len_getter_bool);
            $function_bool = true;
        }
    }
}


macro_rules! get_function_name {
    ($expression:expr) => {
        match $expression.get(0).unwrap() {

            Token::Symbol(symbol) => symbol,

            _ => panic!("error with getting function name."),
        };
    }
}

macro_rules! get_variable_name {
    ($token_vec:expr, $vec_index:expr) => {
        match $token_vec.get($vec_index)
                .expect("please pass an argument to your function.") {
                Token::Literal(var_name) => match var_name {
                    Literal::Str(val) => val,
                    Literal::Integer(_) => panic!("pass a string.")
                },
                _ => panic!("probably using a taken keyword as a variable name."),
            };
    }
}

macro_rules! get_expression {
    ($expression:expr) => {
        match $expression {
            Expression::Declaration(decl_vec) => decl_vec.0,
            Expression::Call(call) => match call {
                Call::StdCall(my_call) => my_call.0,
                Call::CustomCall(my_call) => my_call.0,
            }
        }
    }
}




pub struct Compiler {
    file: File,

    // bools so double imports don't happen
    windows_std_handle_input_bool: bool,
    writefile_bool: bool,

    // to avoid repeated string count instructions
    std_print_function_bool: bool,
    std_string_length_getter_bool: bool,
}

impl Compiler {
    pub fn new(expression_vec: Vec<Vec<Expression>>, output: &Path) -> Self {
        let output_file = File::create(output).unwrap();

        let mut ret_vec = Vec::new();

        for expression in expression_vec {
            ret_vec.push(expression.into_iter().peekable());
        }

        Compiler {
            file: output_file,

            windows_std_handle_input_bool: false,
            writefile_bool: false,

            std_print_function_bool: false,
            std_string_length_getter_bool: false,
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

    pub fn do_bss_section(&mut self) {
        self.file.write_all(BSS_SECTION_ASM.as_bytes()).expect("Failed to write bss section to file.");
    }


    pub fn add_import(&mut self, expression: Expression) {
        let mut code: String = String::from("");
        let import = match expression {
            Expression::Declaration(var) if var.0.get(0).unwrap() ==
                &Token::Symbol("import".parse().unwrap()) => var,
            _ => panic!("something else passed to add_import method.")
        };

        // index 1 to get the import name
        match import.0.get(1).unwrap() {
            Token::Symbol(in_import) if in_import == "print" => {
                non_doubling_import!(code, WINDOWS_STD_HANDLE_IMPORT, self.windows_std_handle_input_bool);
                non_doubling_import!(code, WRITEFILE, self.writefile_bool);
            },
            _ => panic!("incorrect import syntax."),
        }

        code.push('\n');
        self.file.write_all(code.as_bytes()).expect("Failed to write import to file.");
    }

    pub fn add_constant(&mut self, expression: Expression) {
        // 4 spaces for a tab in
        let mut code: String = String::from("    ");

        let expression = get_expression!(expression);

        // index 1 to get the constant name
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
            Token::Literal(val) => cont_value_literal = &val,
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
                    code.push('`');
                    let mut new_val = val.replace('\r', "");
                    new_val = new_val.replace('\n', "\\n");
                    code.push_str(new_val.as_str());
                    code.push_str("`, 0");
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

    pub fn add_mutable(&mut self, mutable_variable: (String, String, i32)) {
        // 4 spaces for a tab in
        let mut code: String = String::from("    ");

        // the variable name is the first in the tuple
        code.push_str(&*mutable_variable.0);

        code.push_str(": ");

        // size is the second in the tuple
        code.push_str(&*mutable_variable.1);

        code.push(' ');

        // amount is the last in the tuple
        code.push_str(&*mutable_variable.2.to_string());


        code.push('\n');
        self.file.write_all(code.as_bytes()).expect("Failed to write mutable to file.");
    }


    pub fn add_instruction(&mut self, expression: Expression, mov_size: &str) {
        let expression = get_expression!(expression);

        let function = get_function_name!(expression);

        //println!("{:?}", function);

        match function.as_str() {
            "print" => {
                // index 2 is the value contained in the parentheses
                let variable_name = get_variable_name!(expression, 2);


                let mut code = String::from("");
                call_std_print_function!(code, variable_name);


                code.push('\n');
                self.file.write_all(code.as_bytes()).expect("Failed to write print code to file.");
            },
            "let" => {
                let mov_size = match mov_size {
                    "resb" => "byte",
                    "resw" => "word",
                    "resd" => "dword",
                    _ => panic!("error with making mutable reassignment."),
                };

                // 4 spaces for a tab in
                let mut code: String = String::from("    ");

                let mutable_variable = get_variable_name!(expression, 1);
                let mutable_variable_value = get_variable_name!(expression, 3);

                code.push_str(&*format!("mov {mov_size} [{variable}_0123456789], `{value}`", mov_size = mov_size, variable = mutable_variable, value = mutable_variable_value));


                code.push('\n');
                self.file.write_all(code.as_bytes()).expect("Failed to write change mutable instruction to file.");
            }
            _ => panic!("not a method.")
        }
    }

    pub fn add_function(&mut self, expression: Expression) {
        let expression = get_expression!(expression);

        let func_name = get_function_name!(expression).as_str();

        match func_name {
              "print" => {
                  let mut code = String::from("");
                  std_print_function!(code, variable_name, self.std_print_function_bool, self.std_string_length_getter_bool);


                  code.push('\n');
                  self.file.write_all(code.as_bytes()).expect("Failed to write code to file.");
            },
            _ => panic!("`{}` is not a method.", func_name),
        }
    }
}
