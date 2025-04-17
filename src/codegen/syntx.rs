use std::collections::HashMap;
#[derive(Clone)]
pub enum CurrentState {
    Info,
    Tokens,
    Delimeters,
    Operators,
    Comments,
    Keywords,
    Scapes,
    Numbers,
}

#[derive(Clone)]
pub struct Syntx {
    pub name: String,
    pub tokens: HashMap<String, String>,
    pub delimiters: Vec<String>,
    pub operators: Vec<String>,
    pub numbers: Vec<String>,
    pub keywords: HashMap<String, String>,
    pub scapes: HashMap<String, String>,
    pub comments: [[u8; 2]; 2],
    pub state: Option<CurrentState>,
}

impl Default for Syntx {
    fn default() -> Self {
        Syntx {
            scapes: HashMap::new(),
            name: String::new(),
            tokens: HashMap::new(),
            delimiters: Vec::new(),
            numbers: Vec::new(),
            operators: Vec::new(),
            keywords: HashMap::new(),
            comments: [[0; 2]; 2],
            state: None,
        }
    }
}
