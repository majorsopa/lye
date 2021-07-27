use std::path::Path;
use std::fs::File;
use std::io::Write;

mod lexer;
mod parser;

/*macro_rules! println_debug {
    ($input:expr, $id:expr) => {
        println!("[DEBUG {}] {}", $id.to_string(), $input);
    }
}*/

const SOURCE_EXTENSION: &str = "lye";
const DEBUG_EXTENSION: &str = "debug";

fn main() {
    // not constants so clap can be implemented
    let program_dir = "./program/";

    let src_file_dir = program_dir.to_owned() + "src/";
    let lye_src_file = src_file_dir.clone() + "main." + SOURCE_EXTENSION;

    let inter_file_dir = program_dir.to_owned() + "intermediate/";
    let lye_tree_file = inter_file_dir + "ast." + DEBUG_EXTENSION;



    let tokens = lexer::tokenizer::Tokenizer::from_file(&*lye_src_file)
        .expect(
            &*format!("No source file! A source file should be located in `<workspace>/src/main.{}`", SOURCE_EXTENSION)
        ).produce_tokens();



    let syntax_tree = parser::parser::Parser::from_tokens(tokens).parse();


    // for debugging the syntax tree
    File::create(
        Path::new(
            lye_tree_file.as_str()
        )
    ).expect("error making tree file")
        .write_all(
            syntax_tree
                .to_string()
                .as_bytes()
        )
        .expect("failed to write tree to file");
}