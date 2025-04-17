//! Token Traits Interface (tokens/token_traits.rs)
//!
//! Needed functions to make the engine work, also use of generics for multi-language support
use crate::engine::states::State;
use std::{char, hash::Hash};
pub trait Token: Clone + std::fmt::Debug + PartialEq {
    fn kind(&self) -> TokenKind;
    //fn to_id(&self) -> String;
}

pub trait Resolvable {
    fn resolve_import(
        name: String,
        sender: crossbeam::channel::Sender<Vec<<Self as Lexable>::Token>>,
    ) where
        Self: Lexable;
}

/// Main trait for the tokensets, defines the tokentypes in Token and holds the inference logic for
/// types and enum assignment.
pub trait Lexable {
    type Token: Clone + std::fmt::Debug + PartialEq + Token;
    fn infer_token(c: String, state: &mut State<Self>) -> Option<Self::Token>
    where
        Self: Sized + Delimeted + Lexable;
}

/// Describes the language use of delimeters, comments, operators, etc. The lexer engine uses these
/// for more dynamic token inference.
pub trait Delimeted
where
    Self: Sized + Eq + Lexable + Clone + Hash,
    Self::Token: Token,
{
    fn is_delimeter(c: &u8) -> bool;
    /// Max secuence length for inline comment
    fn may_trigger_line_comment(c: char) -> Option<usize>;
    /// Max secuence length for block comment (end secuence / usize))
    fn may_trigger_block_comment(c: char) -> Option<(Vec<char>, usize)>;
    /// Match the n chars following the comment trigger -> dispatch eater
    fn trigger_comment_line(c: &[char]) -> bool;
    /// Match the n chars following the comment trigger -> dispatch eater
    fn trigger_comment_block(c: &[char]) -> bool;
    /// returns if a string is an operator of the defined lang by the user
    fn is_operator(c: &[u8]) -> bool;
    /// checks if a char is a valid number char
    fn allowed_number_chars(c: &char) -> bool;
    /// checks if a scaped char is a valid unicode char for the lang
    fn allowed_unicode_char(c: &char) -> Option<usize>;
    /// checks for scape secuences valid per lang
    fn is_scape(c: &char) -> Option<char>;
}

/// TODO: This should be for making ASTs
pub trait Parseable {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Operator,
    Delimeter,
    Literal,
    Whitespace,
    Unknown,
}
