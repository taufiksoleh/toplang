#pragma once
#include <string>
#include <vector>
#include <memory>

// Forward declaration
class CodeGenerator;

// Base AST node class
class ASTNode {
public:
    virtual ~ASTNode() = default;
    virtual void accept(CodeGenerator& generator) = 0;
};

// Program node (root of AST)
class ProgramNode : public ASTNode {
public:
    std::vector<std::unique_ptr<ASTNode>> statements;
    
    void accept(CodeGenerator& generator) override;
};

// Block node (for code blocks)
class BlockNode : public ASTNode {
public:
    std::vector<std::unique_ptr<ASTNode>> statements;
    
    void accept(CodeGenerator& generator) override;
};

// Variable declaration node
class VariableDeclarationNode : public ASTNode {
public:
    std::string name;
    bool isConstant;
    std::unique_ptr<ASTNode> initialValue;
    
    VariableDeclarationNode(std::string varName, bool isConst, std::unique_ptr<ASTNode> value)
        : name(std::move(varName)), isConstant(isConst), initialValue(std::move(value)) {}
    
    void accept(CodeGenerator& generator) override;
};

// Binary operation node
class BinaryOpNode : public ASTNode {
public:
    enum class Op { Add, Subtract, Multiply, Divide, Assign, Equals, NotEquals, Greater, Less };
    
    Op operation;
    std::unique_ptr<ASTNode> left;
    std::unique_ptr<ASTNode> right;
    
    BinaryOpNode(Op op, std::unique_ptr<ASTNode> l, std::unique_ptr<ASTNode> r)
        : operation(op), left(std::move(l)), right(std::move(r)) {}
    
    void accept(CodeGenerator& generator) override;
};

// Number literal node
class NumberNode : public ASTNode {
public:
    double value;
    
    explicit NumberNode(double val) : value(val) {}
    
    void accept(CodeGenerator& generator) override;
};

// String literal node
class StringNode : public ASTNode {
public:
    std::string value;
    
    explicit StringNode(std::string val) : value(std::move(val)) {}
    
    void accept(CodeGenerator& generator) override;
};

// Identifier node
class IdentifierNode : public ASTNode {
public:
    std::string name;
    
    explicit IdentifierNode(std::string id) : name(std::move(id)) {}
    
    void accept(CodeGenerator& generator) override;
};

// Function declaration node
class FunctionNode : public ASTNode {
public:
    std::string name;
    std::vector<std::string> parameters;
    std::unique_ptr<BlockNode> body;
    
    FunctionNode(std::string funcName, std::vector<std::string> params, std::unique_ptr<BlockNode> funcBody)
        : name(std::move(funcName)), parameters(std::move(params)), body(std::move(funcBody)) {}
    
    void accept(CodeGenerator& generator) override;
};

// Function call node
class CallNode : public ASTNode {
public:
    std::string callee;
    std::vector<std::unique_ptr<ASTNode>> arguments;
    
    CallNode(std::string funcName, std::vector<std::unique_ptr<ASTNode>> args)
        : callee(std::move(funcName)), arguments(std::move(args)) {}
    
    void accept(CodeGenerator& generator) override;
};

// If statement node
class IfNode : public ASTNode {
public:
    std::unique_ptr<ASTNode> condition;
    std::unique_ptr<BlockNode> thenBlock;
    std::unique_ptr<BlockNode> elseBlock;
    
    IfNode(std::unique_ptr<ASTNode> cond, std::unique_ptr<BlockNode> thenB, std::unique_ptr<BlockNode> elseB)
        : condition(std::move(cond)), thenBlock(std::move(thenB)), elseBlock(std::move(elseB)) {}
    
    void accept(CodeGenerator& generator) override;
};

// While loop node
class WhileNode : public ASTNode {
public:
    std::unique_ptr<ASTNode> condition;
    std::unique_ptr<BlockNode> body;
    
    WhileNode(std::unique_ptr<ASTNode> cond, std::unique_ptr<BlockNode> loopBody)
        : condition(std::move(cond)), body(std::move(loopBody)) {}
    
    void accept(CodeGenerator& generator) override;
};

// Print statement node
class PrintNode : public ASTNode {
public:
    std::unique_ptr<ASTNode> expression;
    
    explicit PrintNode(std::unique_ptr<ASTNode> expr) : expression(std::move(expr)) {}
    
    void accept(CodeGenerator& generator) override;
};

// Return statement node
class ReturnNode : public ASTNode {
public:
    std::unique_ptr<ASTNode> value;
    
    explicit ReturnNode(std::unique_ptr<ASTNode> val) : value(std::move(val)) {}
    
    void accept(CodeGenerator& generator) override;
};
