mod ast;
use std::{collections::HashMap, path::Path};

use crate::ast::{File, Attachment};


extern crate pest;
#[macro_use]
extern crate pest_derive;
mod parser;
use parser::parse;

fn main() {
    let mut files : HashMap<Attachment, File> = HashMap::new();
    parse(Path::new("src/tests/cast.necr"), &mut files);
    println!("{:#?}", files);
    
}
