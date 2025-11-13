/// Constant folding optimizer
///
/// This module performs compile-time evaluation of constant expressions,
/// eliminating runtime overhead for operations with known values.

use crate::ast::*;

/// Optimize an expression by folding constants
pub fn fold_constants(expr: &Expr) -> Expr {
    match expr {
        Expr::Binary { left, op, right } => {
            let left = fold_constants(left);
            let right = fold_constants(right);

            // Try to fold if both sides are constants
            match (&left, op, &right) {
                (Expr::Number(a), BinaryOp::Add, Expr::Number(b)) => {
                    Expr::Number(a + b)
                }
                (Expr::Number(a), BinaryOp::Subtract, Expr::Number(b)) => {
                    Expr::Number(a - b)
                }
                (Expr::Number(a), BinaryOp::Multiply, Expr::Number(b)) => {
                    Expr::Number(a * b)
                }
                (Expr::Number(a), BinaryOp::Divide, Expr::Number(b)) => {
                    if *b != 0.0 {
                        Expr::Number(a / b)
                    } else {
                        // Can't fold division by zero
                        Expr::Binary {
                            left: Box::new(left),
                            op: op.clone(),
                            right: Box::new(right),
                        }
                    }
                }
                (Expr::Number(a), BinaryOp::Modulo, Expr::Number(b)) => {
                    if *b != 0.0 {
                        Expr::Number(a % b)
                    } else {
                        Expr::Binary {
                            left: Box::new(left),
                            op: op.clone(),
                            right: Box::new(right),
                        }
                    }
                }
                (Expr::Boolean(a), BinaryOp::And, Expr::Boolean(b)) => {
                    Expr::Boolean(*a && *b)
                }
                (Expr::Boolean(a), BinaryOp::Or, Expr::Boolean(b)) => {
                    Expr::Boolean(*a || *b)
                }
                (Expr::Number(a), BinaryOp::Equals, Expr::Number(b)) => {
                    Expr::Boolean((a - b).abs() < f64::EPSILON)
                }
                (Expr::Number(a), BinaryOp::NotEquals, Expr::Number(b)) => {
                    Expr::Boolean((a - b).abs() >= f64::EPSILON)
                }
                (Expr::Number(a), BinaryOp::Greater, Expr::Number(b)) => {
                    Expr::Boolean(a > b)
                }
                (Expr::Number(a), BinaryOp::GreaterOrEquals, Expr::Number(b)) => {
                    Expr::Boolean(a >= b)
                }
                (Expr::Number(a), BinaryOp::Less, Expr::Number(b)) => {
                    Expr::Boolean(a < b)
                }
                (Expr::Number(a), BinaryOp::LessOrEquals, Expr::Number(b)) => {
                    Expr::Boolean(a <= b)
                }
                (Expr::String(a), BinaryOp::Add, Expr::String(b)) => {
                    Expr::String(format!("{}{}", a, b))
                }
                // Special optimizations
                (Expr::Number(n), BinaryOp::Add, _) if *n == 0.0 => right,
                (_, BinaryOp::Add, Expr::Number(n)) if *n == 0.0 => left,
                (Expr::Number(n), BinaryOp::Multiply, _) if *n == 1.0 => right,
                (_, BinaryOp::Multiply, Expr::Number(n)) if *n == 1.0 => left,
                (Expr::Number(n), BinaryOp::Multiply, _) if *n == 0.0 => Expr::Number(0.0),
                (_, BinaryOp::Multiply, Expr::Number(n)) if *n == 0.0 => Expr::Number(0.0),
                _ => Expr::Binary {
                    left: Box::new(left),
                    op: op.clone(),
                    right: Box::new(right),
                },
            }
        }

        Expr::Unary { op, operand } => {
            let operand = fold_constants(operand);

            match (op, &operand) {
                (UnaryOp::Not, Expr::Boolean(b)) => Expr::Boolean(!b),
                (UnaryOp::Negate, Expr::Number(n)) => Expr::Number(-n),
                (UnaryOp::Length, Expr::String(s)) => Expr::Number(s.len() as f64),
                (UnaryOp::Length, Expr::Array(arr)) => Expr::Number(arr.len() as f64),
                (UnaryOp::Uppercase, Expr::String(s)) => Expr::String(s.to_uppercase()),
                _ => Expr::Unary {
                    op: op.clone(),
                    operand: Box::new(operand),
                },
            }
        }

        Expr::Array(elements) => {
            let folded: Vec<Expr> = elements.iter().map(fold_constants).collect();
            Expr::Array(folded)
        }

        Expr::Index { array, index } => {
            let array = fold_constants(array);
            let index = fold_constants(index);

            // Try to fold constant array indexing
            match (&array, &index) {
                (Expr::Array(arr), Expr::Number(idx)) => {
                    let idx = *idx as usize;
                    if idx < arr.len() {
                        arr[idx].clone()
                    } else {
                        Expr::Index {
                            array: Box::new(array),
                            index: Box::new(index),
                        }
                    }
                }
                _ => Expr::Index {
                    array: Box::new(array),
                    index: Box::new(index),
                },
            }
        }

        Expr::Call { name, args } => {
            let folded_args: Vec<Expr> = args.iter().map(fold_constants).collect();
            Expr::Call {
                name: name.clone(),
                args: folded_args,
            }
        }

        Expr::Substring { string, from, to } => {
            let string = fold_constants(string);
            let from = fold_constants(from);
            let to = fold_constants(to);

            // Try to fold constant substring
            match (&string, &from, &to) {
                (Expr::String(s), Expr::Number(f), Expr::Number(t)) => {
                    let from_idx = *f as usize;
                    let to_idx = *t as usize;
                    let chars: Vec<char> = s.chars().collect();

                    if from_idx <= to_idx && to_idx <= chars.len() {
                        let result: String = chars[from_idx..to_idx].iter().collect();
                        Expr::String(result)
                    } else {
                        Expr::Substring {
                            string: Box::new(string),
                            from: Box::new(from),
                            to: Box::new(to),
                        }
                    }
                }
                _ => Expr::Substring {
                    string: Box::new(string),
                    from: Box::new(from),
                    to: Box::new(to),
                },
            }
        }

        // These can't be folded
        Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Identifier(_) => expr.clone(),
    }
}

/// Optimize a statement by folding constants in expressions
pub fn optimize_stmt(stmt: &Stmt) -> Stmt {
    match stmt {
        Stmt::VarDecl { name, value, is_const } => Stmt::VarDecl {
            name: name.clone(),
            value: fold_constants(value),
            is_const: *is_const,
        },

        Stmt::Assignment { name, value } => Stmt::Assignment {
            name: name.clone(),
            value: fold_constants(value),
        },

        Stmt::IndexAssignment { array, index, value } => Stmt::IndexAssignment {
            array: Box::new(fold_constants(array)),
            index: Box::new(fold_constants(index)),
            value: fold_constants(value),
        },

        Stmt::Print(expr) => Stmt::Print(fold_constants(expr)),

        Stmt::Ask { name, prompt } => Stmt::Ask {
            name: name.clone(),
            prompt: prompt.as_ref().map(fold_constants),
        },

        Stmt::If {
            condition,
            then_block,
            else_block,
        } => {
            let folded_condition = fold_constants(condition);

            // If condition is constant, we can eliminate branches!
            match &folded_condition {
                Expr::Boolean(true) => {
                    // Always true: only keep then_block
                    if then_block.len() == 1 {
                        optimize_stmt(&then_block[0])
                    } else {
                        Stmt::If {
                            condition: folded_condition,
                            then_block: then_block.iter().map(optimize_stmt).collect(),
                            else_block: None,
                        }
                    }
                }
                Expr::Boolean(false) => {
                    // Always false: only keep else_block or eliminate
                    if let Some(else_stmts) = else_block {
                        if else_stmts.len() == 1 {
                            optimize_stmt(&else_stmts[0])
                        } else {
                            Stmt::If {
                                condition: folded_condition,
                                then_block: Vec::new(),
                                else_block: Some(else_stmts.iter().map(optimize_stmt).collect()),
                            }
                        }
                    } else {
                        // No-op, but we need to return something
                        Stmt::Expression(Expr::Number(0.0))
                    }
                }
                _ => Stmt::If {
                    condition: folded_condition,
                    then_block: then_block.iter().map(optimize_stmt).collect(),
                    else_block: else_block
                        .as_ref()
                        .map(|stmts| stmts.iter().map(optimize_stmt).collect()),
                },
            }
        }

        Stmt::While { condition, body } => Stmt::While {
            condition: fold_constants(condition),
            body: body.iter().map(optimize_stmt).collect(),
        },

        Stmt::For {
            init,
            condition,
            increment,
            body,
        } => Stmt::For {
            init: Box::new(optimize_stmt(init)),
            condition: fold_constants(condition),
            increment: Box::new(optimize_stmt(increment)),
            body: body.iter().map(optimize_stmt).collect(),
        },

        Stmt::Return(expr) => Stmt::Return(expr.as_ref().map(fold_constants)),

        Stmt::Expression(expr) => Stmt::Expression(fold_constants(expr)),

        // These don't need optimization
        Stmt::Break | Stmt::Continue => stmt.clone(),
    }
}

/// Optimize a function
pub fn optimize_function(func: &Function) -> Function {
    Function {
        name: func.name.clone(),
        params: func.params.clone(),
        body: func.body.iter().map(optimize_stmt).collect(),
    }
}

/// Optimize a program
pub fn optimize_program(program: &Program) -> Program {
    Program {
        functions: program.functions.iter().map(optimize_function).collect(),
    }
}
