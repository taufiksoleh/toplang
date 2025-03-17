#include "parser.h"
#include <stdexcept>
#include <iostream>

Parser::Parser(const TokenList& tokenList) : tokens(tokenList), position(0) {}

Token Parser::currentToken() {
    if (position >= tokens.size()) {
        return Token(TokenType::END_OF_FILE, "", 0, 0);
    }
    return tokens[position];
}

Token Parser::peek(int offset) {
    if (position + offset >= tokens.size()) {
        return Token(TokenType::END_OF_FILE, "", 0, 0);
    }
    return tokens[position + offset];
}

void Parser::advance() {
    position++;
}

std::unique_ptr<ASTNode> Parser::parse() {
    return parseProgram();
}

std::unique_ptr<ASTNode> Parser::parseProgram() {
    auto program = std::make_unique<ProgramNode>();
    
    while (currentToken().type != TokenType::END_OF_FILE) {
        try {
            auto statement = parseStatement();
            if (statement) {
                program->statements.push_back(std::move(statement));
            }
        } catch (const std::exception& e) {
            std::cerr << "Parser error at line " << currentToken().line 
                      << ", column " << currentToken().column << ": " << e.what() << std::endl;
            
            // Skip to the next statement to try to recover
            while (currentToken().type != TokenType::EOL && 
                   currentToken().type != TokenType::END_OF_FILE) {
                advance();
            }
            if (currentToken().type == TokenType::EOL) {
                advance();
            }
        }
    }
    
    return program;
}

std::unique_ptr<ASTNode> Parser::parseStatement() {
    // Skip empty lines
    while (currentToken().type == TokenType::EOL) {
        advance();
    }
    
    if (currentToken().type == TokenType::END_OF_FILE) {
        return nullptr;
    }
    
    // Handle different statement types
    switch (currentToken().type) {
        case TokenType::FUNCTION:
            return parseFunction();
        
        case TokenType::VARIABLE:
        case TokenType::CONSTANT:
            return parseVariableDeclaration();
        
        case TokenType::IF:
            return parseIfStatement();
        
        case TokenType::WHILE:
            return parseWhileLoop();
        
        case TokenType::PRINT:
            return parsePrintStatement();
        
        case TokenType::RETURN: {
            advance();
            auto returnValue = parseExpression();
            return std::make_unique<ReturnNode>(std::move(returnValue));
        }
        
        case TokenType::LEFT_BRACE:
            return parseBlock();
        
        default: {
            // Assume it's an expression statement (assignment or function call)
            auto expr = parseExpression();
            return expr;
        }
    }
}

std::unique_ptr<ASTNode> Parser::parseBlock() {
    auto block = std::make_unique<BlockNode>();
    
    // Expect a left brace
    if (currentToken().type != TokenType::LEFT_BRACE) {
        throw std::runtime_error("Expected '{' at the beginning of a block");
    }
    advance();
    
    // Parse statements until closing brace
    while (currentToken().type != TokenType::RIGHT_BRACE && 
           currentToken().type != TokenType::END_OF_FILE) {
        auto statement = parseStatement();
        if (statement) {
            block->statements.push_back(std::move(statement));
        }
    }
    
    // Expect a right brace
    if (currentToken().type != TokenType::RIGHT_BRACE) {
        throw std::runtime_error("Expected '}' at the end of a block");
    }
    advance();
    
    return block;
}

std::unique_ptr<ASTNode> Parser::parseVariableDeclaration() {
    bool isConstant = currentToken().type == TokenType::CONSTANT;
    advance();
    
    if (currentToken().type != TokenType::IDENTIFIER) {
        throw std::runtime_error("Expected identifier after 'var' or 'const'");
    }
    
    std::string varName = currentToken().value;
    advance();
    
    if (currentToken().type != TokenType::ASSIGN) {
        throw std::runtime_error("Expected 'is' after variable name");
    }
    advance();
    
    auto initialValue = parseExpression();
    
    return std::make_unique<VariableDeclarationNode>(varName, isConstant, std::move(initialValue));
}

std::unique_ptr<ASTNode> Parser::parseExpression() {
    auto left = parseTerm();
    
    while (currentToken().type == TokenType::ASSIGN || 
           currentToken().type == TokenType::EQUALS || 
           currentToken().type == TokenType::NOT_EQUALS || 
           currentToken().type == TokenType::GREATER || 
           currentToken().type == TokenType::LESS) {
        
        BinaryOpNode::Op op;
        switch (currentToken().type) {
            case TokenType::ASSIGN:
                op = BinaryOpNode::Op::Assign;
                break;
            case TokenType::EQUALS:
                op = BinaryOpNode::Op::Equals;
                break;
            case TokenType::NOT_EQUALS:
                op = BinaryOpNode::Op::NotEquals;
                break;
            case TokenType::GREATER:
                op = BinaryOpNode::Op::Greater;
                break;
            case TokenType::LESS:
                op = BinaryOpNode::Op::Less;
                break;
            default:
                throw std::runtime_error("Unexpected operator");
        }
        advance();
        
        // For "than" in comparisons like "greater than", "less than"
        if (op == BinaryOpNode::Op::Greater || op == BinaryOpNode::Op::Less) {
            if (currentToken().type == TokenType::IDENTIFIER && currentToken().value == "than") {
                advance();
            }
        }
        
        auto right = parseTerm();
        left = std::make_unique<BinaryOpNode>(op, std::move(left), std::move(right));
    }
    
    return left;
}

std::unique_ptr<ASTNode> Parser::parseTerm() {
    auto left = parseFactor();
    
    while (currentToken().type == TokenType::PLUS || 
           currentToken().type == TokenType::MINUS) {
        
        BinaryOpNode::Op op;
        switch (currentToken().type) {
            case TokenType::PLUS:
                op = BinaryOpNode::Op::Add;
                break;
            case TokenType::MINUS:
                op = BinaryOpNode::Op::Subtract;
                break;
            default:
                throw std::runtime_error("Unexpected operator");
        }
        advance();
        
        auto right = parseFactor();
        left = std::make_unique<BinaryOpNode>(op, std::move(left), std::move(right));
    }
    
    return left;
}

std::unique_ptr<ASTNode> Parser::parseFactor() {
    auto left = parsePrimary();
    
    while (currentToken().type == TokenType::MULTIPLY || 
           currentToken().type == TokenType::DIVIDE) {
        
        BinaryOpNode::Op op;
        switch (currentToken().type) {
            case TokenType::MULTIPLY:
                op = BinaryOpNode::Op::Multiply;
                break;
            case TokenType::DIVIDE:
                op = BinaryOpNode::Op::Divide;
                break;
            default:
                throw std::runtime_error("Unexpected operator");
        }
        advance();
        
        // Handle "by" after "divided by"
        if (op == BinaryOpNode::Op::Divide && currentToken().type == TokenType::IDENTIFIER && 
            currentToken().value == "by") {
            advance();
        }
        
        auto right = parsePrimary();
        left = std::make_unique<BinaryOpNode>(op, std::move(left), std::move(right));
    }
    
    return left;
}

std::unique_ptr<ASTNode> Parser::parsePrimary() {
    switch (currentToken().type) {
        case TokenType::NUMBER: {
            double value = std::stod(currentToken().value);
            advance();
            return std::make_unique<NumberNode>(value);
        }
        
        case TokenType::STRING: {
            std::string value = currentToken().value;
            advance();
            return std::make_unique<StringNode>(value);
        }
        
        case TokenType::IDENTIFIER: {
            std::string name = currentToken().value;
            advance();
            
            // Check if it's a function call
            if (currentToken().type == TokenType::LEFT_PAREN) {
                advance(); // Skip '('
                
                std::vector<std::unique_ptr<ASTNode>> args;
                
                // Parse arguments
                if (currentToken().type != TokenType::RIGHT_PAREN) {
                    args.push_back(parseExpression());
                    
                    while (currentToken().type == TokenType::COMMA) {
                        advance(); // Skip comma
                        args.push_back(parseExpression());
                    }
                }
                
                // Expect closing parenthesis
                if (currentToken().type != TokenType::RIGHT_PAREN) {
                    throw std::runtime_error("Expected ')' after function arguments");
                }
                advance(); // Skip ')'
                
                return std::make_unique<CallNode>(name, std::move(args));
            } else {
                // Otherwise it's a variable reference
                return std::make_unique<IdentifierNode>(name);
            }
        }
        
        case TokenType::LEFT_PAREN: {
            advance(); // Skip '('
            auto expr = parseExpression();
            
            if (currentToken().type != TokenType::RIGHT_PAREN) {
                throw std::runtime_error("Expected ')'");
            }
            advance(); // Skip ')'
            
            return expr;
        }
        
        default:
            throw std::runtime_error("Unexpected token: " + currentToken().value);
    }
}

std::unique_ptr<ASTNode> Parser::parseFunction() {
    advance(); // Skip 'function' keyword
    
    std::string name;
    if (currentToken().type == TokenType::IDENTIFIER) {
        name = currentToken().value;
        advance();
    } else {
        throw std::runtime_error("Expected function name");
    }
    
    // Parse parameter list
    if (currentToken().type != TokenType::LEFT_PAREN) {
        throw std::runtime_error("Expected '(' after function name");
    }
    advance(); // Skip '('
    
    std::vector<std::string> parameters;
    
    // Parse parameters
    if (currentToken().type != TokenType::RIGHT_PAREN) {
        if (currentToken().type != TokenType::IDENTIFIER) {
            throw std::runtime_error("Expected parameter name");
        }
        
        parameters.push_back(currentToken().value);
        advance();
        
        while (currentToken().type == TokenType::COMMA) {
            advance(); // Skip comma
            
            if (currentToken().type != TokenType::IDENTIFIER) {
                throw std::runtime_error("Expected parameter name after comma");
            }
            
            parameters.push_back(currentToken().value);
            advance();
        }
    }
    
    // Expect closing parenthesis
    if (currentToken().type != TokenType::RIGHT_PAREN) {
        throw std::runtime_error("Expected ')' after parameters");
    }
    advance(); // Skip ')'
    
    // Parse function body
    auto body = std::unique_ptr<BlockNode>(dynamic_cast<BlockNode*>(parseBlock().release()));
    if (!body) {
        throw std::runtime_error("Expected function body");
    }
    
    return std::make_unique<FunctionNode>(name, std::move(parameters), std::move(body));
}

std::unique_ptr<ASTNode> Parser::parseIfStatement() {
    advance(); // Skip 'if' keyword
    
    auto condition = parseExpression();
    
    auto thenBlock = std::unique_ptr<BlockNode>(dynamic_cast<BlockNode*>(parseBlock().release()));
    if (!thenBlock) {
        throw std::runtime_error("Expected block after if condition");
    }
    
    std::unique_ptr<BlockNode> elseBlock;
    if (currentToken().type == TokenType::ELSE) {
        advance(); // Skip 'else' keyword
        
        elseBlock = std::unique_ptr<BlockNode>(dynamic_cast<BlockNode*>(parseBlock().release()));
        if (!elseBlock) {
            throw std::runtime_error("Expected block after else");
        }
    }
    
    return std::make_unique<IfNode>(std::move(condition), std::move(thenBlock), std::move(elseBlock));
}

std::unique_ptr<ASTNode> Parser::parseWhileLoop() {
    advance(); // Skip 'while' keyword
    
    auto condition = parseExpression();
    
    auto body = std::unique_ptr<BlockNode>(dynamic_cast<BlockNode*>(parseBlock().release()));
    if (!body) {
        throw std::runtime_error("Expected block after while condition");
    }
    
    return std::make_unique<WhileNode>(std::move(condition), std::move(body));
}

std::unique_ptr<ASTNode> Parser::parsePrintStatement() {
    advance(); // Skip 'print' keyword
    
    auto expr = parseExpression();
    
    return std::make_unique<PrintNode>(std::move(expr));
}
