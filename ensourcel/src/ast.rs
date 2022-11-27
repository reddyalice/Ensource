use std::boxed::Box;
use std::mem;

pub enum FileType {
    Necr,
    Sorc,
    Wiza,
    Hexy,
}

pub enum Privacy {
    Forall,
    Mine,
}

pub enum Target {
    Golem,
    Whisper,
}

pub enum Mal{
    Entropic, 
    Crystal
}

pub struct TypeDec {
    base_type: Type,
    pointer: bool,
    dimensions: Vec<usize>,
}

pub struct Type {
    identifier: &'static str,
    signed: bool,
    size: usize,
}

impl Type {
    fn from_ritual(ritual: Ritual) -> Type {
        let mut size: usize = 0;
        for t in ritual.content {
            let mut s;
            if !t.par_type.pointer {
                s = t.par_type.base_type.size;
                for d in t.par_type.dimensions {
                    s *= d;
                }
            } else {
                s = mem::size_of::<usize>();
            }
            size += s;
        }
        Type {
            identifier: ritual.identifier,
            signed: false,
            size,
        }
    }

    fn float() -> Type {
        Type {
            identifier: "f",
            signed: true,
            size: 4,
        }
    }

    fn fixed() -> Type {
        Type {
            identifier: "fx",
            signed: true,
            size: 4,
        }
    }

    fn int() -> Type {
        Type {
            identifier: "i",
            signed: true,
            size: 4,
        }
    }

    fn char() -> Type {
        Type {
            identifier: "c",
            signed: true,
            size: 1,
        }
    }

    fn bool() -> Type {
        Type {
            identifier: "b",
            signed: false,
            size: 1,
        }
    }

    fn str(str_size: usize) -> Type {
        Type {
            identifier: "s",
            signed: false,
            size: str_size + 1,
        }
    }

    fn get_void() -> Type {
        Type {
            identifier: "void",
            signed: false,
            size: 0,
        }
    }
}

pub struct File {
    identifier: &'static str,
    file_type: FileType,
    spells: Vec<Spell>,
    attachments: Vec<Attachment>,
    rituals: Vec<Ritual>,
    content: Vec<Expr>,
}

pub struct Attachment {
    identifier: &'static str,
    file_name: &'static str,
    file_type: FileType,
    context: i8,
}

pub struct Spell {
    identifier: &'static str,
    rtn_type: TypeDec,
    privacy: Privacy,
    target: Target,
    context: i8,
    pars: Vec<Par>,
    content: Vec<Expr>,
}

pub struct Par {
    identifier: &'static str,
    par_type: TypeDec,
}


pub struct Ritual {
    identifier: &'static str,
    content: Vec<Par>,
}

pub struct ConditionalCase {
    condition: Expr,
    true_case: Vec<Expr>,
    false_case: Vec<Expr>,
}

pub struct ChannelWhile {
    condition: Expr,
    context: Vec<Expr>,
}

pub struct ChannelStandartFor {
    identifier: &'static str,
    condition: Expr,
    increment: Expr,
    context: Vec<Expr>,
}

pub struct ChannelListFor {
    identifier: Expr,
    list: Expr,
    context: Vec<Expr>,
}

pub struct Lambda {
    rtn_type: TypeDec,
    pars: Vec<Par>,
    context: Vec<Expr>
}

pub struct Cast{
    args : Vec<Expr>,
    cast : Expr
}

pub struct Sigil{
    privacy: Privacy,
    target: Target,
    mal : Mal,
    pars : Vec<Par>,
    args : Vec<Expr>
}

pub struct Print{
    output : &'static str,
    args : Vec<Expr>
}

pub struct BinaryOp{
    left : Expr,
    operation : BinaryOperations,
    right : Expr
}


pub struct UnaryOp{
    operation : UnaryOps,
    exp : Expr
}

pub struct Transmute{
    old_type : TypeDec,
    expr : Expr,
    new_type : TypeDec
}

pub struct Index{
    expr : Expr,
    index : Expr
}

pub struct Wait{
    expr : Expr
}

pub struct Stop{
    condition : Expr
}

pub struct Skip{
    condition : Expr,
    time : usize
}

pub struct Expr{
    context : i8,
    expr_type : TypeDec,
    exp : ExprType
}



pub enum ExprType {
    ConditionalCase(Box<ConditionalCase>),
    ChannelWhile(Box<ChannelWhile>),
    ChannelStandartFor(Box<ChannelStandartFor>),
    ChannelListFor(Box<ChannelListFor>),
    Lambda(Box<Lambda>),
    Cast(Box<Cast>),
    Sigil(Box<Sigil>),
    Print(Box<Print>),
    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>),
    Transmute(Box<Transmute>),
    Index(Box<Index>),
    Wait(Box<Wait>),
    Stop(Box<Stop>),
    Skip(Box<Stop>),
    Identifier(&'static str),
    String(&'static str),
    Bool(bool),
    Char(char),
    Float(f32),
    Fixed(i32),
    Integer(i32),
    None,
}

pub enum BinaryOperations{
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    And,
    Or,
    Bigger,
    Lesser,
    Equal,
    NotEqual,
    BEqual,
    LEqual
}

pub enum UnaryOps{
    PreIncr,
    PostIncr,
    PreDecr,
    PostDecr,
    Not,
    Neg,
    Sign,
    Unsign
}