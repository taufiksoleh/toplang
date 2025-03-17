#include "lexer.h"
#include <cctype>

Lexer::Lexer(const std::string& src) : source(src), position(0), line(1), column(1) {
    if (!source.empty()) {
        currentChar = source[0];
    } else {
        currentChar = '\0';
    }
    
    // Initialize keyword mapping
    keywords["function"] = TokenType::FUNCTION;
    keywords["return"] = TokenType::RETURN;
    keywords["if"] = TokenType::IF;
    keywords["else"] = TokenType::ELSE;
    keywords["while"] = TokenType::WHILE;
    keywords["for"] = TokenType::FOR;
    keywords["var"] = TokenType::VARIABLE;
    keywords["const"] = TokenType::CONSTANT;
    keywords["print"] = TokenType::PRINT;
    keywords["plus"] = TokenType::PLUS;
    keywords["minus"] = TokenType::MINUS;
    keywords["times"] = TokenType::MULTIPLY;
    keywords["divided"] = TokenType::DIVIDE;
    keywords["is"] = TokenType::ASSIGN;
    keywords["equals"] = TokenType::EQUALS;
    keywords["not"] = TokenType::NOT_EQUALS;
    keywords["greater"] = TokenType::GREATER;
    keywords["less"] = TokenType::LESS;
}

void Lexer::advance() {
    position++;
    if (position < source.length()) {
        currentChar = source[position];
        column++;
        
        if (currentChar == '\n') {
            line++;
            column = 1;
        }
    } else {
        currentChar = '\0';
    }
}

void Lexer::skipWhitespace() {
    while (currentChar != '\0' && std::isspace(currentChar)) {
        advance();
    }
}

Token Lexer::identifier() {
    std::string result;
    int startColumn = column;
    
    while (currentChar != '\0' && (std::isalnum(currentChar) || currentChar == '_')) {
        result += currentChar;
        advance();
    }
    
    // Check if the identifier is a keyword
    if (keywords.find(result) != keywords.end()) {
        return Token(keywords[result], result, line, startColumn);
    }
    
    return Token(TokenType::IDENTIFIER, result, line, startColumn);
}

Token Lexer::number() {
    std::string result;
    int startColumn = column;
    
    while (currentChar != '\0' && std::isdigit(currentChar)) {
        result += currentChar;
        advance();
    }
    
    // Handle decimal numbers
    if (currentChar == '.') {
        result += currentChar;
        advance();
        
        while (currentChar != '\0' && std::isdigit(currentChar)) {
            result += currentChar;
            advance();
        }
    }
    
    return Token(TokenType::NUMBER, result, line, startColumn);
}

Token Lexer::string() {
    std::string result;
    int startColumn = column;
    advance(); // Skip the opening quote
    
    while (currentChar != '\0' && currentChar != '"') {
        result += currentChar;
        advance();
    }
    
    if (currentChar == '"') {
        advance(); // Skip the closing quote
    }
    
    return Token(TokenType::STRING, result, line, startColumn);
}

void Lexer::skipComment() {
    while (currentChar != '\0' && currentChar != '\n') {
        advance();
    }
}

TokenList Lexer::tokenize() {
    TokenList tokens;
    
    while (currentChar != '\0') {
        // Skip whitespace
        if (std::isspace(currentChar)) {
            skipWhitespace();
            continue;
        }
        
        // Handle identifiers and keywords
        if (std::isalpha(currentChar) || currentChar == '_') {
            tokens.push_back(identifier());
            continue;
        }
        
        // Handle numbers
        if (std::isdigit(currentChar)) {
            tokens.push_back(number());
            continue;
        }
        
        // Handle special characters
        switch (currentChar) {
            case '{':
                tokens.push_back(Token(TokenType::LEFT_BRACE, "{", line, column));
                advance();
                break;
            case '}':
                tokens.push_back(Token(TokenType::RIGHT_BRACE, "}", line, column));
                advance();
                break;
            case '(':
                tokens.push_back(Token(TokenType::LEFT_PAREN, "(", line, column));
                advance();
                break;
            case ')':
                tokens.push_back(Token(TokenType::RIGHT_PAREN, ")", line, column));
                advance();
                break;
            case ',':
                tokens.push_back(Token(TokenType::COMMA, ",", line, column));
                advance();
                break;
            case '"':
                tokens.push_back(string());
                break;
            case '#': // Comment
                skipComment();
                break;
            case '\n':
                tokens.push_back(Token(TokenType::EOL, "\n", line, column));
                advance();
                break;
            default:
                tokens.push_back(Token(TokenType::UNKNOWN, std::string(1, currentChar), line, column));
                advance();
                break;
        }
    }
    
    tokens.push_back(Token(TokenType::END_OF_FILE, "", line, column));
    return tokens;
}
