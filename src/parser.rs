use crate::ast::*;
use crate::token::{Token, TokenType};
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_token(&self, offset: usize) -> Option<&Token> {
        let pos = self.current + offset;
        if pos < self.tokens.len() {
            Some(&self.tokens[pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
    }

    fn expect(&mut self, expected: &TokenType) -> Result<()> {
        if std::mem::discriminant(&self.current_token().token_type)
            == std::mem::discriminant(expected)
        {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!(
                "Expected {:?}, found {:?} at line {}",
                expected,
                self.current_token().token_type,
                self.current_token().line
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut functions = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::Eof) {
            functions.push(self.parse_function()?);
        }

        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function> {
        self.expect(&TokenType::Function)?;

        let name = if let TokenType::Identifier(n) = &self.current_token().token_type {
            let name = n.clone();
            self.advance();
            name
        } else {
            return Err(anyhow!("Expected function name"));
        };

        self.expect(&TokenType::LeftParen)?;

        let mut params = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::RightParen) {
            if let TokenType::Identifier(param) = &self.current_token().token_type {
                params.push(param.clone());
                self.advance();

                if matches!(self.current_token().token_type, TokenType::Comma) {
                    self.advance();
                }
            } else {
                return Err(anyhow!("Expected parameter name"));
            }
        }

        self.expect(&TokenType::RightParen)?;
        self.expect(&TokenType::LeftBrace)?;

        let body = self.parse_block()?;

        self.expect(&TokenType::RightBrace)?;

        Ok(Function { name, params, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !matches!(
            self.current_token().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        match &self.current_token().token_type {
            TokenType::Variable => self.parse_var_decl(false),
            TokenType::Constant => self.parse_var_decl(true),
            TokenType::Print => self.parse_print(),
            TokenType::Ask => self.parse_ask(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Return => self.parse_return(),
            TokenType::Break => {
                self.advance();
                Ok(Stmt::Break)
            }
            TokenType::Continue => {
                self.advance();
                Ok(Stmt::Continue)
            }
            TokenType::Identifier(_) => self.parse_assignment_or_expr(),
            _ => Err(anyhow!(
                "Unexpected token: {:?} at line {}",
                self.current_token().token_type,
                self.current_token().line
            )),
        }
    }

    fn parse_var_decl(&mut self, is_const: bool) -> Result<Stmt> {
        self.advance(); // Skip 'var' or 'const'

        let name = if let TokenType::Identifier(n) = &self.current_token().token_type {
            let name = n.clone();
            self.advance();
            name
        } else {
            return Err(anyhow!("Expected variable name"));
        };

        self.expect(&TokenType::Assign)?;

        let value = self.parse_expression()?;

        Ok(Stmt::VarDecl {
            name,
            value,
            is_const,
        })
    }

    fn parse_assignment_or_expr(&mut self) -> Result<Stmt> {
        let name = if let TokenType::Identifier(n) = &self.current_token().token_type {
            let name = n.clone();
            self.advance();
            name
        } else {
            return Err(anyhow!("Expected identifier"));
        };

        // Check for array index assignment: identifier at index is value
        if matches!(self.current_token().token_type, TokenType::At) {
            self.advance();
            let index = self.parse_term()?;

            if matches!(self.current_token().token_type, TokenType::Assign) {
                self.advance();
                let value = self.parse_expression()?;
                return Ok(Stmt::IndexAssignment {
                    array: Box::new(Expr::Identifier(name)),
                    index: Box::new(index),
                    value,
                });
            } else {
                return Err(anyhow!("Expected 'is' after array index"));
            }
        }

        if matches!(self.current_token().token_type, TokenType::Assign) {
            self.advance();
            let value = self.parse_expression()?;
            Ok(Stmt::Assignment { name, value })
        } else {
            // It's a function call
            self.current -= 1; // Go back
            let expr = self.parse_expression()?;
            Ok(Stmt::Expression(expr))
        }
    }

    fn parse_print(&mut self) -> Result<Stmt> {
        self.advance(); // Skip 'print'
        let expr = self.parse_expression()?;
        Ok(Stmt::Print(expr))
    }

    fn parse_ask(&mut self) -> Result<Stmt> {
        self.advance(); // Skip 'ask'

        let name = if let TokenType::Identifier(n) = &self.current_token().token_type {
            let name = n.clone();
            self.advance();
            name
        } else {
            return Err(anyhow!("Expected variable name after 'ask'"));
        };

        // Optional prompt
        let prompt = if !matches!(
            self.current_token().token_type,
            TokenType::RightBrace
                | TokenType::Eof
                | TokenType::Variable
                | TokenType::Constant
                | TokenType::Print
                | TokenType::Ask
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Return
                | TokenType::Identifier(_)
        ) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Stmt::Ask { name, prompt })
    }

    fn parse_if(&mut self) -> Result<Stmt> {
        self.advance(); // Skip 'if'

        let condition = self.parse_expression()?;
        self.expect(&TokenType::LeftBrace)?;
        let then_block = self.parse_block()?;
        self.expect(&TokenType::RightBrace)?;

        let else_block = if matches!(self.current_token().token_type, TokenType::Else) {
            self.advance();
            self.expect(&TokenType::LeftBrace)?;
            let block = self.parse_block()?;
            self.expect(&TokenType::RightBrace)?;
            Some(block)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_block,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt> {
        self.advance(); // Skip 'while'

        let condition = self.parse_expression()?;
        self.expect(&TokenType::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(&TokenType::RightBrace)?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt> {
        self.advance(); // Skip 'for'

        self.expect(&TokenType::LeftParen)?;

        let init = Box::new(self.parse_statement()?);

        let condition = self.parse_expression()?;

        let increment = Box::new(self.parse_statement()?);

        self.expect(&TokenType::RightParen)?;
        self.expect(&TokenType::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(&TokenType::RightBrace)?;

        Ok(Stmt::For {
            init,
            condition,
            increment,
            body,
        })
    }

    fn parse_return(&mut self) -> Result<Stmt> {
        self.advance(); // Skip 'return'

        if matches!(
            self.current_token().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expression()?;
            Ok(Stmt::Return(Some(expr)))
        }
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr> {
        let mut left = self.parse_and()?;

        while matches!(self.current_token().token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr> {
        let mut left = self.parse_equality()?;

        while matches!(self.current_token().token_type, TokenType::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr> {
        let mut left = self.parse_comparison()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::Equals => BinaryOp::Equals,
                TokenType::Not => {
                    // Check if this is "not equals"
                    if matches!(self.peek_token(1).map(|t| &t.token_type), Some(TokenType::Equals)) {
                        self.advance(); // Skip "not"
                        self.advance(); // Skip "equals"
                        BinaryOp::NotEquals
                    } else {
                        break;
                    }
                }
                _ => break,
            };

            if op == BinaryOp::Equals {
                self.advance();
            }

            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_term()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::Greater => {
                    self.advance();
                    // Check for "greater than"
                    if let Some(TokenType::Identifier(ref s)) =
                        self.peek_token(0).map(|t| &t.token_type)
                    {
                        if s == "than" {
                            self.advance();
                            // Check for "or equals" after "greater than"
                            if matches!(self.current_token().token_type, TokenType::Or) {
                                if matches!(self.peek_token(1).map(|t| &t.token_type), Some(TokenType::Equals)) {
                                    self.advance(); // Skip "or"
                                    self.advance(); // Skip "equals"
                                    BinaryOp::GreaterOrEquals
                                } else {
                                    BinaryOp::Greater
                                }
                            } else {
                                BinaryOp::Greater
                            }
                        } else {
                            BinaryOp::Greater
                        }
                    } else {
                        BinaryOp::Greater
                    }
                }
                TokenType::Less => {
                    self.advance();
                    // Check for "less than"
                    if let Some(TokenType::Identifier(ref s)) =
                        self.peek_token(0).map(|t| &t.token_type)
                    {
                        if s == "than" {
                            self.advance();
                            // Check for "or equals" after "less than"
                            if matches!(self.current_token().token_type, TokenType::Or) {
                                if matches!(self.peek_token(1).map(|t| &t.token_type), Some(TokenType::Equals)) {
                                    self.advance(); // Skip "or"
                                    self.advance(); // Skip "equals"
                                    BinaryOp::LessOrEquals
                                } else {
                                    BinaryOp::Less
                                }
                            } else {
                                BinaryOp::Less
                            }
                        } else {
                            BinaryOp::Less
                        }
                    } else {
                        BinaryOp::Less
                    }
                }
                _ => break,
            };

            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut left = self.parse_factor()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.advance();
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::Multiply => BinaryOp::Multiply,
                TokenType::Divide => BinaryOp::Divide,
                TokenType::Modulo => BinaryOp::Modulo,
                _ => break,
            };

            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        match &self.current_token().token_type {
            TokenType::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            TokenType::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(operand),
                })
            }
            TokenType::Length => {
                self.advance();
                // Expect "of"
                self.expect(&TokenType::Of)?;
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Length,
                    operand: Box::new(operand),
                })
            }
            TokenType::Uppercase => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Uppercase,
                    operand: Box::new(operand),
                })
            }
            TokenType::Substring => {
                self.advance();
                let string = self.parse_primary()?;
                self.expect(&TokenType::From)?;
                let from = self.parse_primary()?;
                self.expect(&TokenType::To)?;
                let to = self.parse_primary()?;
                Ok(Expr::Substring {
                    string: Box::new(string),
                    from: Box::new(from),
                    to: Box::new(to),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        let mut expr = match &self.current_token().token_type.clone() {
            TokenType::Number(n) => {
                let num = *n;
                self.advance();
                Expr::Number(num)
            }
            TokenType::StringLit(s) => {
                let string = s.clone();
                self.advance();
                Expr::String(string)
            }
            TokenType::Boolean(b) => {
                let val = *b;
                self.advance();
                Expr::Boolean(val)
            }
            TokenType::List => {
                self.advance(); // Skip 'list'
                let mut elements = Vec::new();

                // Parse list elements until we hit a statement boundary or right brace
                loop {
                    if matches!(
                        self.current_token().token_type,
                        TokenType::RightBrace
                            | TokenType::Eof
                            | TokenType::Variable
                            | TokenType::Constant
                            | TokenType::Print
                            | TokenType::Ask
                            | TokenType::If
                            | TokenType::While
                            | TokenType::For
                            | TokenType::Return
                    ) {
                        break;
                    }

                    // Try to parse an expression
                    let element = self.parse_term()?;
                    elements.push(element);

                    // Check if there's a comma
                    if matches!(self.current_token().token_type, TokenType::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }

                Expr::Array(elements)
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // Check for function call
                if matches!(self.current_token().token_type, TokenType::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();

                    while !matches!(self.current_token().token_type, TokenType::RightParen) {
                        args.push(self.parse_expression()?);

                        if matches!(self.current_token().token_type, TokenType::Comma) {
                            self.advance();
                        }
                    }

                    self.expect(&TokenType::RightParen)?;
                    Expr::Call { name, args }
                } else {
                    Expr::Identifier(name)
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let e = self.parse_expression()?;
                self.expect(&TokenType::RightParen)?;
                e
            }
            _ => {
                return Err(anyhow!(
                    "Unexpected token in expression: {:?} at line {}",
                    self.current_token().token_type,
                    self.current_token().line
                ))
            }
        };

        // Check for array indexing with 'at'
        while matches!(self.current_token().token_type, TokenType::At) {
            self.advance();
            let index = self.parse_term()?;
            expr = Expr::Index {
                array: Box::new(expr),
                index: Box::new(index),
            };
        }

        Ok(expr)
    }
}
