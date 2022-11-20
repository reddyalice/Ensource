extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
use parser::parse;

fn main() {
    parse("src/test.necr");
}
