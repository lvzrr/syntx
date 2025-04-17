mod test {
    #[cfg(test)]
    use crate::engine::lexer::Lexer;
    #[cfg(test)]
    use crate::engine::semantic_traits::Walker;
    #[cfg(test)]
    use crate::langs::c::*;
    #[cfg(test)]
    use procfs::process::Process;
    #[cfg(test)]
    use std::time::Instant;

    #[cfg(test)]
    fn assert_runtime_and_memory<F: FnOnce()>(f: F, max_time_ms: u128, max_mem_mb: f64) {
        let pid = std::process::id() as i32;
        let start_mem = Process::new(pid).unwrap().statm().unwrap().resident;
        let page_size = procfs::page_size();
        let start = Instant::now();

        f();

        let elapsed = start.elapsed();
        let end_mem = Process::new(pid).unwrap().statm().unwrap().resident;
        let used_pages = end_mem.saturating_sub(start_mem);
        let used_mb = (used_pages * page_size as u64) as f64 / (1024.0 * 1024.0);

        if elapsed.as_millis() > max_time_ms {
            panic!(
                "Test exceeded time limit: {}ms > {}ms",
                elapsed.as_millis(),
                max_time_ms
            );
        }
        if used_mb > max_mem_mb {
            panic!(
                "Test exceeded memory limit: {:.2}MB > {:.2}MB",
                used_mb, max_mem_mb
            );
        }
    }

    #[cfg(test)]
    fn run_lexer(input: &str) -> Vec<ClangToken> {
        let mut lexer = Lexer::<ClangTokenSet>::from_str(input);
        lexer.tokenize();
        lexer.tokens
    }

    #[cfg(test)]
    fn assert_token_present(tokens: &Vec<ClangToken>, expected: ClangToken) {
        assert!(
            tokens.iter().any(|tok| *tok == expected),
            "Token {:?} not found!",
            expected,
        );
    }

    #[test]
    fn test_basic_tokenization() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("int main() {}");
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::ClangKeyword(ClangKeyword::Int)),
                );
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_literals_and_operators() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("int x = 0xFF + 42;");
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::Integer(
                        "0xFF".into(),
                        ClangBase::Hexadecimal,
                    )),
                );
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::Integer(
                        "42".into(),
                        ClangBase::Decimal,
                    )),
                );
                assert_token_present(&tokens, ClangToken::Operator(ClangOperator::Plus));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_string_and_char_literals() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("char c = '\n'; char* s = \"hello\\nworld\";");
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::CharLiteral(s)) if s == "\n"
                )));
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s.contains("hello")
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_comments_are_skipped() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("// comment\nint x; /* block */");
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::ClangKeyword(ClangKeyword::Int)),
                );
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_operators_and_delimiters() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("x += 1; if (x >= 2) {}");
                assert_token_present(&tokens, ClangToken::Operator(ClangOperator::PlusEq));
                assert_token_present(&tokens, ClangToken::Operator(ClangOperator::Geq));
                assert_token_present(&tokens, ClangToken::Delimeter(ClangDelimeters::LParen));
                assert_token_present(&tokens, ClangToken::Delimeter(ClangDelimeters::RBrace));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_unicode_and_escaped_quotes() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#"char* s = "🔥\"Hello\"\n";"#);
                dbg!(&tokens);
                assert!(tokens.iter().any(|t| matches!(
                t,
                ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s.contains("🔥") && s.contains("\"Hello\"")
            )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_scientific_and_suffix_floats() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("float a = 3.14e10f; double b = 6.022E23;");
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::Float(f)) if *f > 1e10
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_mismatched_delimiters() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("if (x > 5 {");
                assert_token_present(&tokens, ClangToken::Delimeter(ClangDelimeters::LParen));
                assert_token_present(&tokens, ClangToken::Delimeter(ClangDelimeters::LBrace));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_unicode_escape_u_fixed() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u00A9""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s == "©"
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_unicode_escape_brace_short() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u{A9}""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s == "©"
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_unicode_escape_brace_long() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u{1F600}""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s == "😀"
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_unicode_escape_multiple_in_string() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u{A9} and \u{1F600}""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s == "© and 😀"
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_invalid_unicode_escape_is_raw() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\uZZZZ""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s == "ZZZZ"
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_unclosed_brace_escape_fallback() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u{1F600""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s.contains("1F600")
                )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_brace_unicode_overflow() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u{110000}""#);
                assert!(tokens.iter().any(|t| matches!(
                t,
                ClangToken::Identifier(ClangIdentifier::StringLiteral(s)) if s.contains("110000")
            )));
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_variable_declarations() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("int a = 10; float b = 3.14;");
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::ClangKeyword(ClangKeyword::Int)),
                );
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::ClangKeyword(ClangKeyword::Float)),
                );
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::Integer(
                        "10".into(),
                        ClangBase::Decimal,
                    )),
                );
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::Float(3.14)),
                );
            },
            10,
            10.0,
        );
    }

    #[test]
    fn test_more_primitive_declarations() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("char c = 'x'; double d = 1.23e4;");
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::ClangKeyword(ClangKeyword::Char)),
                );
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::CharLiteral("x".into())),
                );
                assert_token_present(
                    &tokens,
                    ClangToken::Identifier(ClangIdentifier::ClangKeyword(ClangKeyword::Double)),
                );
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    ClangToken::Identifier(ClangIdentifier::Float(f)) if *f == 1.23e4
                )));
            },
            10,
            10.0,
        );
    }
}
