use crate::ast::*;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
    functions: HashMap<String, Function>,
    return_value: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            locals: Vec::new(),
            functions: HashMap::new(),
            return_value: None,
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<i32> {
        // Store all functions
        for func in program.functions {
            self.functions.insert(func.name.clone(), func);
        }

        // Look for main function
        if let Some(main_func) = self.functions.get("main").cloned() {
            let result = self.call_function(&main_func, Vec::new())?;

            // Return the exit code
            match result {
                Value::Number(n) => Ok(n as i32),
                _ => Ok(0),
            }
        } else {
            Err(anyhow!("No main function found"))
        }
    }

    fn call_function(&mut self, func: &Function, args: Vec<Value>) -> Result<Value> {
        if args.len() != func.params.len() {
            return Err(anyhow!(
                "Function '{}' expects {} arguments, got {}",
                func.name,
                func.params.len(),
                args.len()
            ));
        }

        // Create new scope
        let mut local_scope = HashMap::new();
        for (param, arg) in func.params.iter().zip(args.iter()) {
            local_scope.insert(param.clone(), arg.clone());
        }
        self.locals.push(local_scope);

        // Execute function body
        for stmt in &func.body {
            self.execute_stmt(stmt)?;

            if self.return_value.is_some() {
                break;
            }
        }

        // Pop scope
        self.locals.pop();

        // Get return value or default to Null
        let result = self.return_value.take().unwrap_or(Value::Null);
        Ok(result)
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::VarDecl {
                name,
                value,
                is_const: _,
            } => {
                let val = self.eval_expr(value)?;
                self.set_variable(name.clone(), val);
                Ok(())
            }
            Stmt::Assignment { name, value } => {
                let val = self.eval_expr(value)?;
                self.set_variable(name.clone(), val);
                Ok(())
            }
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", val);
                Ok(())
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let cond_val = self.eval_expr(condition)?;
                if cond_val.is_truthy() {
                    for stmt in then_block {
                        self.execute_stmt(stmt)?;
                        if self.return_value.is_some() {
                            break;
                        }
                    }
                } else if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.execute_stmt(stmt)?;
                        if self.return_value.is_some() {
                            break;
                        }
                    }
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond_val = self.eval_expr(condition)?;
                    if !cond_val.is_truthy() {
                        break;
                    }

                    for stmt in body {
                        self.execute_stmt(stmt)?;
                        if self.return_value.is_some() {
                            return Ok(());
                        }
                    }
                }
                Ok(())
            }
            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                self.execute_stmt(init)?;

                loop {
                    let cond_val = self.eval_expr(condition)?;
                    if !cond_val.is_truthy() {
                        break;
                    }

                    for stmt in body {
                        self.execute_stmt(stmt)?;
                        if self.return_value.is_some() {
                            return Ok(());
                        }
                    }

                    self.execute_stmt(increment)?;
                }
                Ok(())
            }
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::Null
                };
                self.return_value = Some(val);
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.eval_expr(expr)?;
                Ok(())
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Identifier(name) => self.get_variable(name),
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary_op(&left_val, op, &right_val)
            }
            Expr::Unary { op, operand } => {
                let val = self.eval_expr(operand)?;
                self.eval_unary_op(op, &val)
            }
            Expr::Call { name, args } => {
                let func = self
                    .functions
                    .get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Undefined function: {}", name))?;

                let arg_values: Result<Vec<Value>> =
                    args.iter().map(|arg| self.eval_expr(arg)).collect();

                self.call_function(&func, arg_values?)
            }
        }
    }

    fn eval_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match op {
                BinaryOp::Add => Ok(Value::Number(l + r)),
                BinaryOp::Subtract => Ok(Value::Number(l - r)),
                BinaryOp::Multiply => Ok(Value::Number(l * r)),
                BinaryOp::Divide => {
                    if *r == 0.0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                BinaryOp::Equals => Ok(Value::Boolean((l - r).abs() < f64::EPSILON)),
                BinaryOp::NotEquals => Ok(Value::Boolean((l - r).abs() >= f64::EPSILON)),
                BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                BinaryOp::Less => Ok(Value::Boolean(l < r)),
                BinaryOp::GreaterEquals => Ok(Value::Boolean(l >= r)),
                BinaryOp::LessEquals => Ok(Value::Boolean(l <= r)),
                _ => Err(anyhow!("Invalid operation for numbers")),
            },
            (Value::String(l), Value::String(r)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                BinaryOp::Equals => Ok(Value::Boolean(l == r)),
                BinaryOp::NotEquals => Ok(Value::Boolean(l != r)),
                _ => Err(anyhow!("Invalid operation for strings")),
            },
            (Value::Boolean(l), Value::Boolean(r)) => match op {
                BinaryOp::And => Ok(Value::Boolean(*l && *r)),
                BinaryOp::Or => Ok(Value::Boolean(*l || *r)),
                BinaryOp::Equals => Ok(Value::Boolean(l == r)),
                BinaryOp::NotEquals => Ok(Value::Boolean(l != r)),
                _ => Err(anyhow!("Invalid operation for booleans")),
            },
            // String concatenation with numbers
            (Value::String(s), Value::Number(n)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", s, n))),
                _ => Err(anyhow!("Invalid operation between string and number")),
            },
            (Value::Number(n), Value::String(s)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", n, s))),
                _ => Err(anyhow!("Invalid operation between number and string")),
            },
            _ => Err(anyhow!("Type mismatch in binary operation")),
        }
    }

    fn eval_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value> {
        match op {
            UnaryOp::Not => Ok(Value::Boolean(!operand.is_truthy())),
            UnaryOp::Negate => match operand {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(anyhow!("Cannot negate non-number")),
            },
        }
    }

    fn set_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }

    fn get_variable(&self, name: &str) -> Result<Value> {
        // Check local scopes (from innermost to outermost)
        for scope in self.locals.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }

        // Check global scope
        if let Some(val) = self.globals.get(name) {
            Ok(val.clone())
        } else {
            Err(anyhow!("Undefined variable: {}", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_truthiness() {
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
        assert!(Value::Number(1.0).is_truthy());
        assert!(!Value::Number(0.0).is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
        assert!(!Value::Null.is_truthy());
    }
}
