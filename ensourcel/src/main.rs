mod ast;
use crate::ast::File;


extern crate pest;
#[macro_use]
extern crate pest_derive;
mod parser;
use parser::parse_file;


fn main() {
    let files : Vec<File> = Vec::new();
    parse_file("src/tests/general_test.necr", files);
    
}
