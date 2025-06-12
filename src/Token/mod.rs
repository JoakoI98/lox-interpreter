use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
pub enum SingleCharToken {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    Equal,
    Greater,
    Less,
    Eof,
}
#[derive(Debug)]
pub enum TwoCharToken {
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
}
#[derive(Debug)]
pub enum LiteralToken {
    Identifier,
    String(String),
    Number(f64),
}
#[derive(Debug)]
pub enum KeywordToken {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl SingleCharToken {
    fn from_str(str: &str) -> Option<SingleCharToken> {
        if str.len() != 1 {
            return None;
        }

        let c = str.chars().next().unwrap();
        match c {
            '(' => Some(SingleCharToken::LeftParen),
            ')' => Some(SingleCharToken::RightParen),
            '{' => Some(SingleCharToken::LeftBrace),
            '}' => Some(SingleCharToken::RightBrace),
            ',' => Some(SingleCharToken::Comma),
            '.' => Some(SingleCharToken::Dot),
            '-' => Some(SingleCharToken::Minus),
            '+' => Some(SingleCharToken::Plus),
            ';' => Some(SingleCharToken::Semicolon),
            '/' => Some(SingleCharToken::Slash),
            '*' => Some(SingleCharToken::Star),
            '!' => Some(SingleCharToken::Bang),
            '=' => Some(SingleCharToken::Equal),
            '>' => Some(SingleCharToken::Greater),
            '<' => Some(SingleCharToken::Less),
            '\0' => Some(SingleCharToken::Eof),
            _ => None,
        }
    }
}

impl Display for SingleCharToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleCharToken::LeftParen => write!(f, "LEFT_PAREN"),
            SingleCharToken::RightParen => write!(f, "RIGHT_PAREN"),
            SingleCharToken::LeftBrace => write!(f, "LEFT_BRACE"),
            SingleCharToken::RightBrace => write!(f, "RIGHT_BRACE"),
            SingleCharToken::Comma => write!(f, "COMMA"),
            SingleCharToken::Dot => write!(f, "DOT"),
            SingleCharToken::Minus => write!(f, "MINUS"),
            SingleCharToken::Plus => write!(f, "PLUS"),
            SingleCharToken::Semicolon => write!(f, "SEMICOLON"),
            SingleCharToken::Slash => write!(f, "SLASH"),
            SingleCharToken::Star => write!(f, "STAR"),
            SingleCharToken::Bang => write!(f, "BANG"),
            SingleCharToken::Equal => write!(f, "EQUAL"),
            SingleCharToken::Greater => write!(f, "GREATER"),
            SingleCharToken::Less => write!(f, "LESS"),
            SingleCharToken::Eof => write!(f, "EOF"),
        }
    }
}

impl TwoCharToken {
    fn from_str(str: &str) -> Option<TwoCharToken> {
        None
    }
}

impl Display for TwoCharToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwoCharToken::BangEqual => write!(f, "BANG_EQUAL"),
            TwoCharToken::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TwoCharToken::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TwoCharToken::LessEqual => write!(f, "LESS_EQUAL"),
        }
    }
}

impl LiteralToken {
    fn from_str(str: &str) -> Option<LiteralToken> {
        None
    }
}

impl Display for LiteralToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralToken::Identifier => write!(f, "IDENTIFIER"),
            LiteralToken::String(_string) => write!(f, "STRING"),
            LiteralToken::Number(_number) => write!(f, "NUMBER"),
        }
    }
}

impl KeywordToken {
    fn from_str(str: &str) -> Option<KeywordToken> {
        None
    }
}

impl Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordToken::And => write!(f, "AND"),
            KeywordToken::Class => write!(f, "CLASS"),
            KeywordToken::Else => write!(f, "ELSE"),
            KeywordToken::False => write!(f, "FALSE"),
            KeywordToken::Fun => write!(f, "FUN"),
            KeywordToken::For => write!(f, "FOR"),
            KeywordToken::If => write!(f, "IF"),
            KeywordToken::Nil => write!(f, "NIL"),
            KeywordToken::Or => write!(f, "OR"),
            KeywordToken::Return => write!(f, "RETURN"),
            KeywordToken::Super => write!(f, "SUPER"),
            KeywordToken::This => write!(f, "THIS"),
            KeywordToken::True => write!(f, "TRUE"),
            KeywordToken::Var => write!(f, "VAR"),
            KeywordToken::While => write!(f, "WHILE"),
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    SingleCharToken(SingleCharToken),
    TwoCharToken(TwoCharToken),
    LiteralToken(LiteralToken),
    KeywordToken(KeywordToken),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::SingleCharToken(token) => write!(f, "{}", token),
            TokenType::TwoCharToken(token) => write!(f, "{}", token),
            TokenType::LiteralToken(token) => write!(f, "{}", token),
            TokenType::KeywordToken(token) => write!(f, "{}", token),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column_start: usize,
    pub column_end: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal_value = match &self.token_type {
            TokenType::LiteralToken(LiteralToken::String(s)) => s.clone(),
            TokenType::LiteralToken(LiteralToken::Number(n)) => n.to_string(),
            _ => "null".to_string(),
        };

        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_value)
    }
}

impl Token {
    pub fn from_str(str: &str, line: usize, column_start: usize) -> Option<Token> {
        let single_char_token = SingleCharToken::from_str(str);
        if let Some(single_char_token) = single_char_token {
            return Some(Token {
                token_type: TokenType::SingleCharToken(single_char_token),
                lexeme: str.to_string(),
                line,
                column_start,
                column_end: column_start + str.len(),
            });
        }

        return None;
    }
}

static ALLOWED_NON_TOKEN_CHARS: [char; 4] = [' ', '\t', '\r', '\n'];
const LINE_SEPARATOR: char = '\n';

pub fn scan_tokens(file_content: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
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
                eprintln!("[line {}] Error: Unexpected character: {}", line, c);
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

    tokens
}
