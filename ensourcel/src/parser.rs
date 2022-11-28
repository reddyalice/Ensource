
use pest::{Parser, iterators::Pairs, error::Error};
use std::{fs, path::Path};
use crate::ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EnsourceLParser;



fn create_file(filepath : &Path) -> File{

    
    let identifier = match filepath.file_stem().unwrap().to_str() {
        Some(i) => i,
        None => ""
    }.clone();

    let extension = match filepath.extension().unwrap().to_str() {
        Some(i) => i,
        None => ""
    };

    let file_type = match extension {
        "necr" => FileType::Necr,
        "wiza" => FileType::Wiza,
        "sorc" => FileType::Sorc,
        "hexy" => FileType::Hexy,
        _ => FileType::Necr
    };

    File{
        identifier : String::from(identifier),
        file_type,
        spells : Vec::new(),
        attachments : Vec::new(),
        rituals : Vec::new(),
        content : Vec::new()
    }

}


pub fn parse(filepath: &str, mut files : Vec<File>) {

    let path = Path::new(filepath);

    let unparsed = fs::read_to_string(path).expect("Couldn't read");
    println!("Unparsed : {:#?}", unparsed);
    let source = EnsourceLParser::parse(Rule::file, &unparsed);

    files.push(create_file(path));
   
    match files.last_mut() {
        Some(file) =>{
            match source {
                Ok(r) => parse_file(r, file),
                Err(e) => print!("Failed to parse because of {}", e)
            }
        },
        None => panic!("File doesn't exists")
    }
    
   

    
    
}

fn parse_file(rules : Pairs<Rule>, file : &mut File){
    let mut baseCtx : usize;
    
    for pair in rules {
        //println!("{:#?}", pair);
        match pair.as_rule() {
            Rule::ctx => {
                let str = pair.as_span().as_str();
                baseCtx = str.matches("\t").count();
            },
            Rule::spell_dec => {

            }
            
            _ => ()
        }
    }
}
