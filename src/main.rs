use std::path::Path;
use std::fs::File;
use std::io::Write;
use crate::parser::parser::Parser;

mod lexer;
mod parser;


macro_rules! println_debug {
    ($input:expr, $id:expr) => {
        println!("[DEBUG {}] {:#?}", $id.to_string(), $input);
    }
}


const SOURCE_EXTENSION: &str = "lye";
const DEBUG_EXTENSION: &str = "debug";

fn main() {
    // not constants so clap can be implemented
    let program_dir = "./program/";

    let lye_src_file = program_dir.to_owned() + "src/" + "main." + SOURCE_EXTENSION;

    let inter_file_dir = program_dir.to_owned() + "intermediate/";


    {
        let lye_src_file_path = Path::new(lye_src_file.as_str());
        if !lye_src_file_path.exists() {
            File::create(&lye_src_file);
        }
    }

    let tokens = lexer::tokenizer::Tokenizer::from_file(&*lye_src_file)
        .expect(
            &*format!("No source file! A source file should be located in `<workspace>/src/main.{}`", SOURCE_EXTENSION)
        ).produce_tokens();


    let ast = Parser::new(tokens).parse();


    println_debug!(ast, -1);
}