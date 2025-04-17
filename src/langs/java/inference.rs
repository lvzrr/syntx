use crate::engine::normalize::normalize;
use crate::engine::states::State;
use crate::langs::java::tokenset::*;
use crate::tokens::token_traits::{Delimeted, Lexable};
use std::borrow::Cow;

impl Lexable for JavaTokenSet {
    type Token = JavaToken;
    fn infer_token(s: String, state: &mut State<Self>) -> Option<Self::Token> {
        let raw = s.as_str();
        if state.in_char {
            return Some(JavaToken::Identifier(JavaIdentifier::CharLiteral(s)));
        }
        if state.in_str {
            return Some(JavaToken::Identifier(JavaIdentifier::StringLiteral(s)));
        }

        if JavaTokenSet::is_operator(s.as_bytes()) {
            return Some(match s.as_bytes() {
                b"!" => JavaToken::Operator(JavaOperator::Not),
                b"." => JavaToken::Operator(JavaOperator::Dot),
                b"@" => JavaToken::Operator(JavaOperator::At),
                b">" => JavaToken::Operator(JavaOperator::Gt),
                b">=" => JavaToken::Operator(JavaOperator::Geq),
                b"<" => JavaToken::Operator(JavaOperator::Lt),
                b"<=" => JavaToken::Operator(JavaOperator::Leq),
                b">>" => JavaToken::Operator(JavaOperator::BitShiftRight),
                b">>=" => JavaToken::Operator(JavaOperator::BitShiftRightEq),
                b">>>" => JavaToken::Operator(JavaOperator::UBitShiftRight),
                b">>>=" => JavaToken::Operator(JavaOperator::UBitShiftRightEq),
                b"<<" => JavaToken::Operator(JavaOperator::BitShiftLeft),
                b"<<=" => JavaToken::Operator(JavaOperator::BitShiftLeftEq),
                b"+" => JavaToken::Operator(JavaOperator::Plus),
                b"-" => JavaToken::Operator(JavaOperator::Minus),
                b"*" => JavaToken::Operator(JavaOperator::Mul),
                b"=" => JavaToken::Operator(JavaOperator::Assign),
                b"==" => JavaToken::Operator(JavaOperator::Eq),
                b"/" => JavaToken::Operator(JavaOperator::Div),
                b"+=" => JavaToken::Operator(JavaOperator::PlusEq),
                b"-=" => JavaToken::Operator(JavaOperator::MinusEq),
                b"*=" => JavaToken::Operator(JavaOperator::MulEq),
                b"/=" => JavaToken::Operator(JavaOperator::DivEq),
                b"%" => JavaToken::Operator(JavaOperator::Mod),
                b"%=" => JavaToken::Operator(JavaOperator::ModEq),
                b"++" => JavaToken::Operator(JavaOperator::Increment),
                b"--" => JavaToken::Operator(JavaOperator::Decrement),
                b"&" => JavaToken::Operator(JavaOperator::BitAnd),
                b"&=" => JavaToken::Operator(JavaOperator::AndEq),
                b"^" => JavaToken::Operator(JavaOperator::BitXor),
                b"^=" => JavaToken::Operator(JavaOperator::XorEq),
                b"~" => JavaToken::Operator(JavaOperator::BitCompl),
                b"|" => JavaToken::Operator(JavaOperator::BitOr),
                b"|=" => JavaToken::Operator(JavaOperator::OrEq),
                b"!=" => JavaToken::Operator(JavaOperator::Neq),
                b"&&" => JavaToken::Operator(JavaOperator::And),
                b"||" => JavaToken::Operator(JavaOperator::Or),
                b"?" => JavaToken::Operator(JavaOperator::Qmark),
                b"instanceof" => JavaToken::Operator(JavaOperator::Instanceof),
                _ => JavaToken::Identifier(JavaIdentifier::Unknown(normalize(
                    &s.as_bytes(),
                    state.brace_level,
                ))),
            });
        }

        if let Some(&first) = s.as_bytes().first() {
            if JavaTokenSet::is_delimeter(&first) {
                return Some(match s.as_bytes() {
                    b":" => JavaToken::Delimeter(JavaDelimeters::Colon),
                    b"," => JavaToken::Delimeter(JavaDelimeters::Comma),
                    b";" => JavaToken::Delimeter(JavaDelimeters::Semicolon),
                    b")" => JavaToken::Delimeter(JavaDelimeters::Rparen),
                    b"(" => JavaToken::Delimeter(JavaDelimeters::LParen),
                    b"{" => JavaToken::Delimeter(JavaDelimeters::LBrace),
                    b"}" => JavaToken::Delimeter(JavaDelimeters::RBrace),
                    b"[" => JavaToken::Delimeter(JavaDelimeters::LBracket),
                    b"]" => JavaToken::Delimeter(JavaDelimeters::RBracket),
                    b" " => JavaToken::Delimeter(JavaDelimeters::Whitespace),
                    b"\t" => JavaToken::Delimeter(JavaDelimeters::Tab),
                    b"\n" => JavaToken::Delimeter(JavaDelimeters::NewLine),
                    _ => JavaToken::Identifier(JavaIdentifier::Unknown(normalize(
                        &s.as_bytes(),
                        state.brace_level,
                    ))),
                });
            }
        }
        match s.as_bytes() {
            b"boolean" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Boolean,
                )));
            }
            b"byte" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Byte,
                )));
            }
            b"char" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Char,
                )));
            }
            b"short" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Short,
                )));
            }
            b"int" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Int,
                )));
            }
            b"long" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Long,
                )));
            }
            b"float" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Float,
                )));
            }
            b"double" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Double,
                )));
            }
            b"void" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Void,
                )));
            }
            b"String" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Class,
                )));
            }
            b"class" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Class,
                )));
            }
            b"interface" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Interface,
                )));
            }
            b"enum" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Enum,
                )));
            }
            b"abstract" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Abstract,
                )));
            }
            b"continue" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Continue,
                )));
            }
            b"for" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::For,
                )));
            }
            b"new" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::New,
                )));
            }
            b"switch" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Switch,
                )));
            }
            b"assert" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Assert,
                )));
            }
            b"default" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Default,
                )));
            }
            b"goto" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Goto,
                )));
            }
            b"package" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Package,
                )));
            }
            b"synchronized" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Synchronized,
                )));
            }
            b"do" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Do,
                )));
            }
            b"if" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::If,
                )));
            }
            b"private" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Private,
                )));
            }
            b"this" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::This,
                )));
            }
            b"break" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Break,
                )));
            }
            b"implements" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Implements,
                )));
            }
            b"protected" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Protected,
                )));
            }
            b"throw" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Throw,
                )));
            }
            b"else" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Else,
                )));
            }
            b"import" => {
                state.read_include = true;
                return None;
            }
            b"public" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Public,
                )));
            }
            b"throws" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Throws,
                )));
            }
            b"case" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Case,
                )));
            }
            b"instanceof" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Instanceof,
                )));
            }
            b"return" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Return,
                )));
            }
            b"transient" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Transient,
                )));
            }
            b"catch" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Catch,
                )));
            }
            b"extends" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Extends,
                )));
            }
            b"try" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Try,
                )));
            }
            b"final" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Final,
                )));
            }
            b"static" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Static,
                )));
            }
            b"finally" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Finally,
                )));
            }
            b"strictfp" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Strictfp,
                )));
            }
            b"volatile" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Volatile,
                )));
            }
            b"const" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Const,
                )));
            }
            b"native" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Native,
                )));
            }
            b"super" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::Super,
                )));
            }
            b"while" => {
                return Some(JavaToken::Identifier(JavaIdentifier::JavaKeyword(
                    JavaKeyword::While,
                )));
            }
            _ => {} // fall through
        }
        if !state.in_str && !state.in_char {
            let mut is_float = false;
            let mut all_digits = true;

            for b in raw.as_bytes() {
                match b {
                    b'0'..=b'9' => {}
                    b'.' | b'e' | b'E' => is_float = true,
                    b'_' => {}
                    _ => {
                        all_digits = false;
                        break;
                    }
                }
            }

            if is_float {
                let cleaned = if raw.contains('_') {
                    Cow::Owned(raw.replace('_', ""))
                } else {
                    Cow::Borrowed(raw)
                };

                if let Ok(f) = cleaned.parse::<f64>() {
                    return Some(JavaToken::Identifier(JavaIdentifier::Float(f)));
                }
            } else if all_digits {
                return Some(JavaToken::Identifier(JavaIdentifier::Integer(
                    s,
                    JavaBase::Decimal,
                )));
            }

            if raw.starts_with("0x") {
                return Some(JavaToken::Identifier(JavaIdentifier::Integer(
                    s,
                    JavaBase::Hexadecimal,
                )));
            } else if raw.starts_with("0b") {
                return Some(JavaToken::Identifier(JavaIdentifier::Integer(
                    s,
                    JavaBase::Octal,
                )));
            } else if raw.starts_with("0o") {
                return Some(JavaToken::Identifier(JavaIdentifier::Integer(
                    s,
                    JavaBase::Binary,
                )));
            }
        }
        Some(JavaToken::Identifier(JavaIdentifier::Unknown(normalize(
            &s.as_bytes(),
            state.brace_level,
        ))))
    }
}
