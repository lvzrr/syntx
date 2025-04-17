use crate::engine::normalize::normalize;
use crate::engine::states::State;
use crate::langs::syntx::syntx_tokenset::*;
use crate::tokens::token_traits::{Lexable, Delimeted };
use std::borrow::Cow;

impl Lexable for syntxTokenSet {
    type Token = syntxToken;
    fn infer_token(s: String, state: &mut State<Self>) -> Option<Self::Token> {
        let raw = s.as_str();
        if state.in_char {
            return Some(syntxToken::Identifier(syntxIdentifier::CharLiteral(s)));
        }
        if state.in_str {
            return Some(syntxToken::Identifier(syntxIdentifier::StringLiteral(s)));
        }
        if syntxTokenSet::is_operator(s.as_bytes()) {
            return Some(match s.as_bytes() {
                b"=" => syntxToken::Operator(syntxOperator::Eq),
                _ => syntxToken::Identifier(syntxIdentifier::Unknown(normalize(&s.as_bytes(), state.brace_level))),
            });
        }
        if let Some(&first) = s.as_bytes().first() {
            if syntxTokenSet::is_delimeter(&first) {
                return Some(match s.as_bytes() {
                    b"=" => syntxToken::Delimeter(syntxDelimeter::Eq),
                    b";" => syntxToken::Delimeter(syntxDelimeter::Semicolon),
                    b"[" => syntxToken::Delimeter(syntxDelimeter::LBracket),
                    b"]" => syntxToken::Delimeter(syntxDelimeter::RBracket),
                    b"," => syntxToken::Delimeter(syntxDelimeter::Comma),
                    _ => syntxToken::Identifier(syntxIdentifier::Unknown(normalize(&s.as_bytes(), state.brace_level))),
                });
            }
        }
        match s.as_bytes() {
            b"info" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Info))); },
            b"operators" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Operators))); },
            b"line" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Line))); },
            b"name" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Name))); },
            b"delimeters" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Delimeters))); },
            b"tokens" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Tokens))); },
            b"block" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Block))); },
            b"comments" => { return Some(syntxToken::Identifier(syntxIdentifier::syntxKeyword(syntxKeyword::Comments))); },
            _ => { },
        }
        if !state.in_str && !state.in_char {
            let mut is_float = false;
            let mut all_digits = true;
            for b in raw.chars() {
                if syntxTokenSet::allowed_number_chars(&b) {
                    match b {
                        '.' | 'e' | 'E' | 'f' | 'F' => is_float = true,
                        _ => {}
                    }
                } else {
                    all_digits = false;
                    break;
                }
            }

            if is_float {
                let cleaned = if raw.contains('_') {
                    Cow::Owned(raw.replace('_', ""))
                } else {
                    Cow::Borrowed(raw)
                };
                if let Ok(f) = cleaned.parse::<f64>() {
                    return Some(syntxToken::Identifier(syntxIdentifier::Float(f)));
                }
            } else if all_digits {
                return Some(syntxToken::Identifier(syntxIdentifier::Integer(s, syntxBase::Decimal)));
            }

            if raw.starts_with("0x") {
                return Some(syntxToken::Identifier(syntxIdentifier::Integer(s, syntxBase::Hexadecimal)));
            } else if raw.starts_with("0b") {
                return Some(syntxToken::Identifier(syntxIdentifier::Integer(s, syntxBase::Binary)));
            } else if raw.starts_with("0o") {
                return Some(syntxToken::Identifier(syntxIdentifier::Integer(s, syntxBase::Octal)));
            }
        }        Some(syntxToken::Identifier(syntxIdentifier::Unknown(normalize(&s.as_bytes(), state.brace_level))))
    }
}
