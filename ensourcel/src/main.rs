mod ast;
use std::collections::HashMap;

use crate::ast::File;


extern crate pest;
#[macro_use]
extern crate pest_derive;
mod parser;
use parser::parse;
use parser::Rule;
use pest::iterators::Pairs;

fn main() {
    let mut files : HashMap<String, File> = HashMap::new();
    parse("src/tests/", &mut files);
    println!("{:#?}", files);
    
}
