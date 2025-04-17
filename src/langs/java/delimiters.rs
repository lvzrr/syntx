use crate::langs::java::tokenset::*;
use crate::tokens::token_traits::Delimeted;
impl Delimeted for JavaTokenSet {
    #[inline(always)]
    fn allowed_unicode_char(c: &char) -> Option<usize> {
        match c {
            'u' => Some(4),
            'U' => Some(8),
            _ => None,
        }
    }

    #[inline(always)]
    fn is_scape(c: &char) -> Option<char> {
        match c {
            't' => Some('\t'),
            'b' => Some('\u{0008}'),
            'n' => Some('\n'),
            'r' => Some('\r'),
            'f' => Some('\u{000c}'),
            '\'' => Some('\''),
            '\"' => Some('\"'),
            '\\' => Some('\\'),
            _ => None,
        }
    }

    #[inline(always)]
    fn may_trigger_line_comment(c: char) -> Option<usize> {
        if c == '/' { Some(2) } else { None }
    }

    #[inline(always)]
    fn trigger_comment_line(input: &[char]) -> bool {
        input == ['/', '/']
    }

    #[inline(always)]
    fn may_trigger_block_comment(c: char) -> Option<(Vec<char>, usize)> {
        if c == '/' {
            Some((vec!['*', '/'], 2))
        } else {
            None
        }
    }

    #[inline(always)]
    fn allowed_number_chars(c: &char) -> bool {
        c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | 'e' | 'E' | '-' | '+' | 'f' | 'F')
    }

    #[inline(always)]
    fn is_operator(s: &[u8]) -> bool {
        matches!(
            s,
            b"!" | b"."
                | b"@"
                | b">"
                | b">="
                | b"<"
                | b"<="
                | b">>"
                | b"<<"
                | b">>>"
                | b">>="
                | b"<<="
                | b">>>="
                | b"+"
                | b"-"
                | b"*"
                | b"="
                | b"=="
                | b"/"
                | b"+="
                | b"-="
                | b"*="
                | b"/="
                | b"%"
                | b"%="
                | b"++"
                | b"--"
                | b"&"
                | b"&="
                | b"^"
                | b"^="
                | b"~"
                | b"|"
                | b"|="
                | b"!="
                | b"&&"
                | b"||"
                | b"?"
                | b"instanceof"
        )
    }

    #[inline(always)]
    fn trigger_comment_block(input: &[char]) -> bool {
        input == ['/', '*']
    }

    #[inline(always)]
    fn is_delimeter(c: &u8) -> bool {
        matches!(
            c,
            b'+' | b'-'
                | b'<'
                | b'>'
                | b'='
                | b'!'
                | b'|'
                | b'*'
                | b'~'
                | b'^'
                | b')'
                | b'('
                | b'}'
                | b'{'
                | b'['
                | b']'
                | b','
                | b';'
                | b'/'
                | b':'
                | b'?'
                | b'&'
                | b'.'
                | b'%'
                | b' '
                | b'\t'
                | b'\n'
        )
    }
}
