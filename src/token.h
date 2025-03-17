#pragma once
#include <string>
#include <vector>

enum class TokenType {
    // Keywords
    FUNCTION,   // "function"
    RETURN,     // "return"
    IF,         // "if"
    ELSE,       // "else"
    WHILE,      // "while"
    FOR,        // "for"
    VARIABLE,   // "var"
    CONSTANT,   // "const"
    PRINT,      // "print"
    
    // Operators
    PLUS,       // "plus"
    MINUS,      // "minus"
    MULTIPLY,   // "times"
    DIVIDE,     // "divided by"
    ASSIGN,     // "is"
    EQUALS,     // "equals"
    NOT_EQUALS, // "not equals"
    GREATER,    // "greater than"
    LESS,       // "less than"
    
    // Delimiters
    LEFT_BRACE,   // "{"
    RIGHT_BRACE,  // "}"
    LEFT_PAREN,   // "("
    RIGHT_PAREN,  // ")"
    COMMA,        // ","
    
    // Others
    IDENTIFIER,   // Variable/function names
    NUMBER,       // Numeric literals
    STRING,       // String literals
    COMMENT,      // Comments
    EOL,          // End of line
    END_OF_FILE,  // End of file
    UNKNOWN       // Unknown token
};

struct Token {
    TokenType type;
    std::string value;
    int line;
    int column;
    
    Token(TokenType t, std::string v, int l, int c)
        : type(t), value(std::move(v)), line(l), column(c) {}
};

using TokenList = std::vector<Token>;
