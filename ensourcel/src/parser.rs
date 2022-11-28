
use pest::{Parser, iterators::{Pairs, Pair}};
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


pub fn parse_file(filepath: &str, mut files : Vec<File>) {

    let path = Path::new(filepath);

    let unparsed = fs::read_to_string(path).expect("Couldn't read");
    let source = EnsourceLParser::parse(Rule::file, &unparsed);

    files.push(create_file(path));
   
    match files.last_mut() {
        Some(file) =>{
            match source {
                Ok(rules) => parse_rules(rules, file),
                Err(e) => print!("Failed to parse because of {:#?}", e)
            }
        },
        None => panic!("File doesn't exists")
    }
    
}

fn parse_rules(rules : Pairs<Rule>, file : &mut File){
    let mut base_ctx : usize = 0;
    
    for pair in rules {
        //println!("{:#?}", pair);
        let line = pair.as_str();
        println!("{:#?}", line);
        match pair.as_rule() {
            Rule::ctx => {
                let str = pair.as_span().as_str();
                base_ctx = str.matches("\t").count();
            },
            Rule::attach => file.attachments.push(parse_attachment(base_ctx, line, pair)),
            Rule::spell_dec => {
                //file.spells.push(parse_spell(base_ctx, line, pair))
            },
            Rule::ritual_dec => {
                let mut inner = pair.into_inner();
                let mut privacy : Privacy = Privacy::Forall;
                let mut target : Target = Target::None;
                let mut identifier = "";

                loop{
                    let p = inner.next();
                    match  p {
                        Some(p1) => {
                            match p1.as_rule() {
                                Rule::privacy => {
                                    privacy = match p1.as_span().as_str() {
                                        "forall" => Privacy::Forall,
                                        "mine" => Privacy::Mine,
                                        _ => Privacy::Forall
                                    } 
                                },
                                Rule::target => {
                                    target = match p1.as_span().as_str() {
                                        "golem" => Target::Golem,
                                        "whisper" => Target::Whisper,
                                        _ => Target::None
                                    } 
                                },
                                Rule::identifier => {
                                    identifier = p1.as_span().as_str();
                                },
                                Rule::ritual_pars => {
                                    println!("{:#?}", p1);
                                }
                                _ => ()
                            }
                        },
                        
                        None => break
                    }
                }




            },//file.rituals.push(parse_ritual(base_ctx, line, pair)),
            _ => ()
        }
    }
}

/* TODO 
fn parse_spell(base_ctx : usize, line: &str, pair : Pair<Rule>) -> Spell{

}*/


/*fn parse_ritual(base_ctx : usize, line: &str, pair : Pair<Rule>) -> Ritual{
    let mut inner = pair.into_inner();
}*/


fn parse_attachment(base_ctx : usize, line: &str, pair : Pair<Rule>) -> Attachment{
    let mut inner = pair.into_inner();
    let file_type = match inner.next()
    {
            Some(p) => match p.as_span().as_str(){
                "necr" => FileType::Necr,
                "wiza" => FileType::Wiza,
                "sorc" => FileType::Sorc,
                "hexy" => FileType::Hexy,
                _ => panic!("No proper type given for {:#?}", line) },
            None => panic!("No type given for {:#?}", line)
    };
    let file_name = match inner.next()
    {
        Some(p) => p.as_span().as_str(),
        None => panic!("No filename given for {:#?}", line)
    };
    let identifier = match inner.next()
    {
        Some(p) => p.as_span().as_str(),
        None => file_name
    };

    Attachment{
        identifier : String::from(identifier),
        file_name : String::from(file_name),
        file_type,
        context : base_ctx
    }
}
