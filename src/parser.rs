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
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Return => self.parse_return(),
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
                _ => break,
            };

            self.advance();
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
                        }
                    }
                    BinaryOp::Greater
                }
                TokenType::Less => {
                    self.advance();
                    // Check for "less than"
                    if let Some(TokenType::Identifier(ref s)) =
                        self.peek_token(0).map(|t| &t.token_type)
                    {
                        if s == "than" {
                            self.advance();
                        }
                    }
                    BinaryOp::Less
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
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match &self.current_token().token_type.clone() {
            TokenType::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            TokenType::StringLit(s) => {
                let string = s.clone();
                self.advance();
                Ok(Expr::String(string))
            }
            TokenType::Boolean(b) => {
                let val = *b;
                self.advance();
                Ok(Expr::Boolean(val))
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
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&TokenType::RightParen)?;
                Ok(expr)
            }
            _ => Err(anyhow!(
                "Unexpected token in expression: {:?} at line {}",
                self.current_token().token_type,
                self.current_token().line
            )),
        }
    }
}
