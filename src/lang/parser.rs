use std::fmt::{Display, Formatter};

use crate::lang::errors::{report, Error};
use crate::lang::lexer::{Literal, Token, TokenType};
use crate::lang::{MAX_ARGUMENTS, MAX_PARAMETERS};

/// An expression is a piece of code that evaluates to a value.
#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable(Token),
    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        parenthesis: Token,
        arguments: Vec<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expression::Grouping(expression) => write!(f, "(group {})", expression),
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Unary { operator, right } => write!(f, "({} {})", operator.lexeme, right),
            Expression::Variable(name) => write!(f, "{}", name.lexeme),
            Expression::Assign { name, value } => write!(f, "(= {} {})", name.lexeme, value),
            Expression::Call {
                callee,
                parenthesis: _parenthesis,
                arguments,
            } => {
                write!(f, "({}(", callee)?;

                for (i, argument) in arguments.iter().enumerate() {
                    write!(f, "{}", argument)?;

                    if i != arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "))")
            }
        }
    }
}

/// A statement is a piece of code that does not evaluate to a value.
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Variable {
        name: Token,
        initializer: Option<Expression>,
    },
    Block(Vec<Statement>),
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Box<Statement>,
    },
    Function {
        name: Token,
        parameters: Vec<(Token, Token)>,
        body: Box<Statement>,
    },
    Return {
        keyword: Token,
        value: Option<Expression>,
    },
    Break {
        keyword: Token,
    },
    Continue {
        keyword: Token,
    },
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(expression) => write!(f, "{}", expression),
            Statement::Print(expression) => write!(f, "(print {})", expression),
            Statement::Variable { name, initializer } => {
                if let Some(initializer) = initializer {
                    write!(f, "(var {} {})", name.lexeme, initializer)
                } else {
                    write!(f, "(var {})", name.lexeme)
                }
            }
            Statement::Block(statements) => {
                write!(f, "(block ")?;

                for (i, statement) in statements.iter().enumerate() {
                    write!(f, "{}", statement)?;

                    if i != statements.len() - 1 {
                        write!(f, " ")?;
                    }
                }

                write!(f, ")")
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(f, "(if {} {} ", condition, then_branch)?;

                if let Some(else_branch) = else_branch {
                    write!(f, "{}", else_branch)?;
                }

                write!(f, ")")
            }
            Statement::While { condition, body } => write!(f, "(while {} {})", condition, body),
            Statement::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                write!(f, "(for ")?;

                if let Some(initializer) = initializer {
                    write!(f, "{} ", initializer)?;
                }

                if let Some(condition) = condition {
                    write!(f, "{} ", condition)?;
                }

                if let Some(increment) = increment {
                    write!(f, "{} ", increment)?;
                }

                write!(f, "{}", body)?;

                write!(f, ")")
            }
            Statement::Function {
                name,
                parameters,
                body,
            } => {
                write!(f, "(fn {}(", name.lexeme)?;

                for (i, (parameter, _)) in parameters.iter().enumerate() {
                    write!(f, "{}", parameter.lexeme)?;

                    if i != parameters.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ") {})", body)
            }
            Statement::Return { keyword, value } => {
                if let Some(value) = value {
                    write!(f, "(ret {} {})", keyword.lexeme, value)
                } else {
                    write!(f, "(ret {})", keyword.lexeme)
                }
            }
            Statement::Break { keyword } => write!(f, "(break {})", keyword.lexeme),
            Statement::Continue { keyword } => write!(f, "(continue {})", keyword.lexeme),
        }
    }
}

impl Iterator for Box<Statement> {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self.clone())
    }
}

/// A parser for the CPL language.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,

    errors: Vec<Error>,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,

            errors: Vec::new(),
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.had_error {
                break;
            }

            statements.push(self.declaration());
        }

        if self.had_error {
            Err(self.errors.clone())
        } else {
            Ok(statements)
        }
    }

    fn declaration(&mut self) -> Statement {
        if self.matches(&[TokenType::Variable]) {
            self.variable_declaration()
        } else if self.matches(&[TokenType::Function]) {
            self.function_declaration()
        } else {
            *self.statement()
        }
    }

    fn expression(&mut self) -> Expression {
        self.assignment()
    }

    fn assignment(&mut self) -> Expression {
        let expression = self.or();

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment();

            match expression {
                Expression::Variable(name) => {
                    return Expression::Assign {
                        name,
                        value: Box::new(value),
                    };
                }
                _ => {
                    self.error(&equals, "Invalid assignment target!");
                }
            }
        }

        expression
    }

    fn or(&mut self) -> Expression {
        let mut expression = self.and();

        while self.matches(&[TokenType::LogicalOr]) {
            let operator = self.previous().clone();
            let right = self.and();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        expression
    }

    fn and(&mut self) -> Expression {
        let mut expression = self.equality();

        while self.matches(&[TokenType::LogicalAnd]) {
            let operator = self.previous().clone();
            let right = self.equality();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        expression
    }

    fn equality(&mut self) -> Expression {
        let mut expression = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        expression
    }

    fn comparison(&mut self) -> Expression {
        let mut expression = self.term();

        while self.matches(&[
            TokenType::GreaterThan,
            TokenType::GreaterThanOrEqual,
            TokenType::LessThan,
            TokenType::LessThanOrEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        expression
    }

    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();

            Expression::Unary {
                operator,
                right: Box::new(right),
            }
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Expression {
        let mut expression = self.primary();

        loop {
            if self.matches(&[TokenType::LeftParenthesis]) {
                expression = self.finish_call(expression);
            } else {
                break;
            }
        }

        expression
    }

    fn primary(&mut self) -> Expression {
        if self.matches(&[TokenType::False]) {
            Expression::Literal(Literal::Boolean(false))
        } else if self.matches(&[TokenType::True]) {
            Expression::Literal(Literal::Boolean(true))
        } else if self.matches(&[TokenType::None]) {
            Expression::Literal(Literal::None)
        } else if self.matches(&[TokenType::Number, TokenType::String]) {
            let previous = self.previous().clone();
            let literal = previous.literal.clone();
            if literal.is_none() {
                self.error(&previous, "Expected literal!");
            }

            Expression::Literal(literal.unwrap())
        } else if self.matches(&[TokenType::Identifier]) {
            Expression::Variable(self.previous().clone())
        } else if self.matches(&[TokenType::LeftParenthesis]) {
            let expression = self.expression();
            self.consume(
                TokenType::RightParenthesis,
                "Expected ')' after expression!",
            );
            Expression::Grouping(Box::new(expression))
        } else {
            self.error(&self.peek().clone(), "Expected expression!");
            Expression::Literal(Literal::None)
        }
    }

    fn finish_call(&mut self, callee: Expression) -> Expression {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParenthesis) {
            loop {
                if arguments.len() >= MAX_ARGUMENTS {
                    self.error(
                        &self.peek().clone(),
                        &format!("Cannot have more than {} arguments.", MAX_ARGUMENTS),
                    );
                }

                arguments.push(self.expression());

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let parenthesis =
            self.consume(TokenType::RightParenthesis, "Expected ')' after arguments.");

        Expression::Call {
            callee: Box::new(callee),
            parenthesis,
            arguments,
        }
    }

    fn variable_declaration(&mut self) -> Statement {
        let name = self.consume(TokenType::Identifier, "Expected variable name.");

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        );

        Statement::Variable { name, initializer }
    }

    fn function_declaration(&mut self) -> Statement {
        let name = self.consume(TokenType::Identifier, "Expected function name.");
        let parameters = self.function_parameters();
        let body = self.block();

        Statement::Function {
            name,
            parameters,
            body,
        }
    }

    fn function_parameters(&mut self) -> Vec<(Token, Token)> {
        self.consume(
            TokenType::LeftParenthesis,
            "Expected '(' after function name.",
        );

        let mut parameters = Vec::new();

        if !self.check(&TokenType::RightParenthesis) {
            loop {
                if parameters.len() >= MAX_PARAMETERS {
                    self.error(
                        &self.peek().clone(),
                        &format!("Cannot have more than {} parameters.", MAX_PARAMETERS),
                    );
                }

                let identifier = self.consume(TokenType::Identifier, "Expected parameter name.");
                let r#type = if self.matches(&[TokenType::Colon]) {
                    Some(self.consume(TokenType::Identifier, "Expected type name."))
                } else {
                    None
                };
                if r#type.is_none() {
                    self.error(&self.peek().clone(), "Expected type name.");
                }

                parameters.push((identifier, r#type.unwrap()));

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParenthesis,
            "Expected ')' after parameters.",
        );

        parameters
    }

    fn block(&mut self) -> Box<Statement> {
        let mut statements = Vec::new();

        self.consume(TokenType::LeftCurlyBrace, "Expected '{' before block.");
        while !self.check(&TokenType::RightCurlyBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RightCurlyBrace, "Expected '}' after block.");

        Box::new(Statement::Block(statements))
    }

    fn statement(&mut self) -> Box<Statement> {
        if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else if self.matches(&[TokenType::Return]) {
            self.return_statement()
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else if self.matches(&[TokenType::For]) {
            self.for_statement()
        } else if self.matches(&[TokenType::Break]) {
            self.break_statement()
        } else if self.matches(&[TokenType::Continue]) {
            self.continue_statement()
        } else if self.matches(&[TokenType::LeftCurlyBrace]) {
            Box::new(*self.block())
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Box<Statement> {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expected ';' after value.");

        Box::new(Statement::Print(value))
    }

    fn return_statement(&mut self) -> Box<Statement> {
        let keyword = self.previous().clone();
        let value = if !self.check(&TokenType::Semicolon) {
            self.expression()
        } else {
            Expression::Literal(Literal::None)
        };
        self.consume(TokenType::Semicolon, "Expected ';' after return value.");

        Box::new(Statement::Return {
            keyword,
            value: Some(value),
        })
    }

    fn if_statement(&mut self) -> Box<Statement> {
        self.consume(TokenType::LeftParenthesis, "Expected '(' after 'if'.");
        let condition = self.expression();
        self.consume(
            TokenType::RightParenthesis,
            "Expected ')' after if condition.",
        );

        let then_branch = self.statement();
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(self.statement())
        } else {
            None
        };

        Box::new(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Box<Statement> {
        self.consume(TokenType::LeftParenthesis, "Expected '(' after 'while'.");
        let condition = self.expression();
        self.consume(
            TokenType::RightParenthesis,
            "Expected ')' after while condition.",
        );

        let body = self.statement();

        Box::new(Statement::While { condition, body })
    }

    fn for_statement(&mut self) -> Box<Statement> {
        self.consume(TokenType::LeftParenthesis, "Expected '(' after 'for'.");

        // We need an initializer, but it can be empty.
        // An initializer can be a variable declaration or an expression statement.
        // It basically means that we can have a variable declaration, an expression, or nothing.
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Variable]) {
            Some(self.variable_declaration())
        } else {
            Some(*self.expression_statement())
        };

        if let Some(_initializer) = &initializer {
            self.consume(TokenType::Semicolon, "Expected ';' after for initializer.");
        }

        // We need a _condition, but it can be empty.
        // A condition can be an expression or nothing.
        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };

        // Evaluate the condition, but don't consume the semicolon.
        // We need to consume the semicolon in the increment clause.
        if let Some(_condition) = &condition {
            self.consume(TokenType::Semicolon, "Expected ';' after loop condition.");
        }

        let increment = if !self.check(&TokenType::RightParenthesis) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(
            TokenType::RightParenthesis,
            "Expected ')' after for clauses.",
        );

        let mut body = self.statement();

        if let Some(increment) = increment {
            body = Box::new(Statement::Block(vec![
                *body,
                Statement::Expression(increment),
            ]));
        }

        body
    }

    fn break_statement(&mut self) -> Box<Statement> {
        let keyword = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expected ';' after 'break'.");

        Box::new(Statement::Break { keyword })
    }

    fn continue_statement(&mut self) -> Box<Statement> {
        let keyword = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expected ';' after 'continue'.");

        Box::new(Statement::Continue { keyword })
    }

    fn expression_statement(&mut self) -> Box<Statement> {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expected ';' after expression.");

        Box::new(Statement::Expression(value))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(&token_type) {
            self.advance().clone()
        } else {
            let token = self.peek().clone();
            self.error(&token, message);

            token
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EndOfFile
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn error(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::EndOfFile {
            report(token.line, token.column, &format!("{} at end", message));
        } else {
            report(
                token.line,
                token.column,
                &format!("{} at '{}'", token.lexeme, message),
            );
        }

        if !self.had_error {
            self.had_error = true;
        } else {
            panic!("Too many errors!");
        }
    }
}
