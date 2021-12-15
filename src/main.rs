mod tokenizer;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use crate::tokenizer::tokenizer::Tokenizer;

const KEY_LEXEMES: [&str; 5] = [
    "set",
    "//",
    "=",
    "==",
    ";",
];


fn read_file(input_file: &str) -> std::io::Result<String> {
    let mut contents_string = String::new();
    File::open(
        Path::new(input_file)
    )?.read_to_string(&mut contents_string)?;
    Ok(contents_string)
}

fn main() {
    let lye_src = read_file("./run/main.lye").unwrap();
    let lye_tokenizer = Tokenizer::new(Box::from(KEY_LEXEMES));
    let tokens = lye_tokenizer.tokenize(lye_src);
    for token in tokens {
        println!("{:#?}", token);
    }
}
