use crate::lang::lexer::{Token, TokenType};
use crate::lang::parser::Stmt::Expression;

/// The maximum number of arguments a function can have.
pub const MAX_ARGS: usize = 255;

/// The maximum number of parameters a function can have.
pub const MAX_PARAMS: usize = 255;

/// An enumeration of the different types of expressions.
#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
}

/// An enumeration of the different types of literals.
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

/// An enumeration of the different types of statements.
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Variable {
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        parameters: Vec<Token>,
        body: Box<Stmt>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
}

/// An enumeration of the different types of errors.
#[derive(Debug, Clone)]
pub enum Error {
    /// An error that occurs when the parser encounters an unexpected token.
    UnexpectedToken {
        /// The token that was encountered.
        token: Token,
        /// The expected token types.
        expected: Vec<TokenType>,
        /// The error message.
        message: String,
    },
    /// An error that occurs when the parser encounters an unexpected end of file.
    UnexpectedEof {
        /// The expected token types.
        expected: Vec<TokenType>,
        /// The error message.
        message: String,
    },
    /// An error that occurs when the parser encounters too many arguments.
    TooManyArguments {
        /// The token that was encountered.
        paren: Token,
        /// The error message.
        message: String,
    },
    /// An error that occurs when a variable is not defined.
    UndefinedVariable {
        /// The token that was encountered.
        name: Token,
        /// The error message.
        message: String,
    },
}

/// A structure that contains the tokens and the current index.
#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Checks if we're at the end of the tokens.
    ///
    /// # Returns
    /// True if we're at the end of the tokens, false otherwise.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Peeks at the next token.
    ///
    /// # Returns
    /// The next token.
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Finds the previous token in the list.
    ///
    /// # Returns
    /// The previous token.
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    /// Finds the token `n` tokens back in the list.
    ///
    /// # Arguments
    /// * `n` - The number of tokens back to go.
    ///
    /// # Returns
    /// The token `n` tokens back.
    fn backward(&self, n: usize) -> Token {
        if self.current < n {
            self.tokens[0].clone()
        } else {
            self.tokens[self.current - n].clone()
        }
    }

    /// Finds the token `n` tokens forward in the list.
    ///
    /// # Arguments
    /// * `n` - The number of tokens forward to go.
    ///
    /// # Returns
    /// The token `n` tokens forward.
    fn forward(&self, n: usize) -> Token {
        if self.current + n >= self.tokens.len() {
            self.tokens[self.tokens.len() - 1].clone()
        } else {
            self.tokens[self.current + n].clone()
        }
    }

    /// Finds the current token in the list.
    ///
    /// # Returns
    /// The current token.
    fn current(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Advances to the next token.
    ///
    /// # Returns
    /// The previous token before advancing.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /// Checks if the current token matches the given type.
    ///
    /// # Arguments
    /// * `token_type` - The token type to check.
    ///
    /// # Returns
    /// True if the current token matches the given type, false otherwise.
    fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Checks if the current token matches the given type.
    ///
    /// # Arguments
    /// * `token_type` - The token type to check.
    ///
    /// # Returns
    /// True if the current token matches the given type, false otherwise.
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
    }

    /// Consumes the current token if it matches the given type.
    ///
    /// # Arguments
    /// * `token_type` - The token type to check.
    /// * `message` - The error message to display if the token doesn't match.
    ///
    /// # Returns
    /// The consumed token as a result.
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(Error::UnexpectedToken {
                token: self.peek(),
                expected: vec![token_type],
                message: message.to_string(),
            })
        }
    }

    /// Creates a new parser from the given tokens.
    ///
    /// # Arguments
    /// * `tokens` - The tokens to parse.
    ///
    /// # Returns
    /// The new parser instance.
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    /// Parses the tokens into a list of statements.
    ///
    /// # Returns
    /// The list of statements.
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    /// Parses a declaration statement.
    ///
    /// # Returns
    /// The declaration statement as a result.
    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.matches(&[TokenType::Variable]) {
            self.variable_declaration()
        } else if self.matches(&[TokenType::Function]) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }

    /// Parses a variable declaration statement.
    ///
    /// # Returns
    /// The variable declaration statement as a result.
    fn variable_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, &format!("Expected variable name! ({}:{})", self.previous().line, self.previous().column))?;

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, &format!("Expected ';' after variable declaration! ({}:{})", self.previous().line, self.previous().column))?;

        Ok(Stmt::Variable {
            name,
            initializer,
        })
    }

    /// Parses a function declaration statement.
    ///
    /// # Returns
    /// The function declaration statement as a result.
    fn function_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, &format!("Expected function name! ({}:{})", self.previous().line, self.previous().column))?;

        self.consume(TokenType::LeftParen, &format!("Expected '(' after function name! ({}:{})", self.previous().line, self.previous().column))?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= MAX_PARAMS {
                    return Err(Error::UnexpectedToken {
                        token: self.peek(),
                        expected: vec![TokenType::RightParen],
                        message: format!("Cannot have more than {} parameters! ({}:{})", MAX_PARAMS, self.peek().line, self.peek().column),
                    });
                }

                parameters.push(self.consume(TokenType::Identifier, &format!("Expected parameter name! ({}:{})", self.previous().line, self.previous().column))?);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, &format!("Expected ')' after function parameters! ({}:{})", self.previous().line, self.previous().column))?;
        self.consume(TokenType::LeftBrace, &format!("Expected '{{' before function body! ({}:{})", self.previous().line, self.previous().column))?;

        let body = self.block()?;

        Ok(Stmt::Function {
            name,
            parameters,
            body: Box::new(Stmt::Block { statements: body }),
        })
    }

    /// Parses a statement.
    ///
    /// # Returns
    /// The statement as a result.
    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.matches(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else if self.matches(&[TokenType::For]) {
            self.for_statement()
        } else if self.matches(&[TokenType::Return]) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    /// Parses an expression.
    ///
    /// # Returns
    /// The expression as a result.
    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    /// Parses an assignment expression.
    ///
    /// # Returns
    /// The assignment expression as a result.
    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.or()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(Error::UnexpectedToken {
                token: equals,
                expected: vec![TokenType::Identifier],
                message: format!("Invalid assignment target! ({}:{})", self.peek().line, self.peek().column),
            });
        }

        Ok(expr)
    }

    /// Parses an `or` expression.
    ///
    /// # Returns
    /// The `or` expression as a result.
    fn or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses an `and` expression.
    ///
    /// # Returns
    /// The `and` expression as a result.
    fn and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses an `equality` expression.
    ///
    /// # Returns
    /// The `equality` expression as a result.
    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a `comparison` expression.
    ///
    /// # Returns
    /// The `comparison` expression as a result.
    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.matches(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a `term` expression.
    ///
    /// # Returns
    /// The `term` expression as a result.
    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a `factor` expression.
    ///
    /// # Returns
    /// The `factor` expression as a result.
    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a `unary` expression.
    ///
    /// # Returns
    /// The `unary` expression as a result.
    fn unary(&mut self) -> Result<Expr, Error> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.call()
    }

    /// Parses a `call` expression.
    ///
    /// # Returns
    /// The `call` expression as a result.
    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parses a `finish_call` expression.
    ///
    /// # Returns
    /// The `finish call` expression as a result.
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments = Vec::new();
        self.load_arguments(&mut arguments)?;

        // Advance to the closing parenthesis.
        let paren = self.consume(TokenType::RightParen, &format!("Expected ')' after function arguments! ({}:{})", self.peek().line, self.peek().column))?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    /// Parses a `primary` expression.
    ///
    /// # Returns
    /// The `primary` expression as a result.
    fn primary(&mut self) -> Result<Expr, Error> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(false),
            });
        }

        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(true),
            });
        }

        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if self.matches(&[TokenType::Number]) {
            return Ok(Expr::Literal {
                value: Literal::Number(self.previous().lexeme.parse().unwrap()),
            });
        }

        if self.matches(&[TokenType::String]) {
            return Ok(Expr::Literal {
                value: Literal::String(self.previous().lexeme),
            });
        }

        // Function calls.
        if self.matches(&[TokenType::Identifier]) {
            if self.peek().token_type != TokenType::LeftParen {
                return Err(Error::UnexpectedToken {
                    token: self.peek(),
                    expected: vec![TokenType::LeftParen],
                    message: format!("Expected '(' after identifier! ({}:{})", self.peek().line, self.peek().column),
                });
            }

            // It's likely to be an argument if it's another identifier.
            if self.peek().token_type == TokenType::Identifier {
                // Handle the argument.
                return Ok(Expr::Variable {
                    name: self.advance(),
                });
            }

            // Load the arguments.
            let mut arguments = Vec::new();
            println!("Current: {}", self.current);
            self.load_arguments(&mut arguments)?;
            println!("Current: {}", self.current);

            let name = self.backward(arguments.len());

            // Advance to the closing parenthesis.
            let paren = self.consume(TokenType::RightParen, &format!("Expected ')' after function arguments! ({}:{})", self.peek().line, self.peek().column))?;

            return Ok(Expr::Call {
                callee: Box::new(Expr::Variable {
                    name,
                }),
                paren,
                arguments,
            });
        }

        Err(Error::UnexpectedToken {
            token: self.peek(),
            expected: vec![
                TokenType::False,
                TokenType::True,
                TokenType::Nil,
                TokenType::Number,
                TokenType::String,
                TokenType::Identifier,
                TokenType::LeftParen,
            ],
            message: format!("Expected expression! ({}:{})", self.peek().line, self.peek().column),
        })
    }

    /// Parses a block statement.
    ///
    /// # Returns
    /// The `block` statement as a result.
    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, &format!("Expected '}}' after block! ({}:{})", self.peek().line, self.peek().column))?;

        Ok(statements)
    }

    /// Parse an `expression` statement.
    ///
    /// # Returns
    /// The `expression` statement as a result.
    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, &format!("Expected ';' after expression! ({}:{})", self.peek().line, self.peek().column))?;

        Ok(Stmt::Expression {
            expression: expr,
        })
    }

    /// Parse an `if` statement.
    ///
    /// # Returns
    /// The `if` statement as a result.
    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, &format!("Expected '(' after 'if'! ({}:{})", self.peek().line, self.peek().column))?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, &format!("Expected ')' after if condition! ({}:{})", self.peek().line, self.peek().column))?;

        let then_branch = self.statement()?;
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition: *Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    /// Parse a `while` statement.
    ///
    /// # Returns
    /// The `while` statement as a result.
    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, &format!("Expected '(' after 'while'! ({}:{})", self.peek().line, self.peek().column))?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, &format!("Expected ')' after while condition! ({}:{})", self.peek().line, self.peek().column))?;

        let body = self.statement()?;

        Ok(Stmt::While {
            condition: *Box::new(condition),
            body: Box::new(body),
        })
    }

    /// Parse a `for` statement.
    ///
    /// # Returns
    /// The `for` statement as a result.
    fn for_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, &format!("Expected '(' after 'for'! ({}:{})", self.peek().line, self.peek().column))?;

        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Variable]) {
            Some(self.variable_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, &format!("Expected ';' after loop condition! ({}:{})", self.peek().line, self.peek().column))?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::RightParen, &format!("Expected ')' after for clauses! ({}:{})", self.peek().line, self.peek().column))?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expression: increment,
                    },
                ],
            };
        }

        body = Stmt::While {
            condition: *Box::new(condition.unwrap_or(Expr::Literal {
                value: Literal::Boolean(true),
            })),
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![
                    initializer,
                    body,
                ],
            };
        }

        Ok(body)
    }

    /// Parse a `return` statement.
    ///
    /// # Returns
    /// The `return` statement as a result.
    fn return_statement(&mut self) -> Result<Stmt, Error> {
        let keyword = self.previous();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, &format!("Expected ';' after return value! ({}:{})", self.peek().line, self.peek().column))?;

        Ok(Stmt::Return {
            keyword,
            value,
        })
    }

    fn load_arguments(&mut self, arguments: &mut Vec<Expr>) -> Result<Vec<Expr>, Error> {
        let mut current = self.current + 1;

        while self.forward(current).token_type != TokenType::RightParen {
            arguments.push(self.expression()?);
            current += 1;

            if !self.matches(&[TokenType::Comma]) {
                break;
            }
        }

        if arguments.len() > MAX_ARGS {
            return Err(Error::TooManyArguments {
                paren: self.previous(),
                message: format!("Cannot have more than {} arguments! ({}:{})", MAX_ARGS, self.peek().line, self.peek().column),
            });
        }

        // Update the current token.
        self.current = current;

        Ok(arguments.to_vec())
    }
}
