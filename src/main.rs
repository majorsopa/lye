extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use filebuffer::FileBuffer;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LyeParser;



const SOURCE_EXTENSION: &str = "lye";
//const DEBUG_EXTENSION: &str = "debug";

fn main() {
    // not constants so clap can be implemented
    let program_dir = "./program/";

    let lye_src_file = program_dir.to_owned() + "src/" + "main." + SOURCE_EXTENSION;

    //let inter_file_dir = program_dir.to_owned() + "intermediate/";


    let lye_source_file_chars_vec = FileBuffer::open(lye_src_file).expect("No lye source file provided.").to_vec();
    let mut lye_source_code = String::new();
    for character in lye_source_file_chars_vec {
        lye_source_code.push(char::from(character))
    }

    let mut split_lye_source_code = lye_source_code.split(';');


    let mut need_to_break = false;
    loop {
        match split_lye_source_code.next() {
            Some(mut semi_split) if semi_split != "" => {
                loop {
                    let base_split = semi_split;

                    semi_split = semi_split.trim_start_matches("\r\n");
                    semi_split = semi_split.trim_start_matches('\r');
                    semi_split = semi_split.trim_start_matches('\n');
                    semi_split = semi_split.trim_start_matches('\t');
                    semi_split = semi_split.trim_start_matches(' ');

                    if semi_split.is_empty() {
                        need_to_break = true;
                        break;
                    }

                    if base_split == semi_split {
                        break;
                    }
                }
                if need_to_break {
                    break;
                }


                let expressions = LyeParser::parse(Rule::expression, &semi_split);
                match expressions {
                    Ok(expr) => {
                        for pair in expr {
                            println!("\n");
                            println!("Rule:    {:?}", pair.as_rule());
                            println!("Span:    {:?}", pair.as_span());
                            println!("Text:    {}", pair.as_str());
                        }
                    },
                    Err(_) => continue,
                }
            }
            _ => break,
        }
    }
}