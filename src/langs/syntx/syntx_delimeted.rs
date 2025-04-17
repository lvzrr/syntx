use crate::langs::syntx::syntx_tokenset::*;
use crate::tokens::token_traits::Delimeted;

impl Delimeted for syntxTokenSet {
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
			'r' => Some('\r'),
			'f' => Some('\u{000C}'),
			'\\' => Some('\\'),
			'b' => Some('\u{0008}'),
			'\'' => Some('\''),
			'n' => Some('\n'),
			'"' => Some('\"'),
			_ => None
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
            Some((vec!['/', '*'], 2))
        } else {
            None
        }
    }

	#[inline(always)]
    fn is_delimeter(c: &u8) -> bool {
        matches!(
            c,
			b'='
			| b';'
			| b'['
			| b']'
			| b','
		)
	}

	#[inline(always)]
    fn trigger_comment_block(input: &[char]) -> bool {
        input == ['/', '*']
    }

	#[inline(always)]
    fn is_operator(s: &[u8]) -> bool {
        matches!(
            s,
			b"="
		)
	}

	#[inline(always)]
    fn allowed_number_chars(c: &char) -> bool {
        c.is_ascii_alphanumeric() || matches!(c, '-'
			 | '+'
			 | 'f'
			 | 'F'
			 | '.'
			 | '_'
			 | 'e'
			 | 'E'
		)
	}
}