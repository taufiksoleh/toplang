mod ast;
mod bytecode;
mod compiler;
mod interpreter;
mod lexer;
mod nanbox;
mod nanbox_safe;
mod optimizer;
mod parser;
mod peephole;
mod token;
mod vm;
mod vm_optimized;
mod vm_threaded;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use colored::Colorize;
use compiler::Compiler;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;
use vm::VM;
use vm_optimized::OptimizedVM;
use vm_threaded::ThreadedVM;

#[derive(ClapParser)]
#[command(name = "topc")]
#[command(author = "TopLang Contributors")]
#[command(version)]
#[command(about = "TopLang - A simple, human-first programming language", long_about = None)]
struct Cli {
    /// The TopLang source file to compile and run
    #[arg(value_name = "FILE")]
    file: PathBuf,

    /// Print tokens after lexing
    #[arg(short = 't', long)]
    show_tokens: bool,

    /// Print AST after parsing
    #[arg(short = 'a', long)]
    show_ast: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Use bytecode compiler and VM (faster execution)
    #[arg(short = 'b', long)]
    bytecode: bool,

    /// Show compiled bytecode (requires --bytecode)
    #[arg(long)]
    show_bytecode: bool,

    /// Debug VM execution (requires --bytecode)
    #[arg(long)]
    debug_vm: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    // Read source file
    let source = fs::read_to_string(&cli.file)
        .with_context(|| format!("Failed to read file: {}", cli.file.display()))?;

    if cli.verbose {
        println!("{} {}", "Reading file:".blue().bold(), cli.file.display());
    }

    // Lexing
    if cli.verbose {
        println!("{}", "Lexing...".blue().bold());
    }

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    if cli.show_tokens {
        println!("\n{}", "=== Tokens ===".yellow().bold());
        for token in &tokens {
            println!("{}:{} - {:?}", token.line, token.column, token.token_type);
        }
        println!();
    }

    // Parsing
    if cli.verbose {
        println!("{}", "Parsing...".blue().bold());
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse().with_context(|| "Failed to parse program")?;

    if cli.show_ast {
        println!("\n{}", "=== AST ===".yellow().bold());
        println!("{:#?}", program);
        println!();
    }

    // Execution: Choose between interpreter or VM
    let exit_code = if cli.bytecode {
        // Bytecode compilation
        if cli.verbose {
            println!("{}", "Compiling to bytecode...".blue().bold());
        }

        let mut compiler = Compiler::new();
        let chunk = compiler
            .compile(program)
            .with_context(|| "Failed to compile to bytecode")?;

        if cli.show_bytecode {
            println!("\n{}", "=== Bytecode ===".yellow().bold());
            chunk.disassemble("main");
            for (name, func_chunk) in &chunk.functions {
                println!();
                func_chunk.disassemble(name);
            }
            println!();
        }

        // Execute with VM (using optimized VM with inline caching and reduced cloning)
        if cli.verbose {
            println!("{}", "Executing with optimized VM...".blue().bold());
            println!();
        }

        let mut vm = OptimizedVM::new();
        if cli.debug_vm {
            vm.set_debug(true);
        }

        vm.execute(chunk).with_context(|| "VM runtime error")?
    } else {
        // Use traditional tree-walking interpreter
        if cli.verbose {
            println!("{}", "Executing with interpreter...".blue().bold());
            println!();
        }

        let mut interpreter = Interpreter::new();
        interpreter
            .interpret(program)
            .with_context(|| "Runtime error")?
    };

    if cli.verbose {
        println!(
            "\n{} {}",
            "Program exited with code:".blue().bold(),
            exit_code
        );
    }

    process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_program() {
        let source = r#"
function main() {
    var x is 10
    print x
    return 0
}
"#
        .to_string();

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().expect("Failed to parse");

        let mut interpreter = Interpreter::new();
        let exit_code = interpreter.interpret(program).expect("Failed to execute");

        assert_eq!(exit_code, 0);
    }

    #[test]
    fn test_arithmetic() {
        let source = r#"
function main() {
    var result is 5 plus 3
    var result2 is result times 2
    return 0
}
"#
        .to_string();

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().expect("Failed to parse");

        let mut interpreter = Interpreter::new();
        let exit_code = interpreter.interpret(program).expect("Failed to execute");

        assert_eq!(exit_code, 0);
    }
}
