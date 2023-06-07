use std::iter::Peekable;
use std::str::Chars;

/// A token is a single unit of a programming language.
///
/// Tokens are the building blocks of a programming language.
/// * Token Type: The type of token.
/// * Lexeme: The actual text of the token.
/// * Literal: The value of the token.
/// * Line: The line number of the token.
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Function,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Variable,
    While,

    // End of file.
    Eof,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();
    let mut line = 1;

    while let Some(&c) = chars.peek() {
        match c {
            '(' => tokens.push(single_char_token(TokenType::LeftParen, &mut chars, line)),
            ')' => tokens.push(single_char_token(TokenType::RightParen, &mut chars, line)),
            '{' => tokens.push(single_char_token(TokenType::LeftBrace, &mut chars, line)),
            '}' => tokens.push(single_char_token(TokenType::RightBrace, &mut chars, line)),
            ',' => tokens.push(single_char_token(TokenType::Comma, &mut chars, line)),
            '.' => tokens.push(single_char_token(TokenType::Dot, &mut chars, line)),
            '-' => tokens.push(single_char_token(TokenType::Minus, &mut chars, line)),
            '+' => tokens.push(single_char_token(TokenType::Plus, &mut chars, line)),
            ';' => tokens.push(single_char_token(TokenType::Semicolon, &mut chars, line)),
            '/' => tokens.push(single_char_token(TokenType::Slash, &mut chars, line)),
            '*' => tokens.push(single_char_token(TokenType::Star, &mut chars, line)),
            '!' => {
                let token_type = if let Some('=') = chars.peek().cloned() {
                    chars.next(); // Consume the second '=' character
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                tokens.push(single_char_token(token_type, &mut chars, line));
            }
            '=' => {
                let token_type = if let Some('=') = chars.peek().cloned() {
                    chars.next(); // Consume the second '=' character
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                tokens.push(single_char_token(token_type, &mut chars, line));
            }
            '>' => {
                let token_type = if let Some('=') = chars.peek().cloned() {
                    chars.next(); // Consume the second '=' character
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                tokens.push(single_char_token(token_type, &mut chars, line));
            }
            '<' => {
                let token_type = if let Some('=') = chars.peek().cloned() {
                    chars.next(); // Consume the second '=' character
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                tokens.push(single_char_token(token_type, &mut chars, line));
            }
            '"' => tokens.push(string_token(&mut chars, line)),
            _ if c.is_alphabetic() || c == '_' => tokens.push(identifier_or_keyword(&mut chars, line)),
            _ if c.is_ascii_digit() => tokens.push(number_token(&mut chars, line)),
            ' ' | '\r' | '\t' => {
                chars.next(); // Consume whitespace
            }
            '\n' => {
                line += 1;
                chars.next(); // Consume newline
            }
            _ => {
                // Handle unexpected character error.
                chars.next(); // Consume unrecognized character
            }
        }
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: String::new(),
        literal: None,
        line,
    });

    tokens
}

fn single_char_token(token_type: TokenType, chars: &mut Peekable<Chars>, line: u32) -> Token {
    let lexeme = chars.next().unwrap().to_string();
    Token {
        token_type,
        lexeme,
        literal: None,
        line,
    }
}

fn string_token(chars: &mut Peekable<Chars>, mut line: u32) -> Token {
    let mut lexeme = String::new();
    let mut literal = String::new();

    for c in chars.by_ref() {
        match c {
            '"' => {
                return Token {
                    token_type: TokenType::String,
                    lexeme: format!("\"{}\"", lexeme),
                    literal: Some(literal),
                    line,
                };
            }
            '\n' => line += 1,
            _ => {
                lexeme.push(c);
                literal.push(c);
            }
        }
    }

    // Handle unterminated string error.
    Token {
        token_type: TokenType::Eof,
        lexeme: String::new(),
        literal: None,
        line,
    }
}

fn identifier_or_keyword(chars: &mut Peekable<Chars>, line: u32) -> Token {
    let lexeme = take_while(chars, |c| c.is_alphanumeric() || *c == '_');
    let token_type = match lexeme.as_str() {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "fn" => TokenType::Function,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "let" => TokenType::Variable,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    };
    Token {
        token_type,
        lexeme,
        literal: None,
        line,
    }
}

fn number_token(chars: &mut Peekable<Chars>, line: u32) -> Token {
    let mut lexeme = String::new();
    let mut literal = String::new();
    let mut decimal_found = false;

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            lexeme.push(c);
            literal.push(c);
        } else if c == '.' {
            if decimal_found {
                break;
            } else {
                lexeme.push(c);
                literal.push(c);
                decimal_found = true;
            }
        } else {
            break;
        }
        chars.next();
    }

    if let Ok(number) = lexeme.parse::<f64>() {
        Token {
            token_type: TokenType::Number,
            lexeme,
            literal: Some(number.to_string()),
            line,
        }
    } else {
        // Handle number parsing error.
        Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line,
        }
    }
}

fn take_while<I, F>(chars: &mut Peekable<I>, mut condition: F) -> String
    where
        I: Iterator<Item=char>,
        F: FnMut(&char) -> bool,
{
    let mut result = String::new();

    while let Some(&c) = chars.peek() {
        if condition(&c) {
            result.push(c);
            chars.next();
        } else {
            break;
        }
    }

    result
}
