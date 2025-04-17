#[cfg(test)]
mod test {
    use crate::engine::lexer::Lexer;
    use crate::engine::semantic_traits::Walker;
    use crate::langs::java::tokenset::*;
    use crossbeam::channel::unbounded;
    use procfs::process::Process;
    use std::thread;
    use std::time::Instant;

    fn assert_runtime_and_memory<F: FnOnce()>(f: F, max_time_ms: u128, max_mem_mb: f64) {
        let pid = std::process::id();
        let start_mem = Process::new(pid as i32).unwrap().statm().unwrap().resident;
        let page_size = procfs::page_size();
        let start = Instant::now();

        f();

        let elapsed = start.elapsed();
        let end_mem = Process::new(pid as i32).unwrap().statm().unwrap().resident;
        let used_pages = end_mem.saturating_sub(start_mem);
        let used_mb = (used_pages * page_size as u64) as f64 / (1024.0 * 1024.0);

        assert!(
            elapsed.as_millis() <= max_time_ms,
            "⏰ Test exceeded time limit: {}ms > {}ms",
            elapsed.as_millis(),
            max_time_ms
        );
        assert!(
            used_mb <= max_mem_mb,
            "💾 Test exceeded memory limit: {:.2}MB > {:.2}MB",
            used_mb,
            max_mem_mb
        );
    }

    fn run_lexer(input: &str) -> Vec<JavaToken> {
        let (sender, receiver) = unbounded();
        let input = input.to_owned();

        let handle = thread::spawn(move || {
            let mut lexer = Lexer::<JavaTokenSet>::new(&input, sender);
            lexer.tokenize();
        });

        let mut tokens = Vec::with_capacity(512);
        for batch in receiver {
            tokens.extend(batch);
        }

        handle.join().expect("Lexer thread panicked");
        tokens
    }

    fn assert_token_present(tokens: &[JavaToken], expected: JavaToken) {
        assert!(
            tokens.iter().any(|t| t == &expected),
            "🚨 Token {:?} not found!",
            expected,
        );
    }

    // 🔥 Massive Escape Sequences Test
    #[test]
    fn crazy_unicode_escapes_everywhere() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u{1F600}\u{1F602}\u{1F60D}""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::StringLiteral(s)) if s == "😀😂😍"
                )));
            },
            8,
            5.0,
        );
    }

    // 🔥 Nested Multiline and Single Line Comments
    #[test]
    fn nested_comments_survival() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(
                    r#"
                    /* comment /* nested */ still comment */
                    // single-line comment
                    public class A {}
                "#,
                );

                assert_token_present(
                    &tokens,
                    JavaToken::Identifier(JavaIdentifier::JavaKeyword(JavaKeyword::Public)),
                );
                assert_token_present(
                    &tokens,
                    JavaToken::Identifier(JavaIdentifier::JavaKeyword(JavaKeyword::Class)),
                );
            },
            10,
            6.0,
        );
    }

    // 🔥 Massive Variable Soup
    #[test]
    fn insane_variable_naming() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("int αβγ = 123; String $name_123 = \"ok\";");
                assert_token_present(
                    &tokens,
                    JavaToken::Identifier(JavaIdentifier::JavaKeyword(JavaKeyword::Int)),
                );
                assert_token_present(
                    &tokens,
                    JavaToken::Identifier(JavaIdentifier::JavaKeyword(JavaKeyword::Class)),
                );
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::Integer(s, _)) if s == "123"
                )));
            },
            10,
            5.0,
        );
    }

    // 🔥 Super crazy float/hex/bin literals
    #[test]
    fn crazy_numeric_literals() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(
                    "double pi = 3.1415; int hex = 0xDEAD; int bin = 0b1010; float big = 6.02e23f;",
                );
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::Float(f)) if *f > 3.0
                )));
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::Integer(s, _)) if s.contains("0xDEAD")
                )));
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::Integer(s, _)) if s.contains("0b1010")
                )));
            },
            10,
            5.0,
        );
    }

    // 🔥 Ultra-deep angle bracket <> nesting
    #[test]
    fn deep_generics_brackets() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer("Map<String, List<Map<Integer, List<String>>>> deepMap;");
                dbg!(&tokens);
                assert_token_present(&tokens, JavaToken::Operator(JavaOperator::Lt));
                assert_token_present(&tokens, JavaToken::Operator(JavaOperator::UBitShiftRight));
            },
            10,
            5.0,
        );
    }

    // 🔥 Brutal Large String and Char Literals
    #[test]
    fn large_strings_and_chars() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(
                    r#"
                    String s = "This is a massive string with a huge number of characters and escape sequences \n \t \u{1F60D}";
                    char c = '💀';
                "#,
                );

                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::StringLiteral(s)) if s.contains("massive string")
                )));
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::CharLiteral(s)) if s.contains("💀")
                )));
            },
            15,
            7.0,
        );
    }

    // 🔥 Invalid Unicode / Garbage inside
    #[test]
    fn invalid_unicode_and_recovery() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(r#""\u{ZZZZ} garbage""#);
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::StringLiteral(s)) if s.contains("garbage")
                )));
            },
            8,
            5.0,
        );
    }

    // 🔥 Wildcard Import Expansion Test
    #[test]
    fn wildcard_import_expansion() {
        assert_runtime_and_memory(
            || {
                let tokens = run_lexer(
                    r#"
                import testpkg.*;
                int x = 42;
                "#,
                );

                // Should see the classes imported first
                assert_token_present(
                    &tokens,
                    JavaToken::Identifier(JavaIdentifier::JavaKeyword(JavaKeyword::Public)),
                );
                assert_token_present(
                    &tokens,
                    JavaToken::Identifier(JavaIdentifier::JavaKeyword(JavaKeyword::Class)),
                );

                // And our own "int x = 42;" code after
                assert_token_present(
                    &tokens,
                    JavaToken::Identifier(JavaIdentifier::JavaKeyword(JavaKeyword::Int)),
                );
                assert_token_present(&tokens, JavaToken::Operator(JavaOperator::Assign));
                assert!(tokens.iter().any(|t| matches!(
                    t,
                    JavaToken::Identifier(JavaIdentifier::Integer(s, _)) if s == "42"
                )));
            },
            20,
            8.0,
        );
    }
}
