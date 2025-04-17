use crate::codegen::syntx::Syntx;
use std::fs::File;
use std::io::Write;

pub fn enum_codegen(stx: Syntx) {
    let mut f = File::create(format!("langs/{0}/{0}_tokenset.rs", stx.name)).unwrap();

    writeln!(
        f,
        "use crate::tokens::token_traits::{{Token, TokenKind}};
#[derive(Debug, Clone, PartialEq)]
pub enum {0}Token {{
    Identifier({0}Identifier),
    Operator({0}Operator),
    Delimeter({0}Delimeter),
    EOF,
}}",
        stx.name
    )
    .unwrap();
    write!(
        f,
        "impl Token for {0}Token {{
    fn kind(&self) -> TokenKind {{
        match self {{
            {0}Token::EOF => TokenKind::Delimeter,
            {0}Token::Identifier({}Identifier::{0}Keyword(_)) => TokenKind::Keyword,
            {0}Token::Identifier({0}Identifier::StringLiteral(_)) => TokenKind::Literal,
            {0}Token::Identifier(_) => TokenKind::Identifier,
            {0}Token::Operator(_) => TokenKind::Operator,
            {0}Token::Delimeter({0}Delimeter::Whitespace) => TokenKind::Whitespace,
            {0}Token::Delimeter({0}Delimeter::NewLine) => TokenKind::Whitespace,
            {0}Token::Delimeter(_) => TokenKind::Delimeter,
        }}
    }}
}}\n
",
        stx.name
    )
    .unwrap();
    writeln!(
        f,
        "#[derive(Debug, Clone, PartialEq)]
pub enum {0}Identifier {{
    {0}Keyword({0}Keyword),
    StringLiteral(String),
    CharLiteral(String),
    Integer(String, {0}Base),
    Float(f64),
    Unknown(u64),
}}",
        stx.name
    )
    .unwrap();

    writeln!(
        f,
        "#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum {0}Operator {{",
        stx.name
    )
    .unwrap();
    for op in stx.operators.iter() {
        writeln!(f, "    {},", op).unwrap();
    }
    writeln!(f, "}}").unwrap();

    writeln!(
        f,
        "#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum {0}Delimeter {{",
        stx.name
    )
    .unwrap();
    for del in stx.delimiters.iter() {
        writeln!(f, "    {},", del).unwrap();
    }
    writeln!(
        f,
        "   Whitespace,
    NewLine,
}}"
    )
    .unwrap();

    writeln!(
        f,
        "#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum {0}Keyword {{",
        stx.name
    )
    .unwrap();
    for kw in stx.keywords.keys() {
        writeln!(f, "    {},", kw).unwrap();
    }
    writeln!(f, "}}").unwrap();

    writeln!(
        f,
        "#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum {0}Base {{
    Decimal,
    Hexadecimal,
    Binary,
    Octal,
}}",
        stx.name
    )
    .unwrap();

    writeln!(
        f,
        "#[derive(Hash, Debug, Clone, PartialEq, Eq, Default)]
pub struct {0}TokenSet;",
        stx.name
    )
    .unwrap();
}
