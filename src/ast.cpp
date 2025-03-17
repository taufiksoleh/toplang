#include "ast.h"
#include "codegen.h"

// Implementation of the accept methods for each AST node

void ProgramNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void BlockNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void VariableDeclarationNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void BinaryOpNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void NumberNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void StringNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void IdentifierNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void FunctionNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void CallNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void IfNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void WhileNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void PrintNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}

void ReturnNode::accept(CodeGenerator& generator) {
    generator.visit(*this);
}
