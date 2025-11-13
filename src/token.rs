use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Function,
    Return,
    If,
    Else,
    While,
    For,
    Variable,
    Constant,
    Print,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equals,
    Greater,
    Less,
    And,
    Or,
    Not,

    // Delimiters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Comma,

    // Literals
    Identifier(String),
    Number(f64),
    StringLit(String),
    Boolean(bool),

    // Special
    Eof,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Function => write!(f, "function"),
            TokenType::Return => write!(f, "return"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::While => write!(f, "while"),
            TokenType::For => write!(f, "for"),
            TokenType::Variable => write!(f, "var"),
            TokenType::Constant => write!(f, "const"),
            TokenType::Print => write!(f, "print"),
            TokenType::Plus => write!(f, "plus"),
            TokenType::Minus => write!(f, "minus"),
            TokenType::Multiply => write!(f, "times"),
            TokenType::Divide => write!(f, "divided"),
            TokenType::Assign => write!(f, "is"),
            TokenType::Equals => write!(f, "equals"),
            TokenType::Greater => write!(f, "greater"),
            TokenType::Less => write!(f, "less"),
            TokenType::And => write!(f, "and"),
            TokenType::Or => write!(f, "or"),
            TokenType::Not => write!(f, "not"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::Comma => write!(f, ","),
            TokenType::Identifier(s) => write!(f, "identifier '{}'", s),
            TokenType::Number(n) => write!(f, "number {}", n),
            TokenType::StringLit(s) => write!(f, "string \"{}\"", s),
            TokenType::Boolean(b) => write!(f, "boolean {}", b),
            TokenType::Eof => write!(f, "end of file"),
            TokenType::Unknown => write!(f, "unknown"),
        }
    }
}
