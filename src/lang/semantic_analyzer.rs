use std::fmt::{Display, Formatter};

use crate::lang::parser::{Expr, Literal, Stmt};

/// A structure that represents the semantic analyzer.
pub struct Analyzer {}

impl Analyzer {
    /// Performs semantic analysis on the given AST.
    ///
    /// # Arguments
    /// * `statements` - The list of statements in the AST.
    ///
    /// # Returns
    /// A `Result` containing the analyzed AST or an error message.
    pub fn analyze(statements: &[Stmt]) -> Result<(), String> {
        let mut environment = Environment::new();

        for statement in statements {
            if let Err(error) = Self::analyze_statement(statement, &mut environment) {
                return Err(error.to_string());
            }
        }

        Ok(())
    }

    /// Analyzes a single statement in the AST.
    ///
    /// # Arguments
    /// * `statement` - The statement to analyze.
    /// * `environment` - The current environment.
    ///
    /// # Returns
    /// A result indicating success or failure of the analysis.
    fn analyze_statement(statement: &Stmt, environment: &mut Environment) -> Result<(), Error> {
        match statement {
            Stmt::Expression { expression } => Self::analyze_expression(expression, environment),
            Stmt::Variable { name, initializer } => {
                environment.define(name.lexeme.clone(), initializer.is_some())?;
                if let Some(initializer) = initializer {
                    Self::analyze_expression(initializer, environment)?;
                }
                Ok(())
            }
            Stmt::Block { statements } => {
                environment.begin_scope();
                for statement in statements {
                    Self::analyze_statement(statement, environment)?;
                }
                environment.end_scope();
                Ok(())
            }
            Stmt::If { condition, then_branch, else_branch } => {
                Self::analyze_expression(condition, environment)?;
                Self::analyze_statement(then_branch, environment)?;
                if let Some(else_branch) = else_branch {
                    Self::analyze_statement(else_branch, environment)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                Self::analyze_expression(condition, environment)?;
                Self::analyze_statement(body, environment)?;
                Ok(())
            }
            Stmt::Function { name, parameters, body } => {
                environment.define(name.lexeme.clone(), false)?;
                environment.begin_scope();
                for param in parameters {
                    environment.define(param.lexeme.clone(), false)?;
                }
                Self::analyze_statement(body, environment)?;
                environment.end_scope();
                Ok(())
            }
            Stmt::Return { keyword, value } => {
                if let Some(value) = value {
                    Self::analyze_expression(value, environment)?;
                }
                Ok(())
            }
        }
    }

    /// Analyzes a single expression in the AST.
    ///
    /// # Arguments
    /// * `expression` - The expression to analyze.
    /// * `environment` - The current environment.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the analysis.
    fn analyze_expression(expression: &Expr, environment: &mut Environment) -> Result<(), Error> {
        match expression {
            Expr::Binary { left, right, .. } => {
                Self::analyze_expression(left, environment)?;
                Self::analyze_expression(right, environment)?;
                Ok(())
            }
            Expr::Logical { left, right, .. } => {
                Self::analyze_expression(left, environment)?;
                Self::analyze_expression(right, environment)?;
                Ok(())
            }
            Expr::Grouping { expression } => {
                Self::analyze_expression(expression, environment)?;
                Ok(())
            }
            Expr::Literal { value } => match value {
                Literal::Number(_) | Literal::String(_) => Ok(()),
                Literal::Boolean(_) => Ok(()),
                Literal::Nil => Ok(()),
            },
            Expr::Unary { right, .. } => {
                Self::analyze_expression(right, environment)?;
                Ok(())
            }
            Expr::Variable { name, .. } => {
                if let Some(entry) = environment.get(name.lexeme.clone()) {
                    if !entry.is_initialized {
                        Err(Error::UninitializedVariable(name.lexeme.clone(), name.line, name.column))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(Error::VariableNotFound(name.lexeme.clone(), name.line, name.column))
                }
            }
            Expr::Assign { name, value } => {
                Self::analyze_expression(value, environment)?;
                if let Some(entry) = environment.get(name.lexeme.clone()) {
                    if !entry.is_initialized {
                        Err(Error::UninitializedVariable(name.lexeme.clone(), name.line, name.column))
                    } else {
                        Ok(())
                    }.unwrap();

                    Ok(())
                } else {
                    Err(Error::VariableNotFound(name.lexeme.clone(), name.line, name.column))
                }
            }
            Expr::Call { callee, arguments, .. } => {
                Self::analyze_expression(callee, environment)?;
                for arg in arguments {
                    Self::analyze_expression(arg, environment)?;
                }
                Ok(())
            }
        }
    }
}

/// A structure that represents the environment of the semantic analyzer.
struct Environment {
    scopes: Vec<Vec<VariableEntry>>,
}

impl Environment {
    /// Creates a new environment.
    fn new() -> Self {
        Self { scopes: vec![vec![]] }
    }

    /// Defines a new variable in the current scope.
    fn define(&mut self, name: String, is_initialized: bool) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.iter().any(|entry| entry.name == name) {
                return Err(Error::VariableRedeclaration(name));
            }

            scope.push(VariableEntry { name, is_initialized });
            Ok(())
        } else {
            Err(Error::NoActiveScope)
        }
    }

    /// Begins a new scope.
    fn begin_scope(&mut self) {
        self.scopes.push(vec![]);
    }

    /// Ends the current scope.
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// Retrieves the entry for the given variable name.
    fn get(&self, name: String) -> Option<&VariableEntry> {
        for scope in self.scopes.iter().rev() {
            if let Some(entry) = scope.iter().rev().find(|entry| entry.name == name) {
                return Some(entry);
            }
        }
        None
    }
}

/// A structure that represents a variable entry in the environment.
struct VariableEntry {
    name: String,
    is_initialized: bool,
}

/// A structure that represents semantic analysis errors.
#[derive(Debug)]
pub enum Error {
    VariableRedeclaration(String),
    VariableNotFound(String, u32, u32),
    UninitializedVariable(String, u32, u32),
    NoActiveScope,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::VariableRedeclaration(name) => write!(f, "Variable '{}' is already declared in this scope.", name),
            Error::VariableNotFound(name, line, column) => write!(f, "Variable '{}' is not defined. ({}:{})", name, line, column),
            Error::UninitializedVariable(name, line, column) => write!(f, "Variable '{}' is used before being initialized. ({}:{})", name, line, column),
            Error::NoActiveScope => write!(f, "No active scope."),
        }
    }
}
