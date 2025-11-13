use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    Greater,
    GreaterOrEquals,
    Less,
    LessOrEquals,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Negate,
    Length,
    Uppercase,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Array(Vec<Expr>),
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Substring {
        string: Box<Expr>,
        from: Box<Expr>,
        to: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarDecl {
        name: String,
        value: Expr,
        is_const: bool,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    IndexAssignment {
        array: Box<Expr>,
        index: Box<Expr>,
        value: Expr,
    },
    Print(Expr),
    Ask {
        name: String,
        prompt: Option<Expr>,
    },
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Box<Stmt>,
        condition: Expr,
        increment: Box<Stmt>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::Equals => write!(f, "=="),
            BinaryOp::NotEquals => write!(f, "!="),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::GreaterOrEquals => write!(f, ">="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::LessOrEquals => write!(f, "<="),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::Negate => write!(f, "-"),
            UnaryOp::Length => write!(f, "length"),
            UnaryOp::Uppercase => write!(f, "uppercase"),
        }
    }
}
