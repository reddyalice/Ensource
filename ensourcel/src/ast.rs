use std::boxed::Box;
use std::mem;


pub const byeSize : u8 = 8;
pub const memSize : usize = mem::size_of::<usize>();

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
    None
}

pub enum Mal{
    Entropic, 
    Crystal
}

pub struct TypeDec {
    pub base_type: Type,
    pub pointer: bool,
    pub dimensions: Vec<usize>,
}

pub struct Type {
    pub identifier: String,
    pub signed: bool,
    pub size: usize,
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

    fn float() -> Type {
        Type {
            identifier: String::from("f"),
            signed: true,
            size: 4,
        }
    }

    fn fixed() -> Type {
        Type {
            identifier: String::from("fx"),
            signed: true,
            size: 4,
        }
    }

    fn int() -> Type {
        Type {
            identifier: String::from("i"),
            signed: true,
            size: 4,
        }
    }

    fn char() -> Type {
        Type {
            identifier: String::from("c"),
            signed: true,
            size: 1,
        }
    }

    fn bool() -> Type {
        Type {
            identifier: String::from("b"),
            signed: false,
            size: 1,
        }
    }

    fn str(str_size: usize) -> Type {
        Type {
            identifier: String::from("s"),
            signed: false,
            size: str_size + 1,
        }
    }

    fn get_void() -> Type {
        Type {
            identifier: String::from("void"),
            signed: false,
            size: 0,
        }
    }
}

pub struct File {
    pub identifier: String,
    pub file_type: FileType,
    pub spells: Vec<Spell>,
    pub attachments: Vec<Attachment>,
    pub rituals: Vec<Ritual>,
    pub content: Vec<Expr>,
}

pub struct Attachment {
    pub identifier: String,
    pub file_name: String,
    pub file_type: FileType,
    pub context: usize,
}

pub struct Spell {
    pub identifier: String,
    pub rtn_type: TypeDec,
    pub privacy: Privacy,
    pub target: Target,
    pub context: usize,
    pub pars: Vec<Par>,
    pub content: Vec<Expr>,
}

pub struct Par {
    pub identifier: String,
    pub par_type: TypeDec,
}


pub struct Ritual {
    pub identifier: String,
    pub content: Vec<Par>,
}

pub struct ConditionalCase {
    pub condition: Expr,
    pub true_case: Vec<Expr>,
    pub false_case: Vec<Expr>,
}

pub struct ChannelWhile {
    pub condition: Expr,
    pub context: Vec<Expr>,
}

pub struct ChannelStandartFor {
    pub identifier: String,
    pub condition: Expr,
    pub increment: Expr,
    pub context: Vec<Expr>,
}

pub struct ChannelListFor {
    pub identifier: Expr,
    pub list: Expr,
    pub context: Vec<Expr>,
}

pub struct Lambda {
    pub rtn_type: TypeDec,
    pub pars: Vec<Par>,
    pub context: Vec<Expr>
}

pub struct Cast{
    pub args : Vec<Expr>,
    pub cast : Expr
}

pub struct Sigil{
    pub privacy: Privacy,
    pub target: Target,
    pub mal : Mal,
    pub pars : Vec<Par>,
    pub args : Vec<Expr>
}

pub struct Print{
    pub output : String,
    pub args : Vec<Expr>
}

pub struct BinaryOp{
    pub left : Expr,
    pub operation : BinaryOperations,
    pub right : Expr
}


pub struct UnaryOp{
    pub operation : UnaryOps,
    pub exp : Expr
}

pub struct Transmute{
    pub old_type : TypeDec,
    pub expr : Expr,
    pub new_type : TypeDec
}

pub struct Index{
    pub expr : Expr,
    pub index : Expr
}

pub struct Wait{
    pub expr : Expr
}

pub struct Stop{
    pub condition : Expr
}

pub struct Skip{
    pub condition : Expr,
    pub time : usize
}

pub struct Expr{
    context : usize,
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
    Identifier(String),
    String(String),
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