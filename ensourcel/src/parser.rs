use crate::ast::*;
use core::panic;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser, pratt_parser,
};
use std::{collections::HashMap, fs, path::Path};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EnsourceLParser;

#[derive(Clone, Debug)]
struct TypeHolder {
    identifier: String,
    prim: String,
    pointer: bool,
    dimensions: Vec<usize>,
}

fn create_file(
    filepath: &Path,
    files: &mut HashMap<Attachment, File>,
    unparsed: &mut HashMap<Attachment, String>,
) -> Attachment {
    let identifier = match filepath.file_stem().unwrap().to_str() {
        Some(i) => i,
        None => "",
    }
    .clone();

    let extension = match filepath.extension().unwrap().to_str() {
        Some(i) => i,
        None => "",
    };

    let file_type = match extension {
        "necr" => FileType::Necr,
        "wiza" => FileType::Wiza,
        "sorc" => FileType::Sorc,
        "hexy" => FileType::Hexy,
        _ => FileType::Necr,
    };

    let at = Attachment {
        file_name: String::from(identifier),
        file_type,
    };
    files.insert(
        (&at).clone(),
        File {
            file_name: String::from(identifier),
            file_type,
            spells: HashMap::new(),
            attachments: HashMap::new(),
            rituals: HashMap::new(),
            sigils: HashMap::new(),
            content: Vec::new(),
        },
    );
    unparsed.insert(
        (&at).clone(),
        String::from(fs::read_to_string(filepath).expect("Couldn't read")),
    );
    at
}

fn get_file(
    filepath: &str,
    attachment: &Attachment,
    files: &mut HashMap<Attachment, File>,
    unparsed: &mut HashMap<Attachment, String>,
) -> bool {
    if files.contains_key(attachment) && unparsed.contains_key(attachment) {
        return false;
    }
    println!("{}", filepath);
    files.insert(
        attachment.clone(),
        File {
            file_name: (&attachment.file_name).clone(),
            file_type: attachment.file_type,
            spells: HashMap::new(),
            attachments: HashMap::new(),
            rituals: HashMap::new(),
            sigils: HashMap::new(),
            content: Vec::new(),
        },
    );

    unparsed.insert(
        attachment.clone(),
        String::from(fs::read_to_string(filepath).expect("Couldn't read")),
    );
    return true;
}

fn get_from_attachments(
    dirp: &str,
    file: &File,
    purt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    prrt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    files: &mut HashMap<Attachment, File>,
    unparsed: &mut HashMap<Attachment, String>,
) {
    for attachment in &file.attachments {
        let extension = match &attachment.1.file_type {
            FileType::Necr => ".necr",
            FileType::Wiza => ".wiza",
            FileType::Sorc => ".sorc",
            FileType::Hexy => ".hexy",
        };
        let filepath = dirp.to_owned() + "/" + &attachment.1.file_name + extension;
        if (get_file(&filepath, &attachment.1, files, unparsed)) {
            let parsed = EnsourceLParser::parse(
                Rule::file,
                unparsed.get(attachment.1).expect("Couldn't get unparsed"),
            );
            match parsed {
                Ok(mut rules) => {
                    let mut file = files
                        .get_mut(attachment.1)
                        .expect("Couldn't get the file")
                        .clone();
                    pre_parse(&mut rules, &mut file, purt, prrt, files);
                    let dirpath = Path::new(&filepath)
                        .parent()
                        .expect("Directory couldn't be reached")
                        .to_str()
                        .expect("Couldn't parse!");

                    get_from_attachments(dirpath, &file, purt, prrt, files, unparsed);
                    files.insert(attachment.1.clone(), file);
                }
                Err(e) => panic!("\n{}", e),
            }
        }
    }
}

pub fn parse(filepath: &Path, files: &mut HashMap<Attachment, File>) -> Attachment {
    let mut unparsed: HashMap<Attachment, String> = HashMap::new();
    let key = &create_file(filepath, files, &mut unparsed);

    {
        let parsed = EnsourceLParser::parse(
            Rule::file,
            unparsed.get(key).expect("Couldn't get unparsed"),
        );
        match parsed {
            Ok(mut rules) => {
                let mut public_ritual_todo: HashMap<(String, Attachment), Vec<TypeHolder>> =
                    HashMap::new();
                let mut private_ritual_todo: HashMap<(String, Attachment), Vec<TypeHolder>> =
                    HashMap::new();
                let mut file = files.get_mut(key).expect("Couldn't get the file").clone();
                pre_parse(
                    &mut rules,
                    &mut file,
                    &mut public_ritual_todo,
                    &mut private_ritual_todo,
                    files,
                );
                let dirpath = Path::new(&filepath)
                    .parent()
                    .expect("Directory couldn't be reached")
                    .to_str()
                    .expect("Couldn't parse!");
                get_from_attachments(
                    dirpath,
                    &file,
                    &mut public_ritual_todo,
                    &mut private_ritual_todo,
                    files,
                    &mut unparsed,
                );
                files.insert(key.clone(), file);

                let mut combined = ((&public_ritual_todo).clone());
                combined.extend((&private_ritual_todo).clone());
                println!("Public Ritual TODO {:#?}", &public_ritual_todo);
                println!("Private Ritual TODO {:#?}", &private_ritual_todo);
                println!("Combined TODO {:#?}", &combined);

                for todo in combined {
                    do_todo(
                        &todo,
                        &mut public_ritual_todo,
                        &mut private_ritual_todo,
                        files,
                    );
                }
                println!("Public Ritual TODO {:#?}", &public_ritual_todo);
                println!("Private Ritual TODO {:#?}", &private_ritual_todo);
            }
            Err(e) => panic!("\n{}", e),
        }

        for attach in &unparsed {
            let parsed = EnsourceLParser::parse(Rule::file, attach.1);

            let mut file = files
                .get_mut(attach.0)
                .expect("Couldn't get the file")
                .clone();
            match parsed {
                Ok(rules) => parse_rest(rules, &mut file, files),
                Err(e) => panic!("\n{}", e),
            }
            files.insert(attach.0.clone(), file);
        }
    }
    return key.clone();
}

// I need to find a way to prevent infinite sizes
fn do_todo(
    todo: &((String, Attachment), Vec<TypeHolder>),
    purt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    prrt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    files: &mut HashMap<Attachment, File>,
) {
    //Removes the value at the beginning so no loops or repetitions
    if purt.contains_key(&todo.0) {
        purt.remove(&todo.0);
    } else if prrt.contains_key(&todo.0) {
        prrt.remove(&todo.0);
    }

    for holder in &todo.1 {
        let private_key = &(holder.prim.clone(), ((todo.0).1).clone());

        match prrt.get(private_key) {
            Some(rits) => {
                do_todo(&(private_key.clone(), rits.clone()), purt, prrt, files);

                let file = files.get_mut(&todo.0 .1).expect("Couldn't get file");
                let rts = &mut file.rituals;
                let r1 = &rts.get(&holder.prim).expect("Coudln't get ritual").clone();
                let r0 = rts.get_mut(&todo.0 .0).expect("Coudln't get ritual");

                r0.content.retain(|x| {
                    !(x.identifier == (&holder.identifier).clone()
                        && x.par_type.base_type.identifier == (&holder.prim).clone())
                });

                r0.content.push(Par {
                    identifier: (&holder.identifier).clone(),
                    par_type: TypeDec {
                        base_type: Type::from_ritual((&holder.prim).clone(), r1),
                        pointer: holder.pointer,
                        dimensions: holder.dimensions.clone(),
                    },
                });
            }
            None => {
                let mut file = files.get(&todo.0 .1).expect("Couldn't get file").clone();
                match (files.get(&todo.0 .1).expect("Couldn't get tge fie").rituals)
                    .get(&holder.prim)
                {
                    Some(rit) => {
                        let r0 = file
                            .rituals
                            .get_mut(&todo.0 .0)
                            .expect("Coudln't get ritual");
                        let r1 = rit.clone();
                        r0.content.retain(|x| {
                            !(x.identifier == (&holder.identifier).clone()
                                && x.par_type.base_type.identifier == (&holder.prim).clone())
                        });
                        r0.content.push(Par {
                            identifier: (&holder.identifier).clone(),
                            par_type: TypeDec {
                                base_type: Type::from_ritual((&holder.prim).clone(), &r1),
                                pointer: holder.pointer,
                                dimensions: holder.dimensions.clone(),
                            },
                        });
                    }
                    None => {
                        for attach in &file.attachments {
                            let f1 = &files
                                .get(attach.1)
                                .expect("Couldn't get attachment")
                                .clone();
                            let public_key = &(holder.prim.clone(), ((attach.1).clone()));
                            match (&f1.rituals).get(&holder.prim) {
                                Some(rit) => match purt.get(public_key) {
                                    Some(rits) => {
                                        do_todo(
                                            &(public_key.clone(), rits.clone()),
                                            purt,
                                            prrt,
                                            files,
                                        );

                                        let r0 = file
                                            .rituals
                                            .get_mut(&todo.0 .0)
                                            .expect("Coudln't get ritual");
                                        let r1 = rit.clone();
                                        r0.content.retain(|x| {
                                            !(x.identifier == (&holder.identifier).clone()
                                                && x.par_type.base_type.identifier
                                                    == (&holder.prim).clone())
                                        });
                                        r0.content.push(Par {
                                            identifier: (&holder.identifier).clone(),
                                            par_type: TypeDec {
                                                base_type: Type::from_ritual(
                                                    (&holder.prim).clone(),
                                                    &r1,
                                                ),
                                                pointer: holder.pointer,
                                                dimensions: holder.dimensions.clone(),
                                            },
                                        });
                                    }
                                    None => {
                                        let r0 = file
                                            .rituals
                                            .get_mut(&todo.0 .0)
                                            .expect("Coudln't get ritual");
                                        let r1 = rit.clone();
                                        r0.content.retain(|x| {
                                            !(x.identifier == (&holder.identifier).clone()
                                                && x.par_type.base_type.identifier
                                                    == (&holder.prim).clone())
                                        });
                                        r0.content.push(Par {
                                            identifier: (&holder.identifier).clone(),
                                            par_type: TypeDec {
                                                base_type: Type::from_ritual(
                                                    (&holder.prim).clone(),
                                                    &r1,
                                                ),
                                                pointer: holder.pointer,
                                                dimensions: holder.dimensions.clone(),
                                            },
                                        });
                                    }
                                },
                                None => {
                                    continue;
                                }
                            }
                        }
                    }
                }
                files.insert((todo.0 .1).clone(), file);
            }
        }
    }
}

fn pre_parse(
    rules: &mut Pairs<Rule>,
    file: &mut File,
    purt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    prrt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    files: &mut HashMap<Attachment, File>,
) {
    for pair in rules {
        let line = pair.as_str();
        match pair.as_rule() {
            Rule::attach => {
                let at = parse_attachment(line, pair);
                file.attachments.insert(at.0, at.1);
            }
            Rule::ritual_dec => {
                let rd = parse_ritual(line, pair, file, purt, prrt, files);
                file.rituals.insert(rd.0, rd.1);
            }
            _ => (),
        }
    }
}

fn parse_rest(rules: Pairs<Rule>, file: &mut File, files: &mut HashMap<Attachment, File>) {
    for pair in rules {
        let line = pair.as_str();
        println!("{:#?}", pair);
        match pair.as_rule() {
            Rule::attach => (),
            Rule::ritual_dec => (),
            Rule::spell_dec => (), //file.spells.push(parse_spell(base_ctx, line, pair)),
            Rule::sigil_asg => {

            }
            Rule::EOI => (),
            _ => () //file.content.push(parse_expr(0, line, pair, file.sigils.clone(), files)),
        }
    }
}


/*fn parse_sigil(
    context: usize,
    line: &str,
    pair: Pair<Rule>
) -> (String, Sigil){
    let inner = pair.into_inner();
}*/



fn parse_expr(
    context: usize,
    line: &str,
    pair: Pair<Rule>,
    reachable_sigils: HashMap<String, Sigil>,
    files: &mut HashMap<Attachment, File>,
) -> Expr {

    match pair.as_rule() {
        Rule::hex_literal => {
            let mut raw = pair.as_span().as_str();
            raw = raw.trim_start_matches("0x");
            Expr {
                context,
                expr_type: TypeDec {
                    base_type: Type::int(4),
                    pointer: false,
                    dimensions: Vec::new(),
                },
                exp: ExprType::Integer(
                    i32::from_str_radix(raw, 16).expect("Couldn't parse hex literal"),
                ),
            }
        }
        Rule::integer_literal => {
            let raw = pair.as_span().as_str();
            Expr {
                context,
                expr_type: TypeDec {
                    base_type: Type::int(4),
                    pointer: false,
                    dimensions: Vec::new(),
                },
                exp: ExprType::Integer(raw.parse().expect("Couldn't parse integer literal"))
            }
        },
        Rule::float_literal => {
            let raw = pair.as_span().as_str();
            Expr {
                context,
                expr_type: TypeDec {
                    base_type: Type::float(4),
                    pointer: false,
                    dimensions: Vec::new(),
                },
                exp: ExprType::Float(raw.parse().expect("Couldn't parse float literal"))
            }
        }
        _ => todo!(),
    }
}

/*
fn parse_spell(base_ctx : usize, line: &str, pair : Pair<Rule>) -> Spell{

}
*/

fn parse_ritual(
    line: &str,
    pair: Pair<Rule>,
    file: &mut File,
    purt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    prrt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    files: &mut HashMap<Attachment, File>,
) -> (String, Ritual) {
    let mut inner = pair.into_inner();
    let mut privacy: Privacy = Privacy::Forall;
    let mut identifier = "";
    let mut pars: Vec<Par> = Vec::new();
    loop {
        let p = inner.next();
        match p {
            Some(p1) => match p1.as_rule() {
                Rule::privacy => {
                    privacy = match p1.as_span().as_str() {
                        "forall" => Privacy::Forall,
                        "mine" => Privacy::Mine,
                        _ => Privacy::Forall,
                    }
                }
                Rule::identifier => {
                    identifier = p1.as_span().as_str();
                }
                Rule::ritual_pars => {
                    let mut i2 = p1.into_inner();
                    'tloop: loop {
                        let mut id = "";
                        let ty: TypeDec;
                        match i2.next() {
                            Some(p2) => match p2.as_rule() {
                                Rule::element_ident => {
                                    id = p2.into_inner().as_str();
                                    match parse_type(
                                        String::from(id),
                                        line,
                                        i2.next().expect("No type found"),
                                        ExprType::None,
                                        file,
                                        Option::Some((identifier, &privacy, purt, prrt)),
                                        files,
                                    ) {
                                        Some(t) => ty = t,
                                        None => continue 'tloop,
                                    }
                                }
                                Rule::typee => {
                                    match parse_type(
                                        String::from(id),
                                        line,
                                        p2,
                                        ExprType::None,
                                        file,
                                        Option::Some((identifier, &privacy, purt, prrt)),
                                        files,
                                    ) {
                                        Some(t) => ty = t,
                                        None => continue 'tloop,
                                    }
                                }
                                _ => break,
                            },
                            None => break,
                        }
                        pars.push(Par {
                            identifier: String::from(id),
                            par_type: ty,
                        });
                    }
                }
                _ => (),
            },

            None => break,
        }
    }

    (
        String::from(identifier),
        Ritual {
            privacy,
            content: pars.clone(),
        },
    )
}

fn parse_attachment(line: &str, pair: Pair<Rule>) -> (String, Attachment) {
    let mut inner = pair.into_inner();
    let file_type = match inner.next() {
        Some(p) => match p.as_span().as_str() {
            "necr" => FileType::Necr,
            "wiza" => FileType::Wiza,
            "sorc" => FileType::Sorc,
            "hexy" => FileType::Hexy,
            _ => panic!("No proper type given for {}", line),
        },
        None => panic!("No type given for {}", line),
    };
    let file_name = match inner.next() {
        Some(p) => p.as_span().as_str(),
        None => panic!("No filename given for {}", line),
    };
    let identifier = match inner.next() {
        Some(p) => p.as_span().as_str(),
        None => file_name,
    };

    (
        String::from(identifier),
        Attachment {
            file_name: String::from(file_name),
            file_type,
        },
    )
}

fn parse_type(
    ident: String,
    line: &str,
    pair: Pair<Rule>,
    expr: ExprType,
    file: &mut File,
    owner_id: Option<(
        &str,
        &Privacy,
        &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
        &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    )>,
    files: &mut HashMap<Attachment, File>,
) -> Option<TypeDec> {
    let mut inner = pair.into_inner();

    match inner.next() {
        Some(x) => match x.as_rule() {
            Rule::primitive => {
                let prim = x.as_span().as_str();
                match inner.next() {
                    Some(t) => match t.as_rule() {
                        Rule::integer_literal => {
                            let s = (t.as_span().as_str())
                                .parse::<usize>()
                                .expect("Failed to parse integer literal")
                                / BYE_SIZE;
                            match inner.next() {
                                Some(r) => {
                                    match r.as_rule() {
                                        Rule::pointer => match inner.next() {
                                            Some(er) => match er.as_rule() {
                                                Rule::array => {
                                                    let mut dim = Vec::new();
                                                    for index in er.into_inner() {
                                                        match index.as_rule() {
                                                            Rule::integer_literal => {
                                                                dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse intereger literal"));
                                                            }
                                                            _ => panic!(
                                                                "Not expected at {} for {}",
                                                                line,
                                                                index.as_str()
                                                            ),
                                                        }
                                                    }
                                                    return Some(TypeDec {
                                                        base_type: Type::from_primitive(prim, s),
                                                        pointer: true,
                                                        dimensions: dim.clone(),
                                                    });
                                                }
                                                _ => panic!(
                                                    "Not expected at {} for {}",
                                                    line,
                                                    er.as_str()
                                                ),
                                            },
                                            None => {
                                                return Some(TypeDec {
                                                    base_type: Type::from_primitive(prim, s),
                                                    pointer: true,
                                                    dimensions: Vec::new(),
                                                })
                                            }
                                        },
                                        Rule::array => {
                                            let mut dim = Vec::new();
                                            for index in r.into_inner() {
                                                match index.as_rule() {
                                                    Rule::integer_literal => {
                                                        dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse integer literal"));
                                                    }
                                                    _ => panic!(
                                                        "Not expected at {} for {}",
                                                        line,
                                                        index.as_str()
                                                    ),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::from_primitive(prim, s),
                                                pointer: false,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected at {} for {}", line, t.as_str()),
                                    }
                                }
                                None => {
                                    return Some(TypeDec {
                                        base_type: Type::from_primitive(prim, s),
                                        pointer: false,
                                        dimensions: Vec::new(),
                                    })
                                }
                            }
                        }
                        Rule::pointer => {
                            match inner.next() {
                                Some(er) => {
                                    match er.as_rule() {
                                        Rule::array => {
                                            let mut dim = Vec::new();
                                            for index in er.into_inner() {
                                                match index.as_rule() {
                                                    Rule::integer_literal => {
                                                        dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse integer literal"));
                                                    }
                                                    _ => panic!(
                                                        "Not expected at {} for {}",
                                                        line,
                                                        index.as_str()
                                                    ),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::from_cprimitive(prim),
                                                pointer: true,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected at {} for {}", line, er.as_str()),
                                    }
                                }
                                None => {
                                    return Some(TypeDec {
                                        base_type: Type::from_cprimitive(prim),
                                        pointer: true,
                                        dimensions: Vec::new(),
                                    })
                                }
                            }
                        }
                        Rule::array => {
                            let mut dim = Vec::new();
                            for index in t.into_inner() {
                                match index.as_rule() {
                                    Rule::integer_literal => {
                                        dim.push(
                                            index
                                                .as_span()
                                                .as_str()
                                                .parse::<usize>()
                                                .expect("Failed to parse intereger literal"),
                                        );
                                    }
                                    _ => panic!("Not expected at {} for {}", line, index.as_str()),
                                }
                            }
                            return Some(TypeDec {
                                base_type: Type::from_cprimitive(prim),
                                pointer: true,
                                dimensions: dim.clone(),
                            });
                        }
                        _ => panic!("Not expected at {} for {}", line, t.as_str()),
                    },
                    None => {
                        return Some(TypeDec {
                            base_type: Type::from_cprimitive(prim),
                            pointer: false,
                            dimensions: Vec::new(),
                        })
                    }
                }
            }
            Rule::sized_primitive => {
                let prim = x.as_span().as_str();
                match prim {
                    "string" | "str" => {
                        let s = match expr {
                            ExprType::String(ss) => ss,
                            ExprType::Integer(ir) => ir.to_string(),
                            ExprType::Fixed(fr) => fr.to_string(),
                            _ => String::from(""),
                            //panic!("Cannot parse to string expr at {}", x.as_str())
                        };
                        match inner.next() {
                            Some(t) => match t.as_rule() {
                                Rule::pointer => match inner.next() {
                                    Some(er) => match er.as_rule() {
                                        Rule::array => {
                                            let mut dim = Vec::new();
                                            for index in er.into_inner() {
                                                match index.as_rule() {
                                                    Rule::integer_literal => {
                                                        dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse intereger literal"));
                                                    }
                                                    _ => panic!(
                                                        "Not expected at {} for {}",
                                                        line,
                                                        index.as_str()
                                                    ),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::str(s.chars().count()),
                                                pointer: true,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected at {} for {}", line, er.as_str()),
                                    },
                                    None => {
                                        return Some(TypeDec {
                                            base_type: Type::str(s.chars().count()),
                                            pointer: true,
                                            dimensions: Vec::new(),
                                        })
                                    }
                                },
                                Rule::array => {
                                    let mut dim = Vec::new();
                                    for index in t.into_inner() {
                                        match index.as_rule() {
                                            Rule::integer_literal => {
                                                dim.push(
                                                    index
                                                        .as_span()
                                                        .as_str()
                                                        .parse::<usize>()
                                                        .expect(
                                                            "Failed to parse intereger literal",
                                                        ),
                                                );
                                            }
                                            _ => panic!(
                                                "Not expected at {} for {}",
                                                line,
                                                index.as_str()
                                            ),
                                        }
                                    }
                                    return Some(TypeDec {
                                        base_type: Type::str(s.chars().count()),
                                        pointer: true,
                                        dimensions: dim.clone(),
                                    });
                                }
                                _ => panic!("Not expected at {} for {}", line, t.as_str()),
                            },
                            None => {
                                return Some(TypeDec {
                                    base_type: Type::str(s.chars().count()),
                                    pointer: false,
                                    dimensions: Vec::new(),
                                })
                            }
                        }
                    }
                    "char" | "c" => match inner.next() {
                        Some(t) => match t.as_rule() {
                            Rule::pointer => {
                                match inner.next() {
                                    Some(er) => match er.as_rule() {
                                        Rule::array => {
                                            let mut dim = Vec::new();
                                            for index in er.into_inner() {
                                                match index.as_rule() {
                                                    Rule::integer_literal => {
                                                        dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse integer literal"));
                                                    }
                                                    _ => panic!(
                                                        "Not expected at {} for {}",
                                                        line,
                                                        index.as_str()
                                                    ),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::char(),
                                                pointer: true,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected at {} for {}", line, er.as_str()),
                                    },
                                    None => {
                                        return Some(TypeDec {
                                            base_type: Type::char(),
                                            pointer: true,
                                            dimensions: Vec::new(),
                                        })
                                    }
                                }
                            }
                            Rule::array => {
                                let mut dim = Vec::new();
                                for index in t.into_inner() {
                                    match index.as_rule() {
                                        Rule::integer_literal => {
                                            dim.push(
                                                index
                                                    .as_span()
                                                    .as_str()
                                                    .parse::<usize>()
                                                    .expect("Failed to parse intereger literal"),
                                            );
                                        }
                                        _ => panic!(
                                            "Not expected at {} for {}",
                                            line,
                                            index.as_str()
                                        ),
                                    }
                                }
                                return Some(TypeDec {
                                    base_type: Type::char(),
                                    pointer: true,
                                    dimensions: dim.clone(),
                                });
                            }
                            _ => panic!("Not expected at {} for {}", line, t.as_str()),
                        },
                        None => {
                            return Some(TypeDec {
                                base_type: Type::char(),
                                pointer: false,
                                dimensions: Vec::new(),
                            })
                        }
                    },
                    _ => panic!("No prim"),
                }
            }
            Rule::identifier => {
                let prim = x.as_span().as_str();
                let mut pointer = false;
                let mut dim: Vec<usize> = Vec::new();
                match inner.next() {
                    Some(r) => match r.as_rule() {
                        Rule::pointer => {
                            pointer = true;
                            match inner.next() {
                                Some(r1) => {
                                    match r1.as_rule() {
                                        Rule::array => {
                                            for index in r1.into_inner() {
                                                match index.as_rule() {
                                                    Rule::integer_literal => {
                                                        dim.push(
                                                        index
                                                            .as_span()
                                                            .as_str()
                                                            .parse::<usize>()
                                                            .expect("Failed to parse intereger literal"),
                                                    );
                                                    }
                                                    _ => panic!(
                                                        "Not expected at {} for {}",
                                                        line,
                                                        index.as_str()
                                                    ),
                                                }
                                            }
                                        }
                                        _ => panic!("Not expected at {} for {}", line, r1.as_str()),
                                    }
                                }
                                None => (),
                            }
                        }
                        Rule::array => {
                            for index in r.into_inner() {
                                match index.as_rule() {
                                    Rule::integer_literal => {
                                        dim.push(
                                            index
                                                .as_span()
                                                .as_str()
                                                .parse::<usize>()
                                                .expect("Failed to parse intereger literal"),
                                        );
                                    }
                                    _ => panic!("Not expected at {} for {}", line, index.as_str()),
                                }
                            }
                        }
                        _ => panic!("Not expected at {} for {}", line, r.as_str()),
                    },
                    None => (),
                }

                match owner_id {
                    Some(owner) => {
                        if (owner.0 == prim) && !pointer {
                            println!("Try using {} as a pointer", prim);
                            panic!("Ritual size is infinite");
                        } else {
                            parse_todo_type(
                                ident,
                                String::from(prim),
                                pointer,
                                &dim,
                                file,
                                String::from(owner.0),
                                owner.2,
                                owner.3,
                                owner.1,
                            );
                            return None;
                        }
                    }
                    None => match file.rituals.get(&String::from(prim)) {
                        Some(r) => {
                            return Some(TypeDec {
                                base_type: Type::from_ritual(String::from(prim), r),
                                pointer,
                                dimensions: dim.clone(),
                            })
                        }
                        None => {
                            for attach in &file.attachments {
                                match files.get(attach.1) {
                                    Some(f) => match f.rituals.get(&String::from(prim)) {
                                        Some(r) => {
                                            return Some(TypeDec {
                                                base_type: Type::from_ritual(String::from(prim), r),
                                                pointer,
                                                dimensions: dim.clone(),
                                            })
                                        }
                                        None => continue,
                                    },
                                    None => panic!("No file attached! {}", x.as_str()),
                                }
                            }
                            panic!("Couldn't find the ritual for the type! {}", x.as_str());
                        }
                    },
                }
            }
            _ => panic!("No type"),
        },
        None => panic!("No type"),
    }
}

fn parse_todo_type(
    identifier: String,
    prim: String,
    pointer: bool,
    dimensions: &Vec<usize>,
    file: &mut File,
    owner_id: String,
    prrt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    purt: &mut HashMap<(String, Attachment), Vec<TypeHolder>>,
    privacy: &Privacy,
) {
    let key = (
        owner_id,
        Attachment {
            file_name: (&file.file_name).clone(),
            file_type: file.file_type,
        },
    );
    let p = TypeHolder {
        identifier,
        prim,
        pointer,
        dimensions: dimensions.clone(),
    };

    match privacy {
        Privacy::Forall => match purt.get_mut(&key) {
            Some(v1) => v1.push(p),
            None => {
                purt.insert(key, vec![p]);
            }
        },
        Privacy::Mine => match prrt.get_mut(&key) {
            Some(v1) => v1.push(p),
            None => {
                prrt.insert(key, vec![p]);
            }
        },
    }
}

