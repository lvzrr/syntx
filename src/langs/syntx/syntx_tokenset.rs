use crate::tokens::token_traits::{Token, TokenKind};
#[derive(Debug, Clone, PartialEq)]
pub enum syntxToken {
    Identifier(syntxIdentifier),
    Operator(syntxOperator),
    Delimeter(syntxDelimeter),
    EOF,
}
impl Token for syntxToken {
    fn kind(&self) -> TokenKind {
        match self {
            syntxToken::EOF => TokenKind::Delimeter,
            syntxToken::Identifier(syntxIdentifier::syntxKeyword(_)) => TokenKind::Keyword,
            syntxToken::Identifier(syntxIdentifier::StringLiteral(_)) => TokenKind::Literal,
            syntxToken::Identifier(_) => TokenKind::Identifier,
            syntxToken::Operator(_) => TokenKind::Operator,
            syntxToken::Delimeter(syntxDelimeter::Whitespace) => TokenKind::Whitespace,
            syntxToken::Delimeter(syntxDelimeter::NewLine) => TokenKind::Whitespace,
            syntxToken::Delimeter(_) => TokenKind::Delimeter,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum syntxIdentifier {
    syntxKeyword(syntxKeyword),
    StringLiteral(String),
    CharLiteral(String),
    Integer(String, syntxBase),
    Float(f64),
    Unknown(u64),
}
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum syntxOperator {
    Eq,
}
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum syntxDelimeter {
    Eq,
    Semicolon,
    LBracket,
    RBracket,
    Comma,
    Whitespace,
    NewLine,
}
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum syntxKeyword {
    Info,
    Operators,
    Line,
    Name,
    Delimeters,
    Tokens,
    Block,
    Comments,
    Grammar,
}
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum syntxBase {
    Decimal,
    Hexadecimal,
    Binary,
    Octal,
}
#[derive(Hash, Debug, Clone, PartialEq, Eq, Default)]
pub struct syntxTokenSet;
