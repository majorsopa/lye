use std::path::Path;
use crate::parser::ast::node::NodeId;

mod lexer;
mod parser;

macro_rules! println_debug {
    ($input:expr, $id:expr) => {
        println!("[DEBUG {}] {}", $id.to_string(), $input);
    }
}

const SOURCE_EXTENSION: &str = "lye";

fn main() {
    let program_dir = "./program/";

    let file_dir = program_dir.to_owned() + "src/";
    let filename = file_dir.clone() + "main." + SOURCE_EXTENSION;



    let source_file_path = Path::new(filename.as_str());
    if !source_file_path.exists() {
        //File::create(source_file_path).unwrap();
        panic!("No source file! A source file should be located in `<workspace>/src/main.{}`", SOURCE_EXTENSION)
    }




    let tokens = lexer::tokenizer::Tokenizer::from_file(&*filename).unwrap().produce_tokens();



    let syntax_tree = parser::parser::Parser::from_tokens(tokens).parse();


    println_debug!(syntax_tree, -1);
}