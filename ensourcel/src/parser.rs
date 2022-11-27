
use pest::{Parser, iterators::Pairs, error::Error};
use std::fs;
use crate::ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EnsourceLParser;

pub fn parse(filename: &str) {
    let unparsed = fs::read_to_string(filename).expect("Couldn't read");
    println!("{:#?}", unparsed);
    let source = EnsourceLParser::parse(Rule::file, &unparsed);
    parse_rule(source);
    
}

fn parse_rule(rules : Result<Pairs<Rule>, Error<Rule>>){
    let mut baseCtx : i8;

    for pair in rules.unwrap() {
        
        match pair.as_rule() {
            Rule::ctx=> {
                
            },

            
            _ => ()
        }
    }
}
