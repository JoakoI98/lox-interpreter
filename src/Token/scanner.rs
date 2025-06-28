use std::collections::HashSet;
use thiserror::Error;

use crate::Token::{single_char_token::SingleCharToken, token::Token, token_type::TokenType};

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

#[derive(Error, Debug)]
pub enum TokenErrors {
    #[error("Error: Unterminated string.")]
    NotTerminatedString,
}

static ALLOWED_NON_TOKEN_CHARS: [char; 4] = [' ', '\t', '\r', '\n'];
const LINE_SEPARATOR: char = '\n';

pub fn scan_tokens(file_content: &str) -> (Vec<Token>, Vec<ScannerError>) {
    let mut tokens = Vec::new();
    let mut errors: Vec<ScannerError> = Vec::new();
    let mut current_byte_idx = 0;
    let mut line = 1;
    let mut line_start_byte_idx = 0;
    let mut current_lexeme_start_byte_idx: usize = 0;
    let mut inside_lexeme = false;
    let mut last_token: Option<Token> = None;
    let non_token_chars_set: HashSet<char> = ALLOWED_NON_TOKEN_CHARS.into_iter().collect();
    let mut char_index = 0;
    let mut c: char = '\0';

    while current_byte_idx < file_content.len() {
        c = file_content.chars().nth(char_index).unwrap();

        if c == LINE_SEPARATOR && !inside_lexeme {
            line += 1;
            line_start_byte_idx = current_byte_idx;
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
            let safe_tokens = Token::arrange_token(token);
            match safe_tokens {
                Ok(safe_tokens) => {
                    tokens.extend(safe_tokens);
                }
                Err(error) => {
                    errors.push(error);
                }
            }
            inside_lexeme = false;
        } else {
            if !non_token_chars_set.contains(&c) {
                errors.push(ScannerError::UnexpectedCharacter(c, line));
            }
            current_byte_idx += c.len_utf8();
            char_index += 1;
            inside_lexeme = false;
        }
    }

    if let Some(token) = last_token {
        let safe_tokens = Token::arrange_token(token);
        match safe_tokens {
            Ok(safe_tokens) => {
                tokens.extend(safe_tokens);
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    tokens.push(Token {
        token_type: TokenType::SingleCharToken(SingleCharToken::Eof),
        lexeme: "".to_string(),
        line,
        column_start: 0,
        column_end: 0,
    });

    (tokens, errors)
}
