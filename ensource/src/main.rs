extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EnsourceParser;

fn main() {
    let unparsed = fs::read_to_string("ensource/src/test.necr").expect("couldn't read");
    println!("{:#?}", unparsed);
    let source = EnsourceParser::parse(Rule::file, &unparsed)
        .expect("up").next().unwrap();

    println!("{:#?}", source);



}
