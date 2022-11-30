use crate::ast::*;
use core::panic;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EnsourceLParser;

fn create_file(filepath: &Path) -> (String, File) {
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
    (
        String::from(identifier),
        File {
            file_name: String::from(identifier),
            file_type,
            spells: HashMap::new(),
            attachments: HashMap::new(),
            rituals: HashMap::new(),
            sigils: HashMap::new(),
            content: Vec::new(),
        },
    )
}

pub fn parse(dirpath: &str, files: &mut HashMap<String, File>) {
    let mut dir = fs::read_dir(dirpath).expect("Couldn't read");
    let mut unparsed: HashMap<String, String> = HashMap::new();
    println!("{:#?}", &dir);
    for path in dir {
        let pth = path.expect("Couldn't get").path();
        match pth
            .as_path()
            .extension()
            .expect("Couldn't get extension")
            .to_str()
        {
            Some(i) => match i {
                "necr" | "wiza" | "sorc" | "hexy" => {
                    let file = create_file(pth.as_path());
                    files.insert((&file.0).clone(), file.1);
                    unparsed.insert(
                        file.0,
                        fs::read_to_string(pth.as_path()).expect("Couldn't read"),
                    );
                }
                _ => (),
            },
            None => (),
        }
    }

    let mut public_ritual_TODO: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut private_ritual_TODO: HashMap<(String, String), Vec<String>> = HashMap::new();

    for mut file_p in files {
        let source = EnsourceLParser::parse(
            Rule::file,
            unparsed.get(file_p.0).expect("Unparsed doesn't exists"),
        )
        .expect("Couldn't parse the file");
        pre_parse(
            source,
            file_p.1,
            &mut public_ritual_TODO,
            &mut private_ritual_TODO,
        );
    }
}

fn pre_parse(
    rules: Pairs<Rule>,
    file: &mut File,
    purt: &mut HashMap<(String, String), Vec<String>>,
    prrt: &mut HashMap<(String, String), Vec<String>>,
) {
    for pair in rules {
        match pair.as_rule() {
            Rule::attach => {
                let at = parse_attachment(pair.as_str(), pair);

                file.attachments.insert(at.0, at.1);
            }
            Rule::ritual_dec => {
                let rd = parse_ritual(pair.as_str(), pair, file, purt, prrt);

                file.rituals.insert(rd.0, rd.1);
            }
            _ => (),
        }
    }
}

fn parse_rest(rules: Pairs<Rule>, file: &mut File) {
    let mut base_ctx: usize = 0;

    for pair in rules {
        let line = pair.as_str();
        match pair.as_rule() {
            Rule::ctx => {
                let str = pair.as_span().as_str();
                base_ctx = str.matches("\t").count();
            }
            Rule::spell_dec => {
                //file.spells.push(parse_spell(base_ctx, line, pair))
            }
            Rule::EOI => (),
            _ => parse_expr(base_ctx, line, pair),
        }
    }
    println!("{:#?}", file);
}

fn parse_expr(base_ctx: usize, line: &str, pair: Pair<Rule>) {}

fn parse_type(
    pair: Pair<Rule>,
    expr: ExprType,
    file: &mut File,
    owner_id: Option<(
        &str,
        &Privacy,
        &mut HashMap<(String, String), Vec<String>>,
        &mut HashMap<(String, String), Vec<String>>,
    )>,
) -> Option<TypeDec> {
    let mut inner = pair.into_inner();

    match inner.next() {
        Some(x) => match x.as_rule() {
            Rule::primitive => {
                let prim = x.as_span().as_str();
                match inner.next() {
                    Some(t) => {
                        match t.as_rule() {
                            Rule::integer_literal => {
                                let s = (t.as_span().as_str())
                                    .parse::<usize>()
                                    .expect("Failed to parse intereger literal")
                                    / byeSize;
                                match inner.next() {
                                    Some(r) => match r.as_rule() {
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
                                                                "Not expected {}",
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
                                                _ => panic!("Not expected {}", er.as_str()),
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
                                                        dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse intereger literal"));
                                                    }
                                                    _ => panic!("Not expected {}", index.as_str()),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::from_primitive(prim, s),
                                                pointer: false,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected {}", t.as_str()),
                                    },
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
                                    Some(er) => match er.as_rule() {
                                        Rule::array => {
                                            let mut dim = Vec::new();
                                            for index in er.into_inner() {
                                                match index.as_rule() {
                                                    Rule::integer_literal => {
                                                        dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse intereger literal"));
                                                    }
                                                    _ => panic!("Not expected {}", index.as_str()),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::from_cprimitive(prim),
                                                pointer: true,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected {}", er.as_str()),
                                    },
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
                                        _ => panic!("Not expected {}", index.as_str()),
                                    }
                                }
                                return Some(TypeDec {
                                    base_type: Type::from_cprimitive(prim),
                                    pointer: true,
                                    dimensions: dim.clone(),
                                });
                            }
                            _ => panic!("Not expected {}", t.as_str()),
                        }
                    }
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
                            _ => panic!("Cannot parse to string expr"),
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
                                                    _ => panic!("Not expected {}", index.as_str()),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::str(s.chars().count()),
                                                pointer: true,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected {}", er.as_str()),
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
                                            _ => panic!("Not expected {}", index.as_str()),
                                        }
                                    }
                                    return Some(TypeDec {
                                        base_type: Type::str(s.chars().count()),
                                        pointer: true,
                                        dimensions: dim.clone(),
                                    });
                                }
                                _ => panic!("Not expected {}", t.as_str()),
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
                                                        dim.push(index.as_span().as_str().parse::<usize>().expect("Failed to parse intereger literal"));
                                                    }
                                                    _ => panic!("Not expected {}", index.as_str()),
                                                }
                                            }
                                            return Some(TypeDec {
                                                base_type: Type::char(),
                                                pointer: true,
                                                dimensions: dim.clone(),
                                            });
                                        }
                                        _ => panic!("Not expected {}", er.as_str()),
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
                                        _ => panic!("Not expected {}", index.as_str()),
                                    }
                                }
                                return Some(TypeDec {
                                    base_type: Type::char(),
                                    pointer: true,
                                    dimensions: dim.clone(),
                                });
                            }
                            _ => panic!("Not expected {}", t.as_str()),
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
                let mut dim : Vec<usize> = Vec::new();
                match inner.next() {
                    Some (r) => match r.as_rule() {
                        Rule::pointer => {
                            pointer = true;
                            match inner.next() {
                                Some(r1) => match r1.as_rule() {
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
                                                _ => panic!("Not expected {}", index.as_str()),
                                            }
                                        }
                                    },
                                 _ => panic!("Not expected {}", r1.as_str()),
                                },
                                None => ()
                            }
                        },
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
                                    _ => panic!("Not expected {}", index.as_str()),
                                }
                            }
                        },
                        _ => panic!("Not expected {}", r.as_str()),
                        
                    },
                    None => ()                    
                }

                match owner_id {
                    Some(owner) => {
                        if (owner.0 == prim) && !pointer {
                            println!("Try using {} as a pointer", prim);
                            panic!("Ritual size is infinite");
                        } else {
                            parse_todo_type(String::from(prim), pointer, &dim, file, String::from(owner.0), owner.2, owner.3, owner.1);
                            return None;
                        }
                    }
                    None => {
                        match file.rituals.get(&String::from(prim)) {
                            Some(r) => {
                                return Some(TypeDec { base_type: Type::from_ritual(String::from(prim), r), pointer, dimensions: dim.clone() })
                            },
                            None => {
                                //for attach in file.attachments {
                                    return None;
                                //}
                            }
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
    prim: String,
    pointer: bool,
    dimensions: &Vec<usize>,
    file: &mut File,
    owner_id: String,
    prrt: &mut HashMap<(String, String), Vec<String>>,
    purt: &mut HashMap<(String, String), Vec<String>>,
    privacy: &Privacy,
) {
        let key = (owner_id, file.file_name.clone());
        let mut p = prim + if pointer { "/p" } else { "" };
        if dimensions.len() > 0 {
            p.push_str("/d");
            p.push_str(&*format!("{:#?}", dimensions));
        }
        match privacy {
            Privacy::Forall => {
                if purt.contains_key(&key) {
                    match purt.get_mut(&key) {
                        Some(v1) => v1.push(p),
                        None => {
                            purt.insert(key, vec![p]);
                        }
                    }
                } else {
                    purt.insert(key, vec![p]);
                }
            }
            Privacy::Mine => {
                if prrt.contains_key(&key) {
                    match prrt.get_mut(&key) {
                        Some(v1) => v1.push(p),
                        None => {
                            prrt.insert(key, vec![p]);
                        }
                    }
                } else {
                    prrt.insert(key, vec![p]);
                }
            }
        }
}

/* TODO
fn parse_spell(base_ctx : usize, line: &str, pair : Pair<Rule>) -> Spell{

}*/

fn parse_ritual(
    line: &str,
    pair: Pair<Rule>,
    file: &mut File,
    purt: &mut HashMap<(String, String), Vec<String>>,
    prrt: &mut HashMap<(String, String), Vec<String>>,
) -> (String, Ritual) {
    let mut inner = pair.into_inner();
    let mut privacy: Privacy = Privacy::Forall;
    let mut identifier = "";
    let mut pars: Vec<Par> = Vec::new();
    println!("{:#?}", line);
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
                                        i2.next().expect("No type found"),
                                        ExprType::None,
                                        file,
                                        Option::Some((identifier, &privacy, purt, prrt)),
                                    ) {
                                        Some(t) => ty = t,
                                        None => continue 'tloop,
                                    }
                                }
                                Rule::typee => {
                                    match parse_type(
                                        p2,
                                        ExprType::None,
                                        file,
                                        Option::Some((identifier, &privacy, purt, prrt)),
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
    println!("{:#?}", line);
    let file_type = match inner.next() {
        Some(p) => match p.as_span().as_str() {
            "necr" => FileType::Necr,
            "wiza" => FileType::Wiza,
            "sorc" => FileType::Sorc,
            "hexy" => FileType::Hexy,
            _ => panic!("No proper type given for {:#?}", line),
        },
        None => panic!("No type given for {:#?}", line),
    };
    let file_name = match inner.next() {
        Some(p) => p.as_span().as_str(),
        None => panic!("No filename given for {:#?}", line),
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
