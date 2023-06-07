use crate::lang::lexer::{Token, TokenType};

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
    Print {
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
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;

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
        let name = self.consume(TokenType::Identifier, "Expect function name.")?;

        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;

        let mut parameters = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(Error::UnexpectedToken {
                        token: self.peek(),
                        expected: vec![TokenType::RightParen],
                        message: "Cannot have more than 255 parameters.".to_string(),
                    });
                }

                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;

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
        if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else if self.matches(&[TokenType::LeftBrace]) {
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
                message: "Invalid assignment target.".to_string(),
            });
        }

        Ok(expr)
    }

    /// Parses an `or` expression.
    ///
    /// # Returns
    /// The or expression as a result.
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
    /// The and expression as a result.
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
    /// The equality expression as a result.
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
    /// The comparison expression as a result.
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
    /// The term expression as a result.
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
    /// The factor expression as a result.
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
    /// The unary expression as a result.
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
    /// The call expression as a result.
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
    /// The finish call expression as a result.
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(Error::TooManyArguments {
                        paren: self.peek(),
                        message: "Cannot have more than 255 arguments.".to_string(),
                    });
                }

                arguments.push(self.expression()?);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    /// Parses a `primary` expression.
    ///
    /// # Returns
    /// The primary expression as a result.
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

        if self.matches(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {
                value: Literal::String(self.previous().literal.unwrap()),
            });
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous(),
            });
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping {
                expression: Box::new(expr),
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
            message: "Expect expression.".to_string(),
        })
    }

    /// Parses a block statement.
    ///
    /// # Returns
    /// The block statement as a result.
    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    /// Parse a `print` statement.
    ///
    /// # Returns
    /// The print statement as a result.
    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print {
            expression: value,
        })
    }

    /// Parse an `expression` statement.
    ///
    /// # Returns
    /// The expression statement as a result.
    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Expression {
            expression: value,
        })
    }

    /// Parse an `if` statement.
    ///
    /// # Returns
    /// The if statement as a result.
    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

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
    /// The while statement as a result.
    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let body = self.statement()?;

        Ok(Stmt::While {
            condition: *Box::new(condition),
            body: Box::new(body),
        })
    }

    /// Parse a `for` statement.
    ///
    /// # Returns
    /// The for statement as a result.
    fn for_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

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

        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

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
    /// The return statement as a result.
    fn return_statement(&mut self) -> Result<Stmt, Error> {
        let keyword = self.previous();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return {
            keyword,
            value,
        })
    }
}