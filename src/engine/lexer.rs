//! Lexer Engine (engine/lexer.rs)
//!
//! This module defines the core lexer engine behind syntx, responsible for character-wise
//! tokenization of source input. It is designed to be highly flexible by relying on
//! user-defined Lexable and Delimeted traits, allowing support for a wide variety of languages.
use crate::engine::semantic_traits::*;
use crate::engine::states::*;
use crate::tokens::token_traits::*;
use crossbeam::channel::Sender;

use std::iter::Peekable;
use std::str::Chars;

const BATCH_SIZE: usize = 2048;

#[derive(Debug)]
pub struct Lexer<'a, T: Lexable + Resolvable + Delimeted + Eq + Clone> {
    pub tokens: Vec<T::Token>,
    pub contents: Peekable<Chars<'a>>,
    pub state: State<T>,
    pub sender: Sender<Vec<T::Token>>,
}

/// Implementation of lexer constructor from a &str, to avoid duplication and save memory
/// Traits needed:
///        - Lexable:   Must be declared for token inference.
///
///        - Delimeted: Must explicitly specify delimeters (both operators and others).
///                     See syntx/src/langs/java.rs for examples, this is so the engine
///                     knows when to stop eating and try to infer a type.

impl<'a, T: Lexable + Resolvable + Delimeted + Eq + Clone + Default> Lexer<'a, T>
where
    T::Token: Token,
{
    pub fn new(value: &'a str, s: Sender<Vec<T::Token>>) -> Self {
        Lexer {
            sender: s,
            tokens: Vec::with_capacity(BATCH_SIZE),
            contents: value.chars().peekable(),
            state: State::<T>::default(),
        }
    }
}

/// Implements the core tokenizer engine using the Walker trait.
/// This module consumes characters via lookahead (Peekable) and emits structured tokens.
/// The engine behavior is fully customizable through Lexable and Delimeted user-defined trait bounds.
impl<'a, T> Walker<T> for Lexer<'a, T>
where
    T: Lexable + Resolvable + Delimeted + Clone + Eq,
    T::Token: Token,
{
    /// Main logic for advancing char position in the iterator, updating state coordinates on newlines

    fn bump(&mut self, ch: char) {
        if !self.state.in_str && !self.state.in_char && ch == '\n' {
            self.state.row += 1;
            self.state.column = 0;
        } else {
            self.state.column += 1;
        }
        self.contents.next();
    }

    /// Main loop for the tokenizer, eats chars until there are no more in the iterator to consume

    fn tokenize(&mut self) {
        while let Some(&ch) = self.contents.peek() {
            // Comment detection based on greedy search and the maximal-munch principle
            // Uses fixed 4-char lookahead (max known length for comment tokens across languages).
            // Can be extended if needed for languages with longer comment markers.

            if let Some(n) = T::may_trigger_line_comment(ch) {
                let mut buf = ['\0'; 4];
                let mut len = 0;

                let mut cpy = self.contents.clone();
                for i in 0..n {
                    if let Some(&ch) = cpy.peek() {
                        buf[i] = ch;
                        cpy.next();
                        len += 1;
                    } else {
                        break;
                    }
                }
                if T::trigger_comment_line(&buf[..len]) {
                    self.eat_comment_line();
                    continue;
                }
            }

            if let Some((end_seq, n)) = T::may_trigger_block_comment(ch) {
                let mut buf = ['\0'; 4];
                let mut len = 0;

                let mut cpy = self.contents.clone();
                for i in 0..n {
                    if let Some(&ch) = cpy.peek() {
                        buf[i] = ch;
                        cpy.next();
                        len += 1;
                    } else {
                        break;
                    }
                }

                if T::trigger_comment_block(&buf[..len]) {
                    self.eat_comment_block(&end_seq);
                    continue;
                }
            }

            // State-machine updater logic and dispatcher for eaters to consume items based on a
            // condition.
            // NOTE: eat_str and eat_char handle state toggles (in_str/in_char) internally.
            // No need to mutate those flags here.

            match ch {
                '\n' => self.bump(ch),
                '"' => {
                    if !self.state.in_char {
                        self.eat_str();
                    }
                }
                '\'' => {
                    if !self.state.in_str {
                        self.eat_char();
                    }
                }
                '{' => {
                    self.state.brace_level += 1;
                    if let Some(x) = T::infer_token(ch.to_string(), &mut self.state) {
                        self.tokens.push(x);
                    }
                    self.bump(ch);
                }
                '}' => {
                    self.state.brace_level -= 1;
                    if let Some(x) = T::infer_token(ch.to_string(), &mut self.state) {
                        self.tokens.push(x);
                    }
                    self.bump(ch);
                }
                '(' => {
                    self.state.in_paren = true;
                    self.state.paren_level += 1;
                    if let Some(x) = T::infer_token(ch.to_string(), &mut self.state) {
                        self.tokens.push(x);
                    }
                    self.bump(ch);
                }
                ')' => {
                    self.state.paren_level -= 1;
                    if self.state.paren_level == 0 {
                        self.state.in_paren = false;
                    }
                    if let Some(x) = T::infer_token(ch.to_string(), &mut self.state) {
                        self.tokens.push(x);
                    }
                    self.bump(ch);
                }
                x if T::is_operator(&vec![x as u8]) => self.eat_delimeter(x),
                x if T::is_delimeter(&(x as u8)) => {
                    if !x.is_whitespace() {
                        if let Some(t) = T::infer_token(x.to_string(), &mut self.state) {
                            self.tokens.push(t);
                        }
                    }
                    self.goto_next_sequence(x);
                }
                x if self.state.read_include && x.is_alphabetic() => {
                    let name = self.eat_to_newl();
                    self.state.read_include = false;
                    self.sender.send(std::mem::take(&mut self.tokens)).unwrap();
                    T::resolve_import(name, self.sender.clone());
                }
                x if x.is_alphabetic() => self.eat_literal(),
                x if x.is_numeric() => self.eat_number(x),
                _ => {
                    self.bump(ch);
                }
            }
        }
        if self.tokens.len() >= BATCH_SIZE {
            self.sender.send(std::mem::take(&mut self.tokens)).unwrap();
        }
        self.sender.send(std::mem::take(&mut self.tokens)).unwrap();
    }

    /// Logic to eat a generic number, uses the traits in 'Delimeted' to stop eating,
    /// allowing for differnt chars apart from the classics 0-9, this can take floats, ints, etc...
    fn eat_number(&mut self, ch: char) {
        let mut s = String::new();
        s.push(ch);
        self.bump(ch);

        while let Some(&c) = self.contents.peek() {
            if T::allowed_number_chars(&c) {
                s.push(c);
                self.bump(c);
            } else {
                break;
            }
        }
        if let Some(x) = T::infer_token(s, &mut self.state) {
            self.tokens.push(x);
        }
    }

    /// Eats until it finds the terminator secuence defined by the user, ignoring any kind of
    /// special chars

    fn eat_comment_block(&mut self, until: &[char]) {
        let len = until.len();
        let mut buffer = vec!['\0'; len];

        while let Some(_) = self.contents.peek() {
            let mut matched = true;
            let mut temp_iter = self.contents.clone();

            for i in 0..len {
                if let Some(ch) = temp_iter.next() {
                    buffer[i] = ch;
                    if ch != until[i] {
                        matched = false;
                        break;
                    }
                } else {
                    matched = false;
                    break;
                }
            }

            if matched {
                for _ in 0..len {
                    self.contents.next();
                }
                break;
            }
            self.contents.next();
        }
    }

    /// Just skips to the next line

    fn eat_comment_line(&mut self) {
        self.skip_line();
    }

    /// Consumes chars until it reaches a valid character that can be interpreted as a literal,
    /// delimiter, operator, .... Skipping all whitespaces.

    fn goto_next_sequence(&mut self, ch: char) {
        self.bump(ch);
        while let Some(&ch) = self.contents.peek() {
            if !ch.is_whitespace() {
                return;
            }
            self.bump(ch);
        }
    }

    /// Consumes everything until it sees a newline

    fn skip_line(&mut self) {
        while let Some(&ch) = self.contents.peek() {
            if ch == '\n' {
                self.bump(ch);
                return;
            }
            self.bump(ch);
        }
    }

    /// Eats until a delimeter (whitespaces, operators, delimiters, ...)

    fn eat_literal(&mut self) {
        let mut literal: String = String::new();
        while let Some(&ch) = self.contents.peek() {
            if T::is_delimeter(&(ch as u8)) {
                break;
            }
            literal.push(ch);
            self.bump(ch);
        }
        if let Some(x) = T::infer_token(literal, &mut self.state) {
            self.tokens.push(x);
        }
    }

    fn eat_to_newl(&mut self) -> String {
        let mut literal: String = String::new();
        while let Some(&ch) = self.contents.peek() {
            if ch == '\n' {
                break;
            }
            literal.push(ch);
            self.bump(ch);
        }
        match literal.strip_suffix(';') {
            Some(suffix_stripped) => suffix_stripped.to_string(),
            None => literal,
        }
    }

    /// Eats until it sees double quotes, is up to the user to define token inference correctly for
    /// literal interpretation on strigs that contain single quotes,

    fn eat_str(&mut self) {
        self.state.in_str = true;
        self.bump('"');
        let mut s = String::new();
        while let Some(&ch) = self.contents.peek() {
            if ch == '"' {
                break;
            }
            if ch == '\\' {
                self.bump(ch);
                if let Some(&escaped) = self.contents.peek() {
                    // basically check if the char is \u, \U or other type of unicode
                    // t is the length of the unicode to read next
                    if escaped == 'u' || escaped == 'U' {
                        self.bump(escaped);
                        if let Some(&next) = self.contents.peek() {
                            // if the next char is '{', switch to brace-style
                            if next == '{' {
                                self.bump(next);
                                let mut s2: String = String::new();
                                // eat until closing '}'
                                while let Some(&x) = self.contents.peek() {
                                    if x == '}' {
                                        self.bump(x);
                                        break;
                                    }
                                    s2.push(x);
                                    self.bump(x);
                                }
                                // if you can convert it -> push char
                                if let Ok(n) = u32::from_str_radix(&s2, 16) {
                                    if let Some(c) = char::from_u32(n) {
                                        s.push(c);
                                    } else {
                                        s.push_str(&s2);
                                    }
                                } else {
                                    s.push_str(&s2);
                                }
                            } else if let Some(mut t) = T::allowed_unicode_char(&escaped) {
                                // legacy form: eat t chars or until a whitespace is hit
                                let mut s2: String = String::new();
                                while let Some(&x) = self.contents.peek() {
                                    if x.is_whitespace() || t <= 0 {
                                        break;
                                    }
                                    t -= 1;
                                    s2.push(x);
                                    self.bump(x);
                                }
                                // if you can convert it -> push char
                                if let Ok(n) = u32::from_str_radix(&s2, 16) {
                                    if let Some(c) = char::from_u32(n) {
                                        s.push(c);
                                    } else {
                                        s.push_str(&s2);
                                    }
                                } else {
                                    s.push_str(&s2);
                                }
                            }
                        }
                    // if it's a supported escape char, just push it
                    } else if let Some(x) = T::is_scape(&escaped) {
                        s.push(x);
                        self.bump(escaped);
                    } else {
                        // unknown escape: push raw
                        self.bump(escaped);
                        s.push(escaped);
                    }
                }
            } else {
                s.push(ch);
                self.bump(ch);
            }
        }
        self.bump('"');
        if let Some(x) = T::infer_token(s, &mut self.state) {
            self.tokens.push(x);
        }
        self.state.in_str = false;
    }

    /// Eats until it sees a single quote, is up to the user to define token inference correctly for
    /// literal interpretation on strigs that contain souble quotes,

    fn eat_char(&mut self) {
        self.state.in_char = true;
        self.bump('\'');
        let mut s = String::new();
        if let Some(&ch) = self.contents.peek() {
            if ch == '\\' {
                self.bump(ch);
                if let Some(&escaped) = self.contents.peek() {
                    // basically check if the char is \u, \U or other type of unicode
                    // t is the length of the unicode to read next
                    if escaped == 'u' || escaped == 'U' {
                        self.bump(escaped);

                        if let Some(&next) = self.contents.peek() {
                            if next == '{' {
                                // handle brace-style \u{...}
                                self.bump(next); // bump '{'
                                let mut s2 = String::new();
                                while let Some(&x) = self.contents.peek() {
                                    if x == '}' {
                                        self.bump(x); // bump '}'
                                        break;
                                    }
                                    s2.push(x);
                                    self.bump(x);
                                }
                                // if you can convert it -> push char
                                if let Ok(n) = u32::from_str_radix(&s2, 16) {
                                    if let Some(c) = char::from_u32(n) {
                                        s.push(c);
                                    } else {
                                        s.push_str(&s2);
                                    }
                                } else {
                                    s.push_str(&s2);
                                }
                            } else if let Some(mut t) = T::allowed_unicode_char(&escaped) {
                                // legacy \uXXXX form
                                let mut s2 = String::new();
                                while let Some(&x) = self.contents.peek() {
                                    if x.is_whitespace() || t <= 0 {
                                        break;
                                    }
                                    t -= 1;
                                    s2.push(x);
                                    self.bump(x);
                                }
                                // if you can convert it -> push char
                                if let Ok(n) = u32::from_str_radix(&s2, 16) {
                                    if let Some(c) = char::from_u32(n) {
                                        s.push(c);
                                    } else {
                                        s.push_str(&s2);
                                    }
                                } else {
                                    s.push_str(&s2);
                                }
                            }
                        }
                    // if its a supported scape char, just push it
                    } else if let Some(x) = T::is_scape(&escaped) {
                        s.push(x);
                        self.bump(escaped);
                    } else {
                        // fallback: push raw escaped char
                        self.bump(escaped);
                        s.push(escaped);
                    }
                }
            } else {
                s.push(ch);
                self.bump(ch);
            }
        }
        if let Some(x) = T::infer_token(s, &mut self.state) {
            self.tokens.push(x);
        }
        self.state.in_char = false;
    }

    /// Handles multi-character operators like >>=, !=, ->, etc.
    /// Greedily eats characters until a delimeter non-operator or alphanumeric delimiter is found.

    fn eat_delimeter(&mut self, ch: char) {
        let mut s = Vec::<u8>::new();
        s.push(ch as u8);
        self.bump(ch);
        while let Some(&next) = self.contents.peek() {
            s.push(next as u8);

            if !T::is_operator(&s) {
                s.pop(); // undo the last push
                let token_str = unsafe { String::from_utf8_unchecked(s) };
                if let Some(x) = T::infer_token(token_str, &mut self.state) {
                    self.tokens.push(x);
                }
                return;
            }
            self.bump(next);
        }
        let token_str = unsafe { String::from_utf8_unchecked(s) };
        if let Some(x) = T::infer_token(token_str, &mut self.state) {
            self.tokens.push(x);
        }
    }
}
