use std::fs::File;
use std::io::Write;

use crate::codegen::syntx::*;

pub fn delimeted_codegen(stx: Syntx) {
    let mut f = File::create(format!("langs/{0}/{0}_delimeted.rs", stx.name).as_str()).unwrap();
    write!(
        f,
        "use crate::langs::{0}::{0}_tokenset::*;
use crate::tokens::token_traits::Delimeted;\n
impl Delimeted for {}TokenSet {{
    #[inline(always)]
    fn allowed_unicode_char(c: &char) -> Option<usize> {{
        match c {{
            'u' => Some(4),
            'U' => Some(8),
            _ => None,
        }}
    }}\n\n",
        stx.name
    )
    .unwrap();
    write!(
        f,
        "\t#[inline(always)]
    fn is_scape(c: &char) -> Option<char> {{
        match c {{\n"
    )
    .unwrap();
    for (escape_char, real_char) in &stx.scapes {
        write!(f, "\t\t\t'{}' => Some('{}'),\n", escape_char, real_char).unwrap();
    }
    write!(f, "\t\t\t_ => None\n\t\t}}\n").unwrap();
    write!(f, "\t}}\n\n").unwrap();
    write!(
        f,
        "\t#[inline(always)]
    fn may_trigger_line_comment(c: char) -> Option<usize> {{
        if c == '{}' {{ Some({}) }} else {{ None }}
    }}\n
",
        stx.comments[0][0] as char,
        stx.comments[0].len(),
    )
    .unwrap();

    write!(
        f,
        "\t#[inline(always)]
    fn trigger_comment_line(input: &[char]) -> bool {{
        input == ['{}', '{}']
    }}
",
        stx.comments[0][0] as char, stx.comments[0][1] as char,
    )
    .unwrap();
    write!(
        f,
        "
    #[inline(always)]
    fn may_trigger_block_comment(c: char) -> Option<(Vec<char>, usize)> {{
        if c == '{}' {{
            Some((vec!['{}', '{}'], 2))
        }} else {{
            None
        }}
    }}\n\n",
        stx.comments[1][0] as char, stx.comments[1][0] as char, stx.comments[1][1] as char,
    )
    .unwrap();
    write!(
        f,
        "\t#[inline(always)]
    fn is_delimeter(c: &u8) -> bool {{
        matches!(
            c,\n"
    )
    .unwrap();
    let first = stx.delimiters.first().unwrap();
    writeln!(f, "\t\t\tb'{}'", stx.tokens.get(first).unwrap()).unwrap();

    for del in stx.delimiters.iter().skip(1) {
        writeln!(f, "\t\t\t| b'{}'", stx.tokens.get(del).unwrap()).unwrap();
    }
    write!(f, "\t\t)\n\t}}\n\n").unwrap();
    write!(
        f,
        "\t#[inline(always)]
    fn trigger_comment_block(input: &[char]) -> bool {{
        input == ['{}', '{}']
    }}\n\n",
        stx.comments[1][0] as char, stx.comments[1][1] as char
    )
    .unwrap();
    write!(
        f,
        "\t#[inline(always)]
    fn is_operator(s: &[u8]) -> bool {{
        matches!(
            s,\n"
    )
    .unwrap();
    let op = stx.operators.first().unwrap();
    writeln!(f, "\t\t\tb\"{}\"", stx.tokens.get(op).unwrap()).unwrap();
    for op in stx.operators[1..].iter() {
        writeln!(f, "\t\t\t | b\"{}\"", stx.tokens.get(op).unwrap()).unwrap();
    }
    write!(f, "\t\t)\n\t}}\n\n").unwrap();
    write!(
        f,
        "\t#[inline(always)]
    fn allowed_number_chars(c: &char) -> bool {{
        c.is_ascii_alphanumeric() || matches!(c,"
    )
    .unwrap();
    let num = stx.numbers.first().unwrap();
    writeln!(f, " '{}'", num).unwrap();
    for num in stx.numbers[1..].iter() {
        writeln!(f, "\t\t\t | '{}'", num).unwrap();
    }
    write!(f, "\t\t)\n\t}}\n").unwrap();
    write!(f, "}}").unwrap();
}
