use std::path::Path;
use std::fs::File;
use std::io::Write;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use filebuffer::FileBuffer;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LyeParser;



const SOURCE_EXTENSION: &str = "lye";
const DEBUG_EXTENSION: &str = "debug";

fn main() {
    // not constants so clap can be implemented
    let program_dir = "./program/";

    let lye_src_file = program_dir.to_owned() + "src/" + "main." + SOURCE_EXTENSION;

    let inter_file_dir = program_dir.to_owned() + "intermediate/";


    let mut lye_source_code = "".to_string();
    let lye_source_file_buffer = FileBuffer::open(lye_src_file).expect("No lye source file provided.");
    for &character in lye_source_file_buffer.iter() {
        lye_source_code.push(char::from(character));
    }



    let pairs = LyeParser::parse(Rule::identifier_list, &lye_source_code).unwrap();

    for pair in pairs {
        println!("\n");
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::numeric => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
}