///! State Machine Fields (engine/states.rs)
///!
///! This file contains the struct definition for the state machine, as well as the defaults for
///! when we havent started to lex yet.
use crate::{
    engine::errors::LexicalError,
    tokens::token_traits::{Delimeted, Lexable},
};

#[derive(Debug)]
pub struct State<T: Delimeted + Lexable> {
    pub valid: bool,
    pub row: usize,
    pub column: usize,
    pub in_str: bool,
    pub in_char: bool,
    pub in_brace: bool,
    pub in_paren: bool,
    pub scape_next: bool,
    pub brace_level: usize,
    pub generic_level: usize,
    pub paren_level: usize,
    pub read_include: bool,
    pub stacktrace: Option<Vec<LexicalError<T>>>,
}

impl<'a, T: Delimeted + Lexable> Default for State<T> {
    fn default() -> Self {
        Self {
            valid: true,
            row: 0,
            column: 0,
            in_str: false,
            in_char: false,
            in_brace: false,
            in_paren: false,
            scape_next: false,
            brace_level: 0,
            paren_level: 0,
            generic_level: 0,
            read_include: false,
            stacktrace: Some(Vec::new()),
        }
    }
}
