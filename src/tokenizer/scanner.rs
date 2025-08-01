use std::collections::{HashSet, LinkedList};
use thiserror::Error;

use super::token::{EOFToken, StaticToken, Token};

fn skip_single_line_comment(str: &str) -> (usize, usize, usize) {
    let mut byte_idx = 0;
    let mut char_idx = 0;

    while let Some(c) = str.chars().nth(char_idx) {
        char_idx += 1;
        byte_idx += c.len_utf8();
        if c == LINE_SEPARATOR {
            break;
        }
    }

    return (byte_idx, char_idx, 1);
}

fn skip_multi_line_comment(str: &str) -> (usize, usize, usize) {
    let mut byte_idx = 0;
    let mut char_idx = 0;
    let mut comment_lines_count = 0;

    while let Some(c) = str.chars().nth(char_idx) {
        char_idx += 1;
        byte_idx += c.len_utf8();

        if c == LINE_SEPARATOR {
            comment_lines_count += 1;
        }

        if c == '/' {
            if let Some(c2) = str.chars().nth(char_idx - 2) {
                if c2 == '*' {
                    break;
                }
            }
        }
    }

    return (byte_idx, char_idx, comment_lines_count);
}

fn skip_comment(str: &str) -> Option<(usize, usize, usize)> {
    let first_two_chars = str.chars().take(2).collect::<String>();

    match first_two_chars.as_str() {
        "//" => Some(skip_single_line_comment(str)),
        "/*" => Some(skip_multi_line_comment(str)),
        _ => None,
    }
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("[line {1}] Error: Unexpected character: {0}")]
    UnexpectedCharacter(char, usize),

    #[error("[line {0}] Error: Unterminated string.")]
    NotTerminatedString(usize),
}

static ALLOWED_NON_TOKEN_CHARS: [char; 4] = [' ', '\t', '\r', '\n'];
const LINE_SEPARATOR: char = '\n';

fn push_token(
    tokens: &mut LinkedList<Token>,
    errors: &mut LinkedList<ScannerError>,
    arranged_tokens: Result<(Token, Option<Token>), ScannerError>,
) -> () {
    match arranged_tokens {
        Ok((token, None)) => {
            tokens.push_back(token);
        }
        Ok((token, Some(token2))) => {
            tokens.push_back(token);
            tokens.push_back(token2);
        }
        Err(error) => {
            errors.push_back(error);
        }
    };
}

pub fn scan_tokens(file_content: &str) -> (Vec<StaticToken>, Vec<ScannerError>) {
    let mut tokens = LinkedList::new();
    let mut errors = LinkedList::new();
    let mut current_byte_idx = 0;
    let mut line = 1;
    let mut current_lexeme_start_byte_idx: usize = 0;
    let mut inside_lexeme = false;
    let mut last_token: Option<Token> = None;
    let non_token_chars_set: HashSet<char> = ALLOWED_NON_TOKEN_CHARS.into_iter().collect();
    let mut char_index = 0;
    let mut c: char;

    while current_byte_idx < file_content.len() {
        c = file_content.chars().nth(char_index).unwrap();

        if c == LINE_SEPARATOR && !inside_lexeme {
            line += 1;
        }

        if !inside_lexeme {
            if let Some((byte_idx, char_idx, line_count)) =
                skip_comment(&file_content[current_byte_idx..])
            {
                current_byte_idx += byte_idx;
                char_index += char_idx;
                line += line_count;
                continue;
            }

            inside_lexeme = true;
            current_lexeme_start_byte_idx = current_byte_idx;
        }

        let current_lexeme =
            &file_content[current_lexeme_start_byte_idx..current_byte_idx + c.len_utf8()];

        if let Some(token) = Token::from_str(current_lexeme, line, current_lexeme_start_byte_idx) {
            last_token = Some(token);
            current_byte_idx += c.len_utf8();
            char_index += 1;
        } else if let Some(token) = last_token {
            last_token = None;
            push_token(&mut tokens, &mut errors, Token::arrange_token(token));
            inside_lexeme = false;
        } else {
            if !non_token_chars_set.contains(&c) {
                errors.push_back(ScannerError::UnexpectedCharacter(c, line));
            }
            current_byte_idx += c.len_utf8();
            char_index += 1;
            inside_lexeme = false;
        }
    }

    if let Some(token) = last_token {
        push_token(&mut tokens, &mut errors, Token::arrange_token(token));
    }

    tokens.push_back(Token {
        token_type: Box::new(EOFToken),
        lexeme: "".to_string(),
        line,
        column_start: 0,
        column_end: 0,
    });

    (tokens.into_iter().collect(), errors.into_iter().collect())
}
