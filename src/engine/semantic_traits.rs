///! Semantic Traits Interface (engine/semantic_traits.rs)
///!
///! Traits needed for the lexer engine to work, also future interfaces for the LookAhead (grammar
///! prediction)
use crate::tokens::token_traits::*;

//trait LookAhead<T: Lexable + Delimeted> {
//    fn get_context(&self) -> Option<Vec<T::Token>>;
//    fn predict_next(&self, context: Option<Vec<T::Token>>);
//}
//
//trait TokenFactory<T: Lexable + Delimeted> {
//    fn try_match(&self, input: &str) -> Option<T::Token>;
//}

//// This
pub trait Walker<T: Lexable + Delimeted> {
    fn bump(&mut self, ch: char);
    fn tokenize(&mut self);
    fn goto_next_sequence(&mut self, ch: char);
    fn skip_line(&mut self);
    fn eat_literal(&mut self);
    fn eat_to_newl(&mut self) -> String;
    fn eat_str(&mut self);
    fn eat_char(&mut self);
    fn eat_comment_block(&mut self, until: &[char]);
    fn eat_comment_line(&mut self);
    fn eat_delimeter(&mut self, ch: char);
    fn eat_number(&mut self, ch: char);
}
