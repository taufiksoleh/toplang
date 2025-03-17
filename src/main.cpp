#include "lexer.h"
#include "parser.h"
#include "codegen.h"
#include <iostream>
#include <fstream>
#include <string>
#include <exception>

void printUsage() {
    std::cout << "Usage: top <filename> [options]" << std::endl;
    std::cout << "Options:" << std::endl;
    std::cout << "  --emit-llvm      Save the generated LLVM IR to <filename>.ll" << std::endl;
    std::cout << "  --no-exec        Don't execute the program" << std::endl;
    std::cout << "  --compile <name> Compile to executable with the specified name" << std::endl;
    std::cout << "  --exec-ir <file> Execute the specified LLVM IR file directly" << std::endl;
}

int main(int argc, char *argv[]) {
    try {
        // Check for direct IR execution mode
        if (argc >= 3 && std::string(argv[1]) == "--exec-ir") {
            std::string irFilename = argv[2];
            std::cout << "Executing IR file: " << irFilename << std::endl;
            CodeGenerator::executeIRFile(irFilename);
            return 0;
        }

        if (argc < 2) {
            printUsage();
            return 1;
        }
        
        std::string filename = argv[1];
        bool emitLLVM = false;
        bool executeProgram = true;
        bool compileToExecutable = false;
        std::string executableName = "";
        
        // Parse command line options
        for (int i = 2; i < argc; i++) {
            std::string option = argv[i];
            if (option == "--emit-llvm") {
                emitLLVM = true;
            } else if (option == "--no-exec") {
                executeProgram = false;
            } else if (option == "--compile" && i + 1 < argc) {
                compileToExecutable = true;
                executableName = argv[++i];
            } else {
                std::cerr << "Unknown option: " << option << std::endl;
                printUsage();
                return 1;
            }
        }
        
        std::ifstream file(filename);
        
        if (!file.is_open()) {
            std::cerr << "Error: Could not open file " << filename << std::endl;
            return 1;
        }
        
        std::string source((std::istreambuf_iterator<char>(file)), 
                        std::istreambuf_iterator<char>());
        file.close();
        
        std::cout << "Compiling " << filename << "..." << std::endl;
        
        // Create lexer and tokenize the input
        Lexer lexer(source);
        auto tokens = lexer.tokenize();
        std::cout << "Lexical analysis completed." << std::endl;
        
        // Parse the tokens
        Parser parser(tokens);
        auto ast = parser.parse();
        std::cout << "Parsing completed." << std::endl;
        
        // Generate LLVM IR
        CodeGenerator codegen;
        codegen.generate(ast);
        std::cout << "Code generation completed." << std::endl;
        
        // Save LLVM IR to file if requested
        if (emitLLVM) {
            std::string irFilename = filename + ".ll";
            codegen.saveIRToFile(irFilename);
            std::cout << "LLVM IR saved to: " << irFilename << std::endl;
        }
        
        // Compile to executable if requested
        if (compileToExecutable) {
            codegen.compileToExecutable(executableName);
        }
        
        // Execute the compiled code
        if (executeProgram) {
            std::cout << "Executing program..." << std::endl;
            codegen.executeCode();
        }
        
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "ERROR: " << e.what() << std::endl;
        return 1;
    } catch (...) {
        std::cerr << "ERROR: Unknown exception occurred" << std::endl;
        return 1;
    }
}
