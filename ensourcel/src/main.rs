mod old;
mod new;
use std::{collections::HashMap, path::Path};

use crate::old::ast::{File, Attachment};

extern crate pest;
#[macro_use]
extern crate pest_derive;
use old::parser::parse;

fn main() {
    let mut files : HashMap<Attachment, File> = HashMap::new();
    let entry_key = parse(Path::new("src/old/example_test/cast.necr"), &mut files);
    println!("{:#?}", files);
    
}
