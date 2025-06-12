use std::collections::HashSet;
use thiserror::Error;

use crate::Token::{single_char_token::SingleCharToken, token::Token, token_type::TokenType};

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("[line {1}] Error: Unexpected character: {0}")]
    UnexpectedCharacter(char, usize),
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
    let mut non_token_chars_set: HashSet<char> = ALLOWED_NON_TOKEN_CHARS.into_iter().collect();

    while current_byte_idx < file_content.len() {
        let c = file_content.chars().nth(current_byte_idx).unwrap();

        if c == LINE_SEPARATOR && !inside_lexeme {
            line += 1;
            line_start_byte_idx = current_byte_idx;
        }

        if !inside_lexeme {
            current_lexeme_start_byte_idx = current_byte_idx;
            inside_lexeme = true;
        }

        let current_lexeme = &file_content[current_lexeme_start_byte_idx..current_byte_idx + 1];

        if let Some(token) = Token::from_str(current_lexeme, line, current_lexeme_start_byte_idx) {
            last_token = Some(token);
            current_byte_idx += c.len_utf8();
        } else if let Some(token) = last_token {
            last_token = None;
            tokens.push(token);
            inside_lexeme = false;
        } else {
            if !non_token_chars_set.contains(&c) {
                errors.push(ScannerError::UnexpectedCharacter(c, line));
            }
            current_byte_idx += c.len_utf8();
            inside_lexeme = false;
        }
    }

    if let Some(token) = last_token {
        tokens.push(token);
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
