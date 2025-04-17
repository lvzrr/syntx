pub mod engine {
    pub mod errors;
    pub mod lexer;
    pub mod normalize;
    pub mod parser;
    pub mod semantic_traits;
    pub mod states;
}
pub mod tokens {
    pub mod token_traits;
}

//pub mod structures {
//    pub mod hash_tree;
//}

pub mod tests {
    //pub mod c;
    pub mod java;
}

pub mod codegen {
    pub mod codegen;
    pub mod delimeted;
    pub mod lexable;
    pub mod syntx;
    pub mod tokenset;
}

pub mod langs {
    //pub mod c;
    pub mod java {
        pub mod delimiters;
        pub mod import_resolution;
        pub mod inference;
        pub mod tokenset;
    }
    pub mod syntx {
        //pub mod syntax_resolve;
        //pub mod syntx_delimeted;
        //pub mod syntx_inference;
        //pub mod syntx_tokenset;
    }
}
