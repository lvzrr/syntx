use crate::engine::lexer::Lexer;
use crate::engine::semantic_traits::Walker;
use crate::langs::java::tokenset::JavaTokenSet;
use crate::tokens::token_traits::Resolvable;
use memmap2::Mmap;
use std::env;
use std::fs::{File, read_dir};
use std::path::{Path, PathBuf};

use super::tokenset::JavaToken;

impl Resolvable for JavaTokenSet {
    fn resolve_import(name: String, sender: crossbeam::channel::Sender<Vec<JavaToken>>) {
        if name.starts_with("java") || name.starts_with("javax") {
            return;
        }
        let name = name.replace('.', "/") + ".java";
        #[cfg(windows)]
        let mut classpath: Vec<String> = env::var("CLASSPATH")
            .unwrap_or_default()
            .split(';')
            .map(|s| s.to_string())
            .collect();

        #[cfg(not(windows))]
        let mut classpath: Vec<String> = env::var("CLASSPATH")
            .unwrap_or_default()
            .split(':')
            .map(|s| s.to_string())
            .collect();

        if classpath.is_empty() {
            classpath = vec![".".to_string()];
        }
        if name.ends_with("*.java") {
            let base = name.trim_end_matches("*.java");
            let dir_path = Path::new(base);

            if let Ok(entries) = read_dir(dir_path) {
                for file in entries.flatten() {
                    let path = file.path();
                    if let Some(ext) = path.extension() {
                        if ext == "java" {
                            if let Ok(file) = File::open(&path) {
                                if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                                    if let Ok(source_code) = std::str::from_utf8(&mmap) {
                                        let mut lexer =
                                            Lexer::<JavaTokenSet>::new(source_code, sender.clone());
                                        lexer.tokenize();
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                eprintln!("Could not read directory: {}", base);
            }
        }
        for path in &classpath {
            let full_path = PathBuf::from(path).join(&name);

            if let Ok(file) = File::open(&full_path) {
                if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                    let source_code = std::str::from_utf8(&mmap).unwrap_or_default();

                    let mut lexer = Lexer::<JavaTokenSet>::new(source_code, sender.clone());
                    lexer.tokenize();
                    break;
                }
            }
        }
    }
}
