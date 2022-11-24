use std::{vec, boxed::Box};
use std::mem;

enum FileType{
    Necr,
    Sorc,
    Wiza,
    Hexy
}

enum Privacy{
    Forall,
    Mine
}

enum Target{
    Golem,
    Whisper
}

trait Expr{
}


struct TypeDec{
    base_type : Type,
    pointer : bool,
    dimensions : Vec<usize>
}


struct Type{
    identifier: &'static str,
    signed : bool,
    size: usize
}


impl Type{

    fn from_ritual(ritual : Ritual) -> Type{
        let mut size : usize = 0;
        for t in ritual.content{
            let mut s = 0;
            if(!t.par_type.pointer){
                s = t.par_type.base_type.size;
                for d in t.par_type.dimensions{
                    s *= d; 
                }
            }else{
                s = mem::size_of::<usize>();
            }
            size += s;
        }
        Type { 
            identifier: ritual.identifier, 
            signed: false, 
            size
        }
    }

    fn float() -> Type{
        Type{ identifier : "f",
        signed: true,
        size : 4
        }
    }

    fn fixed() -> Type{
        Type{ identifier : "fx",
        signed: true,
        size : 4,
        }
    }

    fn int() -> Type{
        Type{ identifier : "i",
        signed: true,
        size : 4,
        }
    }

    fn char() -> Type{
        Type{ identifier : "c",
        signed: true,
        size : 1
        }
    }

    fn bool() -> Type{
        Type{ identifier : "b",
        signed: false,
        size : 1
        }
    }

    fn str(str_size : usize) -> Type{
        Type{ identifier : "s",
        signed: false,
        size : str_size + 1,
        }
    }
} 

pub struct File{
    identifier : &'static str,
    file_type : FileType,
    spells : Vec<Spell>,
    attachments : Vec<Attachment>,
    content : Vec<Box<dyn Expr>>
}

pub struct Attachment{
    identifier : &'static str,
    file_name : &'static str,
    file_type : FileType,
    context : i8
}


pub struct Spell{
    identifier : &'static str,
    rtn_type : TypeDec,
    privacy : Privacy,
    target : Target,
    context  : i8,
    pars : Vec<Par>,
    content : Vec<Box<dyn Expr>>
}


pub struct Par{
    identifier :&'static str,
    par_type : TypeDec,
}

pub struct Ritual{
    identifier :&'static str,
    content : Vec<Par>
}

impl Expr for Ritual{}
