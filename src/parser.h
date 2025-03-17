#pragma once
#include "token.h"
#include "ast.h"
#include <memory>

class Parser {
private:
    TokenList tokens;
    int position;
    
    Token currentToken();
    Token peek(int offset = 1);
    void advance();
    
    std::unique_ptr<ASTNode> parseProgram();
    std::unique_ptr<ASTNode> parseStatement();
    std::unique_ptr<ASTNode> parseBlock();
    std::unique_ptr<ASTNode> parseVariableDeclaration();
    std::unique_ptr<ASTNode> parseExpression();
    std::unique_ptr<ASTNode> parseTerm();
    std::unique_ptr<ASTNode> parseFactor();
    std::unique_ptr<ASTNode> parsePrimary();  // Add this missing declaration
    std::unique_ptr<ASTNode> parseFunction();
    std::unique_ptr<ASTNode> parseIfStatement();
    std::unique_ptr<ASTNode> parseWhileLoop();
    std::unique_ptr<ASTNode> parsePrintStatement();
    
public:
    Parser(const TokenList& tokenList);
    std::unique_ptr<ASTNode> parse();
};
