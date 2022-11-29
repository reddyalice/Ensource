use std::boxed::Box;
use std::mem;


pub const byeSize : usize = 8;
pub const memSize : usize = mem::size_of::<usize>();

#[derive(Clone, Debug)]
pub enum FileType {
    Necr,
    Sorc,
    Wiza,
    Hexy,
}

#[derive(Clone, Debug)]
pub enum Privacy {
    Forall,
    Mine,
}

#[derive(Clone, Debug)]
pub enum Target {
    Golem,
    Whisper,
    None
}

#[derive(Clone, Debug)]
pub enum Mal{
    Entropic, 
    Crystal
}

#[derive(Clone, Debug)]
pub struct TypeDec {
    pub base_type: Type,
    pub pointer: bool,
    pub dimensions: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub identifier: String,
    pub signed: bool,
    pub size: usize,
}

impl Type {


    pub fn from_primitive(primitive : &str, size: usize) -> Type{
        match primitive {
            "bool" | "b" => Type::bool(size),
            "int"  | "i" => Type::int(size),
            "float" | "f" => Type::float(size),
            "fixed" | "fx" => Type::fixed(size),
            "uint"  | "ui" => Type::uint(size),
            "ufloat" | "uf" => Type::ufloat(size),
            "ufixed" | "ufx" => Type::ufixed(size),
            _ => panic!("No prim")
        }
    }

    pub fn from_cprimitive(primitive : &str) -> Type{
        match primitive {
            "bool" | "b" => Type::bool(1),
            "int"  | "i" => Type::int(4),
            "float" | "f" => Type::float(4),
            "fixed" | "fx" => Type::fixed(4),
            "uint"  | "ui" => Type::uint(4),
            "ufloat" | "uf" => Type::ufloat(4),
            "ufixed" | "ufx" => Type::ufixed(4),
            _ => panic!("No prim")
        }
    }


    pub fn from_ritual(ritual: Ritual) -> Type {
        let mut size: usize = 0;
        for t in ritual.content {
            let mut s;
            if !t.par_type.pointer {
                s = t.par_type.base_type.size;
                for d in t.par_type.dimensions {
                    s *= d;
                }
            } else {
                s = memSize;
            }
            size += s;
        }
        Type {
            identifier: ritual.identifier,
            signed: false,
            size,
        }
    }

    pub fn float(size : usize) -> Type {
        Type {
            identifier: String::from("f"),
            signed: true,
            size,
        }
    }

    pub fn fixed(size : usize) -> Type {
        Type {
            identifier: String::from("fx"),
            signed: true,
            size,
        }
    }

    pub fn int(size : usize) -> Type {
        Type {
            identifier: String::from("i"),
            signed: true,
            size,
        }
    }

    pub fn ufloat(size : usize) -> Type {
        Type {
            identifier: String::from("uf"),
            signed: false,
            size,
        }
    }

    pub fn ufixed(size : usize) -> Type {
        Type {
            identifier: String::from("ufx"),
            signed: false,
            size,
        }
    }

    pub fn uint(size : usize) -> Type {
        Type {
            identifier: String::from("ui"),
            signed: false,
            size,
        }
    }

    pub fn char() -> Type {
        Type {
            identifier: String::from("c"),
            signed: true,
            size: 1,
        }
    }

    pub fn bool(size : usize) -> Type {
        Type {
            identifier: String::from("b"),
            signed: false,
            size,
        }
    }

    pub fn str(str_size : usize) -> Type {
        Type {
            identifier: String::from("s"),
            signed: false,
            size: str_size + 1,
        }
    }

    pub fn get_void() -> Type {
        Type {
            identifier: String::from("void"),
            signed: false,
            size: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct File {
    pub identifier: String,
    pub file_type: FileType,
    pub spells: Vec<Spell>,
    pub attachments: Vec<Attachment>,
    pub rituals: Vec<Ritual>,
    pub content: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct Attachment {
    pub identifier: String,
    pub file_name: String,
    pub file_type: FileType,
    pub context: usize,
}

#[derive(Clone, Debug)]
pub struct Spell {
    pub identifier: String,
    pub rtn_type: TypeDec,
    pub privacy: Privacy,
    pub target: Target,
    pub context: usize,
    pub pars: Vec<Par>,
    pub content: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct Par {
    pub identifier: String,
    pub par_type: TypeDec,
}

#[derive(Clone, Debug)]
pub struct Ritual {
    pub privacy: Privacy,
    pub identifier: String,
    pub content: Vec<Par>,
}

#[derive(Clone, Debug)]
pub struct ConditionalCase {
    pub condition: Expr,
    pub true_case: Vec<Expr>,
    pub false_case: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct ChannelWhile {
    pub condition: Expr,
    pub context: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct ChannelStandartFor {
    pub identifier: String,
    pub condition: Expr,
    pub increment: Expr,
    pub context: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct ChannelListFor {
    pub identifier: Expr,
    pub list: Expr,
    pub context: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct Lambda {
    pub rtn_type: TypeDec,
    pub pars: Vec<Par>,
    pub context: Vec<Expr>
}

#[derive(Clone, Debug)]
pub struct Cast{
    pub args : Vec<Expr>,
    pub cast : Expr
}

#[derive(Clone, Debug)]
pub struct Sigil{
    pub privacy: Privacy,
    pub target: Target,
    pub mal : Mal,
    pub pars : Vec<Par>,
    pub args : Vec<Expr>
}

#[derive(Clone, Debug)]
pub struct Print{
    pub output : String,
    pub args : Vec<Expr>
}

#[derive(Clone, Debug)]
pub struct BinaryOp{
    pub left : Expr,
    pub operation : BinaryOperations,
    pub right : Expr
}

#[derive(Clone, Debug)]
pub struct UnaryOp{
    pub operation : UnaryOps,
    pub exp : Expr
}

#[derive(Clone, Debug)]
pub struct Transmute{
    pub old_type : TypeDec,
    pub expr : Expr,
    pub new_type : TypeDec
}

#[derive(Clone, Debug)]
pub struct Index{
    pub expr : Expr,
    pub index : Expr
}

#[derive(Clone, Debug)]
pub struct Wait{
    pub expr : Expr
}

#[derive(Clone, Debug)]
pub struct Stop{
    pub condition : Expr
}

#[derive(Clone, Debug)]
pub struct Skip{
    pub condition : Expr,
    pub time : usize
}

#[derive(Clone, Debug)]
pub struct Expr{
    context : usize,
    expr_type : TypeDec,
    exp : ExprType
}


#[derive(Clone, Debug)]
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
    Identifier(String),
    String(String),
    Bool(bool),
    Char(char),
    Float(f32),
    Fixed(i32),
    Integer(i32),
    None,
}

#[derive(Clone, Debug)]
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


#[derive(Clone, Debug)]
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