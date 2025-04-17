use crate::codegen::delimeted::delimeted_codegen;
use crate::codegen::lexable::infer_codegen;
use crate::codegen::syntx::Syntx;
use crate::codegen::syntx::*;
use crate::codegen::tokenset::enum_codegen;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn trim_quotes(s: &str) -> &str {
    let s = s.trim_start();
    let s = if s.ends_with(';') {
        &s[..s.len() - 1]
    } else {
        s
    };
    let s = if s.starts_with('"') { &s[1..] } else { s };
    let s = if s.ends_with('"') && s.len() >= 1 {
        &s[..s.len() - 1]
    } else {
        s
    };
    s
}

pub fn codegen(src: &str) {
    let mut stx: Syntx = Syntx::default();

    for l in src.lines() {
        let l = l.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }

        match l {
            "[info]" => {
                stx.state = Some(CurrentState::Info);
                continue;
            }
            "[tokens]" => {
                stx.state = Some(CurrentState::Tokens);
                continue;
            }
            "[delimeters]" => {
                stx.state = Some(CurrentState::Delimeters);
                continue;
            }
            "[operators]" => {
                stx.state = Some(CurrentState::Operators);
                continue;
            }
            "[comments]" => {
                stx.state = Some(CurrentState::Comments);
                continue;
            }
            "[keywords]" => {
                stx.state = Some(CurrentState::Keywords);
                continue;
            }
            "[scapes]" => {
                stx.state = Some(CurrentState::Scapes);
                continue;
            }
            "[numbers]" => {
                stx.state = Some(CurrentState::Numbers);
                continue;
            }
            _ => {}
        }

        if let Some(state) = &stx.state {
            match state {
                CurrentState::Info => {
                    if let Some((_, value)) = l.split_once('=') {
                        stx.name = value
                            .trim()
                            .trim_end_matches(';')
                            .trim_end_matches('"')
                            .trim_start_matches('"')
                            .to_string();
                    }
                }
                CurrentState::Tokens => {
                    if let Some((key, value)) = l.split_once('=') {
                        stx.tokens.insert(
                            key.trim().to_string(),
                            value
                                .trim()
                                .trim_end_matches(';')
                                .trim_matches('"')
                                .trim_end_matches('"')
                                .to_string(),
                        );
                    }
                }
                CurrentState::Delimeters => {
                    let token = l.trim_end_matches(';').to_string();
                    stx.delimiters.push(token);
                }
                CurrentState::Operators => {
                    let token = l.trim_end_matches(';').to_string();
                    stx.operators.push(token);
                }
                CurrentState::Comments => {
                    if let Some((key, value)) = l.split_once('=') {
                        let key = key.trim();
                        let value = value.trim().trim_end_matches(';');

                        if key == "line" {
                            let line_comment = value.trim_matches('"');
                            let chars: Vec<u8> = line_comment.chars().map(|c| c as u8).collect();
                            if chars.len() == 2 {
                                stx.comments[0] = [chars[0], chars[1]];
                            }
                        } else if key == "block" {
                            let parts: Vec<&str> = value
                                .trim_matches(|c| c == '[' || c == ']')
                                .split(',')
                                .map(|s| s.trim().trim_matches('"'))
                                .collect();

                            if parts.len() == 2 {
                                let start = parts[0];
                                let end = parts[1];
                                let start_char = start.chars().next().unwrap_or('\0') as u8;
                                let end_char = end.chars().next().unwrap_or('\0') as u8;
                                stx.comments[1] = [start_char, end_char];
                            }
                        }
                    }
                }
                CurrentState::Keywords => {
                    let keyword = l.trim_end_matches(';').to_string();
                    stx.keywords.insert(capitalize(&keyword), keyword.clone());
                }
                CurrentState::Scapes => {
                    if let Some((mut key, value)) = l.split_once('=') {
                        key = key.trim();
                        let value = trim_quotes(value).to_string();
                        stx.scapes.insert(key.to_string(), value);
                    }
                }
                CurrentState::Numbers => {
                    stx.numbers = l
                        .trim_end_matches(';')
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .split(",")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|x| {
                            x.trim()
                                .trim_start_matches('"')
                                .trim_end_matches('"')
                                .to_string()
                        })
                        .collect::<Vec<String>>();
                }
            }
        }
    }
    enum_codegen(stx.clone());
    delimeted_codegen(stx.clone());
    infer_codegen(stx.clone());
}
