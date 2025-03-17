#pragma once
#include "ast.h"
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Value.h>
#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/ExecutionEngine/GenericValue.h>
#include <map>
#include <memory>
#include <string>

class CodeGenerator {
private:
    std::unique_ptr<llvm::LLVMContext> context;
    std::unique_ptr<llvm::IRBuilder<>> builder;
    std::unique_ptr<llvm::Module> module;
    std::unique_ptr<llvm::ExecutionEngine> executionEngine;
    
    std::map<std::string, llvm::Value*> namedValues;
    llvm::Value* currentValue = nullptr;
    
    llvm::Value* generateBinaryOp(BinaryOpNode& node);
    void declareStandardFunctions();
    
public:
    CodeGenerator();
    
    void generate(std::unique_ptr<ASTNode>& ast);
    void dumpIR();
    void executeCode();
    void saveIRToFile(const std::string& filename);
    void compileToExecutable(const std::string& outputFilename);
    
    // Static method to execute an IR file directly
    static void executeIRFile(const std::string& irFilename);
    
    llvm::Value* getCurrentValue() { return currentValue; }
    void setCurrentValue(llvm::Value* value) { currentValue = value; }
    
    void visit(ProgramNode& node);
    void visit(BlockNode& node);
    void visit(VariableDeclarationNode& node);
    void visit(BinaryOpNode& node);
    void visit(NumberNode& node);
    void visit(StringNode& node);
    void visit(IdentifierNode& node);
    void visit(FunctionNode& node);
    void visit(CallNode& node);
    void visit(IfNode& node);
    void visit(WhileNode& node);
    void visit(PrintNode& node);
    void visit(ReturnNode& node);
};
