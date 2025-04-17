use crate::engine::states::State;
use crate::tokens::token_traits::*;
use std::collections::HashMap;

pub enum Action<T: Lexable + Delimeted> {
    Shift(State<T>),
    Reduce(State<T>),
    Accept,
    Error,
}

pub struct Parser<T: Lexable + Delimeted> {
    action_table: HashMap<(State<T>, T::Token), Action<T>>,
    goto_table: HashMap<(State<T>, T::Token), State<T>>,
}
