#pragma once
#include "token.h"
#include <string>
#include <unordered_map>

class Lexer {
private:
    std::string source;
    int position;
    int line;
    int column;
    char currentChar;
    
    std::unordered_map<std::string, TokenType> keywords;
    
    void advance();
    void skipWhitespace();
    Token identifier();
    Token number();
    Token string();
    void skipComment();
    
public:
    Lexer(const std::string& src);
    TokenList tokenize();
};
