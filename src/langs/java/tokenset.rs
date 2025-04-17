use crate::tokens::token_traits::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum JavaToken {
    Identifier(JavaIdentifier),
    Operator(JavaOperator),
    Delimeter(JavaDelimeters),
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct JavaTokenSet;

impl Token for JavaToken {
    fn kind(&self) -> TokenKind {
        match self {
            JavaToken::EOF => TokenKind::Delimeter,
            JavaToken::Identifier(JavaIdentifier::Var(_, _)) => TokenKind::Identifier,
            JavaToken::Identifier(JavaIdentifier::JavaKeyword(_)) => TokenKind::Keyword,
            JavaToken::Identifier(JavaIdentifier::CharLiteral(_)) => TokenKind::Literal,
            JavaToken::Identifier(JavaIdentifier::StringLiteral(_)) => TokenKind::Literal,
            JavaToken::Identifier(JavaIdentifier::Unknown(_)) => TokenKind::Unknown,
            JavaToken::Identifier(_) => TokenKind::Identifier,
            JavaToken::Operator(_) => TokenKind::Operator,
            JavaToken::Delimeter(JavaDelimeters::Whitespace) => TokenKind::Whitespace,
            JavaToken::Delimeter(JavaDelimeters::NewLine) => TokenKind::Whitespace,
            JavaToken::Delimeter(_) => TokenKind::Delimeter,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum JavaDelimeters {
    LParen,
    Rparen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    Whitespace,
    Tab,
    NewLine,
}
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JavaBase {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}
#[derive(Debug, Clone, PartialEq)]
pub enum JavaIdentifier {
    Var(u64, JavaKeyword),
    ObjVar(u64, u64),
    StringLiteral(String),
    CharLiteral(String),
    JavaKeyword(JavaKeyword),
    Integer(String, JavaBase),
    Float(f64),
    Unknown(u64),
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JavaOperator {
    Dot,
    At,
    Qmark,
    Assign,
    Eq,
    Not,
    Neq,
    Geq,
    Leq,
    Gt,
    Lt,
    Plus,
    Minus,
    PlusEq,
    MinusEq,
    Div,
    Mod,
    Mul,
    DivEq,
    ModEq,
    MulEq,
    BitShiftLeft,
    BitShiftRight,
    UBitShiftRight,
    BitShiftLeftEq,
    BitShiftRightEq,
    UBitShiftRightEq,
    Increment,
    Decrement,
    And,
    Or,
    AndEq,
    OrEq,
    XorEq,
    BitAnd,
    BitOr,
    BitXor,
    BitAndEq,
    BitOrEq,
    BitXorEq,
    BitCompl,
    Instanceof,
}
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum JavaKeyword {
    Abstract,
    Continue,
    For,
    New,
    Switch,
    Assert,
    Default,
    Goto,
    Package,
    Synchronized,
    Boolean,
    Do,
    If,
    Private,
    This,
    Break,
    Double,
    Implements,
    Protected,
    Throw,
    Byte,
    Else,
    Import,
    Public,
    Throws,
    Case,
    Enum,
    Instanceof,
    Return,
    Transient,
    Catch,
    Extends,
    Int,
    Short,
    Try,
    Char,
    Final,
    Interface,
    Static,
    Void,
    Class,
    Finally,
    Long,
    Strictfp,
    Volatile,
    Const,
    Float,
    Native,
    Super,
    While,
}
