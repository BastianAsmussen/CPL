use std::ops::Add;
use super::lexer::{Token, TokenType};

/// An expression in the abstract syntax tree (AST).
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: String,
    },
    Grouping {
        expression: Box<Expr>,
    },
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut tokens_iter = tokens.into_iter().peekable();
    parse_expression(&mut tokens_iter)
}

fn parse_expression(tokens: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Expr, String> {
    parse_binary_expression(tokens, Precedence::Assignment)
}

fn parse_binary_expression(tokens: &mut std::iter::Peekable<std::vec::IntoIter<Token>>, precedence: Precedence) -> Result<Expr, String> {
    let mut left = parse_unary_expression(tokens)?;

    while let Some(token) = tokens.peek().cloned() {
        let operator_precedence = get_precedence(&token.token_type);
        if operator_precedence < precedence {
            break;
        }

        tokens.next();

        let right = parse_binary_expression(tokens, operator_precedence + 1)?;

        left = Expr::Binary {
            left: Box::new(left),
            operator: token,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_unary_expression(tokens: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Expr, String> {
    if let Some(token) = tokens.peek().cloned() {
        match token.token_type {
            TokenType::Minus | TokenType::Bang => {
                tokens.next();
                let operator = token;
                let right = parse_unary_expression(tokens)?;
                Ok(Expr::Unary {
                    operator,
                    right: Box::new(right),
                })
            }
            _ => parse_literal(tokens),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}

fn parse_literal(tokens: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Expr, String> {
    if let Some(token) = tokens.next() {
        match token.token_type {
            TokenType::Number | TokenType::String => {
                Ok(Expr::Literal {
                    value: token.lexeme,
                })
            }
            TokenType::LeftParen => {
                let expression = parse_expression(tokens)?;
                if let Some(token) = tokens.next() {
                    if token.token_type != TokenType::RightParen {
                        return Err(format!("Expected ')' after expression, found '{}'", token.lexeme));
                    }
                } else {
                    return Err("Unexpected end of input".to_string());
                }
                Ok(Expr::Grouping {
                    expression: Box::new(expression),
                })
            }
            _ => Err(format!("Unexpected token: '{}'", token.lexeme)),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Precedence {
    Assignment,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Add<u32> for Precedence {
    type Output = Precedence;

    fn add(self, rhs: u32) -> Precedence {
        match self {
            Precedence::Assignment => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => self,
        }
    }
}

fn get_precedence(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::EqualEqual | TokenType::BangEqual => Precedence::Equality,
        TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => Precedence::Comparison,
        TokenType::Plus | TokenType::Minus => Precedence::Term,
        TokenType::Slash | TokenType::Star => Precedence::Factor,
        _ => Precedence::Primary,
    }
}
