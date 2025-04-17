///! Error Definitions (engine/errors.rs)
///!
///! These are the errors the lexer will raise in the real time
///! TODO: A Display trait for this is essential.

#[derive(Debug)]
pub struct LexicalError<T> {
    pub row: usize,
    pub column: usize,
    pub expected: T,
    pub context: Option<Vec<T>>,
}
