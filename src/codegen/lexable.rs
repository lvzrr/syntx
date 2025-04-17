use crate::codegen::syntx::Syntx;
use std::fs::File;
use std::io::Write;

pub fn infer_codegen(stx: Syntx) {
    let mut f = File::create(format!("langs/{0}/{0}_inference.rs", stx.name)).unwrap();
    write!(
        f,
        "use crate::engine::normalize::normalize;
use crate::engine::states::State;
use crate::langs::{0}::{0}_tokenset::*;
use crate::tokens::token_traits::{{Lexable, Delimeted }};
use std::borrow::Cow;\n\n",
        stx.name
    )
    .unwrap();

    write!(
        f,
        "impl Lexable for {0}TokenSet {{
    type Token = {0}Token;
    fn infer_token(s: String, state: &mut State<Self>) -> Option<Self::Token> {{
        let raw = s.as_str();
        if state.in_char {{
            return Some({0}Token::Identifier({0}Identifier::CharLiteral(s)));
        }}
        if state.in_str {{
            return Some({0}Token::Identifier({0}Identifier::StringLiteral(s)));
        }}\n",
        stx.name
    )
    .unwrap();

    write!(
        f,
        "        if {0}TokenSet::is_operator(s.as_bytes()) {{
            return Some(match s.as_bytes() {{\n",
        stx.name
    )
    .unwrap();
    for op in stx.operators.iter() {
        writeln!(
            f,
            "                b\"{0}\" => {2}Token::Operator({2}Operator::{1}),",
            stx.tokens.get(op).unwrap(),
            op,
            stx.name
        )
        .unwrap();
    }
    writeln!(
        f,
        "                _ => {0}Token::Identifier({0}Identifier::Unknown(normalize(&s.as_bytes(), state.brace_level))),
            }});
        }}",
        stx.name
    ).unwrap();

    write!(
        f,
        "        if let Some(&first) = s.as_bytes().first() {{
            if {0}TokenSet::is_delimeter(&first) {{
                return Some(match s.as_bytes() {{\n",
        stx.name
    )
    .unwrap();
    for del in stx.delimiters.iter() {
        writeln!(
            f,
            "                    b\"{0}\" => {2}Token::Delimeter({2}Delimeter::{1}),",
            stx.tokens.get(del).unwrap(),
            del,
            stx.name
        )
        .unwrap();
    }
    writeln!(
        f,
        "                    _ => {0}Token::Identifier({0}Identifier::Unknown(normalize(&s.as_bytes(), state.brace_level))),
                }});
            }}
        }}",
        stx.name
    ).unwrap();

    writeln!(f, "        match s.as_bytes() {{").unwrap();
    for (kw, literal) in stx.keywords.iter() {
        writeln!(
            f,
            "            b\"{0}\" => {{ return Some({2}Token::Identifier({2}Identifier::{2}Keyword({2}Keyword::{1}))); }},",
            literal,
            kw,
            stx.name
        ).unwrap();
    }
    writeln!(f, "            _ => {{ }},").unwrap();
    writeln!(f, "        }}").unwrap();

    write!(
        f,
        "        if !state.in_str && !state.in_char {{
            let mut is_float = false;
            let mut all_digits = true;
            for b in raw.chars() {{
                if {0}TokenSet::allowed_number_chars(&b) {{
                    match b {{
                        '.' | 'e' | 'E' | 'f' | 'F' => is_float = true,
                        _ => {{}}
                    }}
                }} else {{
                    all_digits = false;
                    break;
                }}
            }}

            if is_float {{
                let cleaned = if raw.contains('_') {{
                    Cow::Owned(raw.replace('_', \"\"))
                }} else {{
                    Cow::Borrowed(raw)
                }};
                if let Ok(f) = cleaned.parse::<f64>() {{
                    return Some({0}Token::Identifier({0}Identifier::Float(f)));
                }}
            }} else if all_digits {{
                return Some({0}Token::Identifier({0}Identifier::Integer(s, {0}Base::Decimal)));
            }}

            if raw.starts_with(\"0x\") {{
                return Some({0}Token::Identifier({0}Identifier::Integer(s, {0}Base::Hexadecimal)));
            }} else if raw.starts_with(\"0b\") {{
                return Some({0}Token::Identifier({0}Identifier::Integer(s, {0}Base::Binary)));
            }} else if raw.starts_with(\"0o\") {{
                return Some({0}Token::Identifier({0}Identifier::Integer(s, {0}Base::Octal)));
            }}
        }}",
        stx.name
    )
    .unwrap();

    writeln!(
        f,
        "        Some({0}Token::Identifier({0}Identifier::Unknown(normalize(&s.as_bytes(), state.brace_level))))
    }}
}}",
        stx.name
    ).unwrap();
}
