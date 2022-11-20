use pest::Parser;
use std::fs;


#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EnsourceLParser;

pub fn parse(filename: &str) {
    let unparsed = fs::read_to_string(filename).expect("Couldn't read");
    println!("{:#?}", unparsed);
    let source = EnsourceLParser::parse(Rule::file, &unparsed);
    for pair in source {
        println!("{:#?}", pair);
    }
}
