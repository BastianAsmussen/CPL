/// An enumeration of all the possible tokens in the language.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    /// A left parenthesis token.
    /// '('
    ///
    /// # Example
    /// ```
    /// // Left parenthesis token is '('.
    /// test_function();
    LeftParenthesis,
    /// A right parenthesis token.
    /// ')'
    ///
    /// # Example
    /// ```
    /// // Right parenthesis token is ')'.
    /// test_function();
    RightParenthesis,
    /// A left curly brace token.
    /// '{'
    ///
    /// # Example
    /// ```
    /// // Left curly brace token is '{'.
    /// fn main() {}
    LeftCurlyBrace,
    /// A right curly brace token.
    /// '}'
    ///
    /// # Example
    /// ```
    /// // Right curly brace token is '}'.
    /// fn main() {}
    /// ```
    RightCurlyBrace,
    /// A semicolon token.
    /// ';'
    ///
    /// # Example
    /// ```
    /// // Semicolon token is ';'.
    /// let a = 6;
    Semicolon,
    /// A comma token.
    /// ','
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    /// // Comma token is ','.
    /// test_function(a, b);
    Comma,
    /// A colon token.
    /// ':'
    /// Used for type annotations.
    ///
    /// # Example
    /// ```
    /// // Colon token is ':'.
    /// let a: i32 = 6;
    /// ```
    Colon,
    /// A plus token.
    /// '+'
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    /// // Plus token is '+'.
    /// let c = a + b;
    Plus,
    /// A minus token.
    /// '-'
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    /// // Minus token is '-'.
    /// let c = a - b;
    Minus,
    /// A star token.
    /// '*'
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    /// // Star token is '*'.
    /// let c = a * b;
    Star,
    /// A slash token.
    /// '/'
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    /// // Slash token is '/'.
    /// let c = a / b;
    Slash,
    /// A percent token.
    /// '%'
    /// Used for the remainder operator.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    /// // Percent token is '%'.
    /// let c = a % b;
    /// ```
    Percent,
    /// A caret token.
    /// '^'
    /// Used for bitwise XOR expressions.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010 ^ 0b1100;
    /// ```
    BitwiseXor,
    /// Ampersand token.
    /// '&'
    /// Used for bitwise AND expressions.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010 & 0b1100;
    /// ```
    BitwiseAnd,
    /// A pipe token.
    /// '|'
    /// Used for bitwise OR expressions.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010 | 0b1100;
    /// ```
    BitwiseOr,
    /// A double ampersand token.
    /// '&&'
    /// Used for logical AND expressions.
    ///
    /// # Example
    /// ```
    /// let a = true && false;
    /// ```
    LogicalAnd,
    /// A double pipe token.
    /// '||'
    /// Used for logical OR expressions.
    ///
    /// # Example
    /// ```
    /// let a = true || false;
    /// ```
    LogicalOr,
    // One or two character tokens.
    /// A bang token.
    /// '!'
    ///
    /// # Example
    /// ```
    /// let a = false;
    ///
    /// // Bang token is '!'.
    /// if !a {
    ///     print("!a = true.");
    /// }
    Bang,
    /// A bang-equal. '!='
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    ///
    /// // Bang-equal token is '!='.
    /// if a != b {
    ///     print("a is not equal to b!");
    /// }
    BangEqual,
    /// An equal token. '='
    ///
    /// # Example
    /// ```
    /// // Equal token is '='.
    /// let a = 6;
    /// ```
    Equal,
    /// An equal-equal token.
    /// '=='
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    ///
    /// // Equal-equal token is '=='.
    /// if a == b {
    ///     print("a is equal to b!");
    /// }
    EqualEqual,
    /// A greater-than token.
    /// '>'
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    ///
    /// // Greater-than token is '>'.
    /// if a > b {
    ///     print("a is greater than b!");
    /// }
    GreaterThan,
    /// A greater-than, or equal to token.
    /// '>='
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    ///
    /// // Greater-than, or equal to token is '>='.
    /// if a >= b {
    ///     print("a is greater than or equal to b!");
    /// }
    GreaterThanOrEqual,
    /// A less-than token.
    /// '<'
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    ///
    /// // Less-than token is '<'.
    /// if a < b {
    ///     print("a is less than b!");
    /// }
    LessThan,
    /// A less than, or equal to token.
    /// '<='
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// let b = 4;
    ///
    /// if a <= b {
    ///     print("a is less than or equal to b!");
    /// }
    /// ```
    LessThanOrEqual,
    /// The increment token.
    /// '++'
    /// Used for incrementing variables.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// a++; // a is now 7.
    /// ```
    Increment,
    /// The decrement token.
    /// '--'
    /// Used for decrementing variables.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// a--; // a is now 5.
    /// ```
    Decrement,
    /// The bitwise left shift token.
    /// '<<'
    /// Used for bitwise left shift expressions.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010 << 2;
    /// ```
    BitwiseLeftShift,
    /// The bitwise right shift token.
    /// '>>'
    /// Used for bitwise right shift expressions.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010 >> 2;
    /// ```
    BitwiseRightShift,
    /// The bitwise right shift equal token.
    /// '>>='
    /// Used for bitwise right shift expressions, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010;
    /// a >>= 2; // a is now 0b10.
    /// ```
    BitwiseRightShiftEqual,
    /// The bitwise left shift equal token.
    /// '<<='
    /// Used for bitwise left shift expressions, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010;
    /// a <<= 2; // a is now 0b101000.
    /// ```
    BitwiseLeftShiftEqual,
    /// The plus-equal token.
    /// '+='
    /// Used for adding a value to a variable, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// a += 4; // a is now 10.
    /// ```
    PlusEqual,
    /// The minus-equal token.
    /// '-='
    /// Used for subtracting a value from a variable, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// a -= 4; // a is now 2.
    /// ```
    MinusEqual,
    /// The star-equal token.
    /// '*='
    /// Used for multiplying a variable by a value, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// a *= 4; // a is now 24.
    /// ```
    StarEqual,
    /// The slash-equal token.
    /// '/='
    /// Used for dividing a variable by a value, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// a /= 4; // a is now 1.5.
    /// ```
    SlashEqual,
    /// The percent-equal token.
    /// '%='
    /// Used for getting the remainder of a variable divided by a value,
    /// and assigning the result to the variable.
    /// Also known as the modulo operator.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// a %= 4; // a is now 2.
    /// ```
    PercentEqual,
    /// The bitwise and-equal token.
    /// '&='
    /// Used for bitwise AND-ing a variable with a value, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010;
    /// a &= 0b1100; // a is now 0b1000.
    /// ```
    BitwiseAndEqual,
    /// The bitwise or-equal token.
    /// '|='
    /// Used for bitwise OR-ing a variable with a value, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010;
    /// a |= 0b1100; // a is now 0b1110.
    /// ```
    BitwiseOrEqual,
    /// The bitwise XOR-equal token.
    /// '^='
    /// Used for bitwise XOR-ing a variable with a value, and assigning the result to the variable.
    ///
    /// # Example
    /// ```
    /// let a = 0b1010;
    /// a ^= 0b1100; // a is now 0b0110.
    /// ```
    BitwiseXorEqual,
    // Literals.
    /// An identifier literal.
    /// Used for variable names, function names, etc.
    ///
    /// # Example
    /// ```
    /// // Identifier is 'a'.
    /// let a = 6;
    ///
    /// // Identifier is 'test_function'.
    /// test_function();
    /// ```
    Identifier,
    /// A string literal.
    /// Used for strings.
    ///
    /// # Example
    /// ```
    /// let text = "Hello, world!";
    /// ```
    String,
    /// A number literal.
    /// Used for numbers.
    ///
    /// # Example
    /// ```
    /// let a = 6.9;
    /// let b = 4.2;
    /// let c = a + b;
    /// ```
    Number,

    // Keywords.
    /// The 'if' keyword.
    /// Used for conditional statements.
    ///
    /// # Example
    /// ```
    /// if true {
    ///     print("Hello, world!");
    /// }
    /// ```
    If,
    /// The 'elif' keyword.
    /// Used for conditional statements.
    ///
    /// # Example
    /// ```
    /// if true {
    ///     print("Hello, world!");
    /// } elif false {
    ///     print("Goodbye, world!");
    /// }
    /// ```
    ElseIf,
    /// The 'else' keyword.
    /// Used for conditional statements.
    ///
    /// # Example
    /// ```
    /// if true {
    ///     print("Hello, world!");
    /// } else {
    ///     print("Goodbye, world!");
    /// }
    /// ```
    Else,
    /// The 'switch' keyword.
    /// Used for conditional statements.
    /// The switch statement is a multi-way branch statement.
    /// It provides an easy way
    /// to dispatch execution to different parts of code based on the value of the expression.
    /// The switch statement evaluates its expression,
    /// then executes all statements that follow the matching case label.
    ///
    /// # Example
    /// ```
    /// switch <expression> {
    ///     case <expression>:
    ///         <statements>
    ///     case <expression>:
    ///         <statements>
    ///     default:
    ///         <statements>
    /// }
    /// ```
    Switch,
    /// The 'case' keyword.
    /// Used for conditional switch statements.
    ///
    /// # Example
    /// ```
    /// switch <expression> {
    ///     case <expression>:
    ///         <statements>
    ///     case <expression>:
    ///         <statements>
    ///     default:
    ///         <statements>
    /// }
    /// ```
    Case,
    /// The 'default' keyword.
    /// Used for conditional switch statements.
    ///
    /// # Example
    /// ```
    /// switch <expression> {
    ///     case <expression>:
    ///         <statements>
    ///     case <expression>:
    ///         <statements>
    ///     default:
    ///         <statements>
    /// }
    /// ```
    Default,
    /// The 'true' keyword.
    /// Used for boolean values.
    ///
    /// # Example
    /// ```
    /// let a = true;
    /// ```
    True,
    /// The 'false' keyword.
    /// Used for boolean values.
    ///
    /// # Example
    /// ```
    /// let a = false;
    /// ```
    False,
    /// The 'none' keyword.
    /// Used for null values.
    ///
    /// # Example
    /// ```
    /// let a = none;
    /// ```
    None,
    /// The 'print' keyword.
    /// Used for printing to the console.
    ///
    /// # Example
    /// ```
    /// print("Hello, world!");
    /// ```
    Print,
    /// The '->' keyword.
    /// Used for function return types.
    ///
    /// # Example
    /// ```
    /// fn test_function() -> f32 {
    ///     return 6.9;
    /// }
    /// ```
    Arrow,
    /// The 'return' keyword.
    /// Used for returning values from functions.
    ///
    /// # Example
    /// ```
    /// fn test_function() -> f32 {
    ///    return 6.9;
    /// }
    /// ```
    Return,
    /// The 'while' keyword.
    /// Used for loops.
    ///
    /// # Example
    /// ```
    /// let a = 0;
    /// while a < 10 {
    ///     print(a++);
    /// }
    /// ```
    While,
    /// The 'for' keyword.
    /// Used for loops.
    ///
    /// # Example
    /// ```
    /// for i in 0 to 10 {
    ///     print(i);
    /// }
    /// ```
    For,
    /// The 'in' keyword.
    /// Used in range-based expressions.
    ///
    /// # Example
    /// ```
    /// for i in 0 to 10 {
    ///     print(i);
    /// }
    /// ```
    In,
    /// The 'to' keyword.
    /// Used in range-based expressions.
    ///
    /// # Example
    /// ```
    /// for i in 0 to 10 {
    ///     print(i);
    /// }
    /// ```
    To,
    /// The 'break' keyword.
    /// Used for breaking out of loops.
    ///
    /// # Example
    /// ```
    /// for i in 0 to 10 {
    ///     if i == 5 {
    ///         break;
    /// }
    /// ```
    Break,
    /// The 'continue' keyword.
    /// Used for skipping to the next iteration of a loop.
    ///
    /// # Example
    /// ```
    /// for i in 0 to 10 {
    ///     if i == 5 {
    ///         continue;
    ///     }
    /// }
    /// ```
    Continue,
    /// The 'fn' keyword.
    /// Used for function declarations.
    ///
    /// # Example
    /// ```
    /// fn test_function() {
    ///     print("Hello, world!");
    /// }
    /// ```
    Function,
    /// The 'let' keyword.
    /// Used for variable declarations.
    ///
    /// # Example
    /// ```
    /// let a = 6;
    /// ```
    Variable,
    /// The 'const' keyword.
    /// Used for constant declarations.
    ///
    /// # Example
    /// ```
    /// const a = 6;
    /// ```
    Constant,
    /// The primitive type tokens.
    /// Indicates the type of a variable, function parameter, or function return type as long as it is a primitive type.
    ///
    /// # Example
    /// ```
    /// let a: i32 = 6;
    /// ```
    PrimitiveType,

    /// Used to represent the end of a file.
    EndOfFile,
}

/// Representation of a literal.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// A string literal.
    String(String),
    /// A number literal.
    Number(f64),
    /// A boolean literal.
    Boolean(bool),
    /// A null literal.
    None,
}

/// Representation of a token, with its type, lexeme, literal, line, and column.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,

    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Creates a new token.
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Option<Literal>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
            column,
        }
    }
}

/// Representation of a scanner.
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    /// Creates a new scanner.
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Scans the source code and returns a vector of tokens.
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EndOfFile,
            "",
            None,
            self.line,
            self.column,
        ));
        self.tokens.clone()
    }

    /// Scans a single token.
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Single-character tokens.
            '(' => self.add_token(TokenType::LeftParenthesis),
            ')' => self.add_token(TokenType::RightParenthesis),
            '{' => self.add_token(TokenType::LeftCurlyBrace),
            '}' => self.add_token(TokenType::RightCurlyBrace),
            ':' => self.add_token(TokenType::Colon),
            ';' => self.add_token(TokenType::Semicolon),
            ',' => self.add_token(TokenType::Comma),

            // One or two character tokens.
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    // Equal to.
                    self.add_token(TokenType::EqualEqual);
                } else {
                    // Assignment.
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    // Less than or equal to.
                    self.add_token(TokenType::LessThanOrEqual);
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        // Bitwise left shift assignment.
                        self.add_token(TokenType::BitwiseLeftShiftEqual);
                    } else {
                        // Bitwise left shift.
                        self.add_token(TokenType::BitwiseLeftShift);
                    }
                } else {
                    // Less than.
                    self.add_token(TokenType::LessThan);
                }
            }
            '>' => {
                if self.match_char('=') {
                    // Greater than or equal to.
                    self.add_token(TokenType::GreaterThanOrEqual);
                } else if self.match_char('>') {
                    if self.match_char('=') {
                        // Bitwise right shift assignment.
                        self.add_token(TokenType::BitwiseRightShiftEqual);
                    } else {
                        // Bitwise right shift.
                        self.add_token(TokenType::BitwiseRightShift);
                    }
                } else {
                    // Greater than.
                    self.add_token(TokenType::GreaterThan);
                }
            }
            '+' => {
                if self.match_char('+') {
                    // Increment.
                    self.add_token(TokenType::Increment);
                } else if self.match_char('=') {
                    // Addition assignment.
                    self.add_token(TokenType::PlusEqual);
                } else {
                    // Addition.
                    self.add_token(TokenType::Plus);
                }
            }
            '-' => {
                if self.match_char('-') {
                    // Decrement.
                    self.add_token(TokenType::Decrement);
                } else if self.match_char('=') {
                    // Subtraction assignment.
                    self.add_token(TokenType::MinusEqual);
                } else if self.match_char('>') {
                    // Arrow.
                    self.add_token(TokenType::Arrow);
                } else {
                    // Subtraction.
                    self.add_token(TokenType::Minus); // Subtraction
                }
            }
            '*' => {
                if self.match_char('=') {
                    // Multiplication assignment.
                    self.add_token(TokenType::StarEqual);
                } else {
                    // Multiplication.
                    self.add_token(TokenType::Star);
                }
            }
            '%' => {
                if self.match_char('=') {
                    // Modulo assignment.
                    self.add_token(TokenType::PercentEqual);
                } else {
                    // Modulo.
                    self.add_token(TokenType::Percent);
                }
            }
            '&' => {
                if self.match_char('&') {
                    // Logical AND.
                    self.add_token(TokenType::LogicalAnd);
                } else if self.match_char('=') {
                    // Bitwise AND assignment.
                    self.add_token(TokenType::BitwiseAndEqual);
                } else {
                    // Bitwise AND.
                    self.add_token(TokenType::BitwiseAnd);
                }
            }
            '|' => {
                if self.match_char('|') {
                    // Logical OR.
                    self.add_token(TokenType::LogicalOr);
                } else if self.match_char('=') {
                    // Bitwise OR assignment.
                    self.add_token(TokenType::BitwiseOrEqual);
                } else {
                    // Bitwise OR.
                    self.add_token(TokenType::BitwiseOr);
                }
            }
            '^' => {
                if self.match_char('=') {
                    // Bitwise XOR assignment.
                    self.add_token(TokenType::BitwiseXorEqual);
                } else {
                    // Bitwise XOR.
                    self.add_token(TokenType::BitwiseXor);
                }
            }

            // Literals.
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            // Whitespace.
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
                self.column = 1;
            }

            '/' => {
                if self.match_char('/') {
                    // Single-line comments.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    // Multi-line comments.
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                            self.column = 1;
                        }
                        self.advance();
                    }

                    if self.is_at_end() {
                        panic!("Scanner tried to advance past the end of the source code!");
                    }

                    self.advance();
                    self.advance();
                } else if self.match_char('=') {
                    // Division assignment.
                    self.add_token(TokenType::SlashEqual);
                } else {
                    // Division.
                    self.add_token(TokenType::Slash);
                }
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    /// Advances the scanner by one character.
    ///
    /// # Returns
    /// The character that was advanced to.
    ///
    /// # Panics
    /// Panics if the scanner tries to advance past the end of the source code.
    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;

        self.source
            .chars()
            .nth(self.current - 1)
            .expect("Scanner tried to advance past the end of the source code!")
    }

    /// Adds a token to the vector of tokens.
    ///
    /// # Arguments
    /// * `token_type` - The type of the token.
    ///
    ///
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        let literal = match token_type {
            TokenType::String => Some(Literal::String(text.clone())),
            TokenType::Number => Some(Literal::Number(text.parse().unwrap())),
            TokenType::True => Some(Literal::Boolean(true)),
            TokenType::False => Some(Literal::Boolean(false)),
            TokenType::None => Some(Literal::None),
            _ => None,
        };

        self.tokens.push(Token::new(
            token_type,
            text.as_str(),
            literal,
            self.line,
            self.column,
        ));
    }

    /// Checks if the next character matches the given character.
    ///
    /// # Arguments
    /// * `expected` - The character to match.
    ///
    /// # Returns
    /// True if the next character matches the given character, false otherwise.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        self.column += 1;

        true
    }

    /// Checks if we are at the end of the source code.
    ///
    /// # Returns
    /// True if we are at the end of the source code, false otherwise.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Peeks at the next character without advancing the scanner.
    ///
    /// # Returns
    /// The next character.
    ///
    /// # Panics
    /// Panics if the scanner tries to peek past the end of the source code.
    fn peek(&self) -> char {
        self.source
            .chars()
            .nth(self.current)
            .expect("Scanner tried to peek past the end of the source code!")
    }

    /// Peeks at the character after the next character without advancing the scanner.
    ///
    /// # Returns
    /// The character after the next character.
    ///
    /// # Panics
    /// Panics if the scanner tries to peek past the end of the source code.
    fn peek_next(&self) -> char {
        self.source
            .chars()
            .nth(self.current + 1)
            .expect("Scanner tried to peek past the end of the source code!")
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string!");
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String);

        self.tokens.last_mut().unwrap().literal = Some(Literal::String(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token(TokenType::Number);

        self.tokens.last_mut().unwrap().literal = Some(Literal::Number(value.parse().unwrap()));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = match text.as_str() {
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "elif" => TokenType::ElseIf,
            "else" => TokenType::Else,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "default" => TokenType::Default,
            "while" => TokenType::While,
            "continue" => TokenType::Continue,
            "break" => TokenType::Break,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "to" => TokenType::To,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "none" => TokenType::None,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "let" => TokenType::Variable,
            "const" => TokenType::Constant,
            | "i8" | "i16" | "i32" | "i64"
            | "u8" | "u16" | "u32" | "u64"
            | "f32" | "f64"
            | "bool" | "char" | "str" => TokenType::PrimitiveType,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }
}
