mod ast;
use crate::ast::File;


extern crate pest;
#[macro_use]
extern crate pest_derive;
mod parser;
use parser::parse;


fn main() {
    let mut files : Vec<File> = Vec::new();
    parse("src/tests/general_test.necr", files);
}
