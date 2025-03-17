#include "codegen.h"
#include <iostream>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/Type.h>
#include <llvm/IR/Verifier.h>
#include <llvm/Support/raw_ostream.h>
#include <llvm/ExecutionEngine/MCJIT.h>
#include <llvm/Support/TargetSelect.h>
#include <llvm/IR/LegacyPassManager.h>
#include <llvm/Transforms/InstCombine/InstCombine.h>
#include <llvm/Transforms/Scalar.h>
#include <llvm/Transforms/Scalar/GVN.h>
#include <fstream>

// External print functions that will be called by our generated code
extern "C" {
    void printDouble(double value) {
        std::cout << value << std::endl;
    }
    
    void printString(const char* str) {
        std::cout << str << std::endl;
    }
}

CodeGenerator::CodeGenerator() {
    // Initialize LLVM components
    llvm::InitializeNativeTarget();
    llvm::InitializeNativeTargetAsmPrinter();
    llvm::InitializeNativeTargetAsmParser();
    
    context = std::make_unique<llvm::LLVMContext>();
    builder = std::make_unique<llvm::IRBuilder<>>(*context);
    module = std::make_unique<llvm::Module>("TopLang Module", *context);
    
    // Declare standard library functions
    declareStandardFunctions();
}

void CodeGenerator::declareStandardFunctions() {
    // Declare printDouble function
    std::vector<llvm::Type*> printDoubleArgs(1, llvm::Type::getDoubleTy(*context));
    llvm::FunctionType* printDoubleFT = llvm::FunctionType::get(
        llvm::Type::getVoidTy(*context), printDoubleArgs, false);
    llvm::Function::Create(printDoubleFT, llvm::Function::ExternalLinkage, 
                          "printDouble", module.get());
    
    // Declare printString function
    // Use PointerType::get to create a pointer type
    std::vector<llvm::Type*> printStringArgs(1, llvm::PointerType::get(llvm::Type::getInt8Ty(*context), 0));
    llvm::FunctionType* printStringFT = llvm::FunctionType::get(
        llvm::Type::getVoidTy(*context), printStringArgs, false);
    llvm::Function::Create(printStringFT, llvm::Function::ExternalLinkage, 
                          "printString", module.get());
}

void CodeGenerator::generate(std::unique_ptr<ASTNode>& ast) {
    if (ast) {
        ast->accept(*this);
    }
    
    // Dump the IR to standard output for debugging
    dumpIR();
}

void CodeGenerator::dumpIR() {
    std::cout << "\n=== Generated LLVM IR ===\n";
    module->print(llvm::outs(), nullptr);
    std::cout << "=========================\n";
}

void CodeGenerator::saveIRToFile(const std::string& filename) {
    std::error_code EC;
    llvm::raw_fd_ostream dest(filename, EC);
    
    if (EC) {
        std::cerr << "Could not open file: " << EC.message() << std::endl;
        return;
    }
    
    // Write the module IR to the file
    module->print(dest, nullptr);
    dest.close();
}

void CodeGenerator::executeCode() {
    std::cout << "Setting up execution engine..." << std::endl;
    
    // Create a temporary IR file to use with lli
    std::string irFilename = "temp_program.ll";
    saveIRToFile(irFilename);
    
    // Execute using the executeIRFile method which handles external functions
    executeIRFile(irFilename);
    
    // Clean up the temporary file
    std::remove(irFilename.c_str());
    
    // Create a new module for future code generation
    module = std::make_unique<llvm::Module>("TopLang Module", *context);
    declareStandardFunctions();
}

// Static method to execute an IR file directly
void CodeGenerator::executeIRFile(const std::string& irFilename) {
    // Initialize LLVM components if not already initialized
    static bool initialized = false;
    if (!initialized) {
        llvm::InitializeNativeTarget();
        llvm::InitializeNativeTargetAsmPrinter();
        llvm::InitializeNativeTargetAsmParser();
        initialized = true;
    }
    
    std::cout << "Creating helper functions..." << std::endl;
    
    // Create helper C file with external functions
    std::string helperFilename = "print_helpers.c";
    {
        std::ofstream helperFile(helperFilename);
        helperFile << "#include <stdio.h>\n\n";
        helperFile << "void printDouble(double value) {\n";
        helperFile << "    printf(\"%f\\n\", value);\n";
        helperFile << "}\n\n";
        helperFile << "void printString(const char* str) {\n";
        helperFile << "    printf(\"%s\\n\", str);\n";
        helperFile << "}\n";
        helperFile.close();
    }
    
    // Compile the helper file
    std::cout << "Compiling helpers..." << std::endl;
    if (system("clang -c print_helpers.c -o print_helpers.o") != 0) {
        std::cerr << "Failed to compile helper functions" << std::endl;
        std::remove("print_helpers.c");
        return;
    }
    
    // Execute the IR file with the helper object
    std::string command = "lli --extra-object=print_helpers.o " + irFilename;
    std::cout << "Executing program..." << std::endl;
    int result = system(command.c_str());
    
    // Clean up temporary files
    std::remove("print_helpers.c");
    std::remove("print_helpers.o");
    
    if (result != 0) {
        std::cerr << "Error: Failed to execute program" << std::endl;
    }
}

void CodeGenerator::compileToExecutable(const std::string& outputFilename) {
    // Save IR to temporary file
    std::string irFilename = outputFilename + ".ll";
    saveIRToFile(irFilename);
    
    // Create a C file with helper functions
    std::string helperFilename = outputFilename + "_helpers.c";
    {
        std::ofstream helperFile(helperFilename);
        helperFile << "#include <stdio.h>\n\n";
        helperFile << "void printDouble(double value) {\n";
        helperFile << "    printf(\"%f\\n\", value);\n";
        helperFile << "}\n\n";
        helperFile << "void printString(const char* str) {\n";
        helperFile << "    printf(\"%s\\n\", str);\n";
        helperFile.close();
    }
    
    // Compile the IR and helper file to an executable
    std::cout << "Compiling to executable..." << std::endl;
    
    // Use system to run the necessary commands
    std::string compileCommand = "clang " + irFilename + " " + helperFilename + " -o " + outputFilename;
    if (system(compileCommand.c_str()) != 0) {
        std::cerr << "Failed to compile executable" << std::endl;
        return;
    }
    
    // Clean up temporary files
    std::remove(irFilename.c_str());
    std::remove(helperFilename.c_str());
    
    std::cout << "Executable created: " << outputFilename << std::endl;
}

void CodeGenerator::visit(ProgramNode& node) {
    // Process all top-level statements
    for (auto& statement : node.statements) {
        if (statement) {
            statement->accept(*this);
        }
    }
}

void CodeGenerator::visit(BlockNode& node) {
    // Process all statements in the block
    for (auto& statement : node.statements) {
        if (statement) {
            statement->accept(*this);
        }
    }
}

void CodeGenerator::visit(VariableDeclarationNode& node) {
    // Generate code for the initial value
    if (node.initialValue) {
        node.initialValue->accept(*this);
        llvm::Value* initialValue = currentValue;
        
        // Create an alloca instruction for the variable
        llvm::Function* function = builder->GetInsertBlock()->getParent();
        llvm::IRBuilder<> tempBuilder(&function->getEntryBlock(), 
                                      function->getEntryBlock().begin());
        
        llvm::AllocaInst* alloca = tempBuilder.CreateAlloca(
            llvm::Type::getDoubleTy(*context), 0, node.name);
        
        // Store the initial value
        builder->CreateStore(initialValue, alloca);
        
        // Add to symbol table
        namedValues[node.name] = alloca;
    }
}

llvm::Value* CodeGenerator::generateBinaryOp(BinaryOpNode& node) {
    // Generate code for left and right operands
    node.left->accept(*this);
    llvm::Value* left = currentValue;
    
    node.right->accept(*this);
    llvm::Value* right = currentValue;
    
    if (!left || !right) {
        return nullptr;
    }
    
    // Generate appropriate operation based on the operator
    switch (node.operation) {
        case BinaryOpNode::Op::Add:
            return builder->CreateFAdd(left, right, "addtmp");
        case BinaryOpNode::Op::Subtract:
            return builder->CreateFSub(left, right, "subtmp");
        case BinaryOpNode::Op::Multiply:
            return builder->CreateFMul(left, right, "multmp");
        case BinaryOpNode::Op::Divide:
            return builder->CreateFDiv(left, right, "divtmp");
        case BinaryOpNode::Op::Assign: {
            // Handle assignment - the left should be a variable reference
            auto* leftIdent = dynamic_cast<IdentifierNode*>(node.left.get());
            if (!leftIdent) {
                std::cerr << "Left side of assignment must be a variable" << std::endl;
                return nullptr;
            }
            
            // Look up the variable in the symbol table
            llvm::Value* variable = namedValues[leftIdent->name];
            if (!variable) {
                std::cerr << "Unknown variable: " << leftIdent->name << std::endl;
                return nullptr;
            }
            
            // Store the right value into the variable
            builder->CreateStore(right, variable);
            return right;
        }
        case BinaryOpNode::Op::Equals:
            // Direct comparison - creates an i1 value (boolean)
            return builder->CreateFCmpOEQ(left, right, "eqtmp");
        case BinaryOpNode::Op::NotEquals:
            // Direct comparison - creates an i1 value (boolean)
            return builder->CreateFCmpONE(left, right, "neqtmp");
        case BinaryOpNode::Op::Greater:
            // Direct comparison - creates an i1 value (boolean)
            return builder->CreateFCmpOGT(left, right, "gttmp");
        case BinaryOpNode::Op::Less:
            // Direct comparison - creates an i1 value (boolean)
            return builder->CreateFCmpOLT(left, right, "lttmp");
        default:
            std::cerr << "Unknown binary operator" << std::endl;
            return nullptr;
    }
}

void CodeGenerator::visit(BinaryOpNode& node) {
    currentValue = generateBinaryOp(node);
}

void CodeGenerator::visit(NumberNode& node) {
    currentValue = llvm::ConstantFP::get(*context, llvm::APFloat(node.value));
}

void CodeGenerator::visit(StringNode& node) {
    currentValue = builder->CreateGlobalStringPtr(node.value, "str");
}

void CodeGenerator::visit(IdentifierNode& node) {
    // Look up the variable in the symbol table
    llvm::Value* variable = namedValues[node.name];
    if (!variable) {
        std::cerr << "Unknown variable: " << node.name << std::endl;
        currentValue = nullptr;
        return;
    }
    
    // Load the value from the variable
    currentValue = builder->CreateLoad(llvm::Type::getDoubleTy(*context), variable, node.name.c_str());
}

void CodeGenerator::visit(FunctionNode& node) {
    // Create LLVM function type
    std::vector<llvm::Type*> paramTypes(node.parameters.size(), 
                                      llvm::Type::getDoubleTy(*context));
    
    llvm::FunctionType* funcType = llvm::FunctionType::get(
        llvm::Type::getDoubleTy(*context), paramTypes, false);
    
    // Create the function
    llvm::Function* function = llvm::Function::Create(
        funcType, llvm::Function::ExternalLinkage, node.name, module.get());
    
    // Set parameter names
    unsigned idx = 0;
    for (auto& arg : function->args()) {
        if (idx < node.parameters.size()) {
            arg.setName(node.parameters[idx++]);
        }
    }
    
    // Create the entry basic block
    llvm::BasicBlock* entryBB = llvm::BasicBlock::Create(*context, "entry", function);
    builder->SetInsertPoint(entryBB);
    
    // Save old symbol table and create a new scope
    std::map<std::string, llvm::Value*> oldNamedValues = namedValues;
    namedValues.clear();
    
    // Add the arguments to the symbol table
    idx = 0;
    for (auto& arg : function->args()) {
        // Create an alloca for this argument
        llvm::AllocaInst* alloca = builder->CreateAlloca(
            llvm::Type::getDoubleTy(*context), 0, node.parameters[idx++]);
        
        // Store the initial value
        builder->CreateStore(&arg, alloca);
        
        // Add to symbol table
        namedValues[std::string(arg.getName())] = alloca;
    }
    
    // Generate code for the function body
    if (node.body) {
        node.body->accept(*this);
    }
    
    // Add a return instruction if one doesn't exist
    if (!builder->GetInsertBlock()->getTerminator()) {
        builder->CreateRet(llvm::ConstantFP::get(*context, llvm::APFloat(0.0)));
    }
    
    // Verify the function
    llvm::verifyFunction(*function);
    
    // Restore the old symbol table
    namedValues = std::move(oldNamedValues);
}

void CodeGenerator::visit(CallNode& node) {
    // Look up the function in the module
    llvm::Function* calleeF = module->getFunction(node.callee);
    if (!calleeF) {
        std::cerr << "Unknown function: " << node.callee << std::endl;
        currentValue = nullptr;
        return;
    }
    
    // Check argument count
    if (calleeF->arg_size() != node.arguments.size()) {
        std::cerr << "Incorrect number of arguments for function " << node.callee << std::endl;
        currentValue = nullptr;
        return;
    }
    
    // Generate code for each argument
    std::vector<llvm::Value*> argsV;
    for (auto& arg : node.arguments) {
        arg->accept(*this);
        argsV.push_back(currentValue);
    }
    
    // Create the call instruction
    currentValue = builder->CreateCall(calleeF, argsV, "calltmp");
}

void CodeGenerator::visit(IfNode& node) {
    // Generate condition
    node.condition->accept(*this);
    llvm::Value* condValue = currentValue;
    
    // Convert condition to boolean (non-zero is true)
    // If the condition is a comparison result (i1), don't try to compare it with 0.0
    if (!condValue->getType()->isIntegerTy(1)) {
        condValue = builder->CreateFCmpONE(
            condValue, 
            llvm::ConstantFP::get(*context, llvm::APFloat(0.0)),
            "ifcond");
    }
    
    llvm::Function* function = builder->GetInsertBlock()->getParent();
    
    // Create basic blocks for then, else, and merge
    llvm::BasicBlock* thenBB = llvm::BasicBlock::Create(*context, "then", function);
    llvm::BasicBlock* elseBB = llvm::BasicBlock::Create(*context, "else");
    llvm::BasicBlock* mergeBB = llvm::BasicBlock::Create(*context, "ifcont");
    
    // Create conditional branch - use the condition value directly
    builder->CreateCondBr(condValue, thenBB, elseBB);
    
    // Generate 'then' block
    builder->SetInsertPoint(thenBB);
    if (node.thenBlock) {
        node.thenBlock->accept(*this);
    }
    
    // Create branch to merge block if not already terminated
    if (!builder->GetInsertBlock()->getTerminator()) {
        builder->CreateBr(mergeBB);
    }
    
    // Generate 'else' block if it exists
    // Add the else block to the function properly
    elseBB->insertInto(function);
    builder->SetInsertPoint(elseBB);
    
    if (node.elseBlock) {
        node.elseBlock->accept(*this);
    }
    
    // Create branch to merge block if not already terminated
    if (!builder->GetInsertBlock()->getTerminator()) {
        builder->CreateBr(mergeBB);
    }
    
    // Continue with the merge block
    // Add the merge block to the function properly
    mergeBB->insertInto(function);
    builder->SetInsertPoint(mergeBB);
}

void CodeGenerator::visit(WhileNode& node) {
    llvm::Function* function = builder->GetInsertBlock()->getParent();
    
    // Create basic blocks for condition, loop body, and after loop
    llvm::BasicBlock* condBB = llvm::BasicBlock::Create(*context, "loopcond", function);
    llvm::BasicBlock* loopBB = llvm::BasicBlock::Create(*context, "loop");
    llvm::BasicBlock* afterBB = llvm::BasicBlock::Create(*context, "afterloop");
    
    // Jump to condition first
    builder->CreateBr(condBB);
    
    // Generate condition code
    builder->SetInsertPoint(condBB);
    node.condition->accept(*this);
    llvm::Value* condValue = currentValue;
    
    // Convert condition to boolean (non-zero is true)
    // If the condition is a comparison result (i1), don't try to compare it with 0.0
    if (!condValue->getType()->isIntegerTy(1)) {
        condValue = builder->CreateFCmpONE(
            condValue, 
            llvm::ConstantFP::get(*context, llvm::APFloat(0.0)),
            "loopcond");
    }
    
    // Create conditional branch
    builder->CreateCondBr(condValue, loopBB, afterBB);
    
    // Generate loop body
    // Add the loop block to the function properly
    loopBB->insertInto(function);
    builder->SetInsertPoint(loopBB);
    
    if (node.body) {
        node.body->accept(*this);
    }
    
    // Create branch back to condition if not already terminated
    if (!builder->GetInsertBlock()->getTerminator()) {
        builder->CreateBr(condBB);
    }
    
    // Continue with the block after the loop
    // Add the after block to the function properly
    afterBB->insertInto(function);
    builder->SetInsertPoint(afterBB);
}

void CodeGenerator::visit(PrintNode& node) {
    // Generate code for the expression to be printed
    if (node.expression) {
        node.expression->accept(*this);
        
        // Determine if it's a string or number
        llvm::Type* type = currentValue->getType();
        
        // Check for string type - typically a pointer type in LLVM
        // In older LLVM versions, we need to check differently
        if (type->isPointerTy()) {
            // String literal would be represented as a global constant string
            // Just checking if it's a pointer is usually enough for our simple case
            llvm::Function* printFunc = module->getFunction("printString");
            builder->CreateCall(printFunc, {currentValue});
        } else {
            // For numeric expressions
            llvm::Function* printFunc = module->getFunction("printDouble");
            builder->CreateCall(printFunc, {currentValue});
        }
    }
}

void CodeGenerator::visit(ReturnNode& node) {
    // Generate code for the return value
    if (node.value) {
        node.value->accept(*this);
        builder->CreateRet(currentValue);
    } else {
        // Return default value
        builder->CreateRet(llvm::ConstantFP::get(*context, llvm::APFloat(0.0)));
    }
}
