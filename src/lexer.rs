use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        // Keywords
        keywords.insert("function".to_string(), TokenType::Function);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("var".to_string(), TokenType::Variable);
        keywords.insert("const".to_string(), TokenType::Constant);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("ask".to_string(), TokenType::Ask);
        keywords.insert("list".to_string(), TokenType::List);
        keywords.insert("at".to_string(), TokenType::At);
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("length".to_string(), TokenType::Length);
        keywords.insert("uppercase".to_string(), TokenType::Uppercase);
        keywords.insert("substring".to_string(), TokenType::Substring);
        keywords.insert("from".to_string(), TokenType::From);
        keywords.insert("to".to_string(), TokenType::To);
        keywords.insert("of".to_string(), TokenType::Of);
        keywords.insert("true".to_string(), TokenType::Boolean(true));
        keywords.insert("false".to_string(), TokenType::Boolean(false));

        // Operators
        keywords.insert("plus".to_string(), TokenType::Plus);
        keywords.insert("minus".to_string(), TokenType::Minus);
        keywords.insert("times".to_string(), TokenType::Multiply);
        keywords.insert("divided".to_string(), TokenType::Divide);
        keywords.insert("is".to_string(), TokenType::Assign);
        keywords.insert("equals".to_string(), TokenType::Equals);
        keywords.insert("greater".to_string(), TokenType::Greater);
        keywords.insert("less".to_string(), TokenType::Less);
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("not".to_string(), TokenType::Not);

        Lexer {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.source.len() {
            Some(self.source[self.position])
        } else {
            None
        }
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.source.len() {
            Some(self.source[pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            self.position += 1;
            self.column += 1;

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        result
    }

    fn read_number(&mut self) -> f64 {
        let mut result = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_numeric() {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Handle decimal point
        if let Some('.') = self.current_char() {
            if let Some(next) = self.peek_char(1) {
                if next.is_numeric() {
                    result.push('.');
                    self.advance();

                    while let Some(ch) = self.current_char() {
                        if ch.is_numeric() {
                            result.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        result.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.advance(); // Skip opening quote

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        result
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char() {
            let start_column = self.column;
            let start_line = self.line;

            match ch {
                ' ' | '\t' | '\r' => {
                    self.skip_whitespace();
                    continue;
                }
                '\n' => {
                    self.advance();
                    continue; // Skip newlines for cleaner parsing
                }
                '#' => {
                    self.skip_comment();
                    continue;
                }
                '{' => {
                    tokens.push(Token::new(TokenType::LeftBrace, start_line, start_column));
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::new(TokenType::RightBrace, start_line, start_column));
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::new(TokenType::LeftParen, start_line, start_column));
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::new(TokenType::RightParen, start_line, start_column));
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::new(TokenType::Comma, start_line, start_column));
                    self.advance();
                }
                '"' => {
                    let string_val = self.read_string();
                    tokens.push(Token::new(
                        TokenType::StringLit(string_val),
                        start_line,
                        start_column,
                    ));
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();

                    // Check for multi-word operators
                    if identifier == "divided" {
                        self.skip_whitespace();
                        if let Some(next_ch) = self.current_char() {
                            if next_ch.is_alphabetic() {
                                let next_word = self.read_identifier();
                                if next_word == "by" {
                                    tokens.push(Token::new(
                                        TokenType::Divide,
                                        start_line,
                                        start_column,
                                    ));
                                    continue;
                                }
                            }
                        }
                    } else if identifier == "modulo"
                        || identifier == "mod"
                        || identifier == "remainder"
                    {
                        // Only treat as modulo operator if followed by "by"
                        let saved_pos = self.position;
                        let saved_line = self.line;
                        let saved_column = self.column;

                        self.skip_whitespace();
                        let mut is_modulo_operator = false;

                        if let Some(next_ch) = self.current_char() {
                            if next_ch.is_alphabetic() {
                                let next_word = self.read_identifier();
                                if next_word == "by" {
                                    is_modulo_operator = true;
                                } else {
                                    // Not "by", restore position
                                    self.position = saved_pos;
                                    self.line = saved_line;
                                    self.column = saved_column;
                                }
                            }
                        }

                        if is_modulo_operator {
                            tokens.push(Token::new(TokenType::Modulo, start_line, start_column));
                            continue;
                        }
                        // Otherwise fall through to treat as identifier
                    }

                    let token_type = if let Some(keyword_type) = self.keywords.get(&identifier) {
                        keyword_type.clone()
                    } else {
                        TokenType::Identifier(identifier)
                    };

                    tokens.push(Token::new(token_type, start_line, start_column));
                }
                _ if ch.is_numeric() => {
                    let number = self.read_number();
                    tokens.push(Token::new(
                        TokenType::Number(number),
                        start_line,
                        start_column,
                    ));
                }
                _ => {
                    tokens.push(Token::new(TokenType::Unknown, start_line, start_column));
                    self.advance();
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, self.line, self.column));
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let source = r#"var x is 10"#.to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5); // var, x, is, 10, EOF
    }

    #[test]
    fn test_string_tokenization() {
        let source = r#"print "Hello, World!""#.to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        assert!(matches!(tokens[0].token_type, TokenType::Print));
        assert!(matches!(tokens[1].token_type, TokenType::StringLit(_)));
    }
}
