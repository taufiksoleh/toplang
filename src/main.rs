mod ast;
mod bytecode;
mod codegen_c;
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
mod vm_nanbox;
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
use vm_nanbox::NanBoxVM;
use vm_optimized::OptimizedVM;

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

    /// Use NaN-boxed VM for maximum performance (requires --bytecode)
    #[arg(long)]
    nanbox: bool,

    /// Compile to native executable (AOT compilation)
    #[arg(short = 'c', long)]
    compile: bool,

    /// Output file for compiled executable
    #[arg(short = 'o', long, value_name = "FILE")]
    output: Option<PathBuf>,
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

    // Execution: Choose between native compilation, bytecode VM, or interpreter
    let exit_code = if cli.compile {
        // Native AOT compilation
        if cli.verbose {
            println!("{}", "Compiling to native code...".blue().bold());
        }

        // First compile to bytecode
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

        // Then transpile bytecode to C code
        let mut codegen = codegen_c::CCodeGen::new();
        let c_code = codegen
            .compile_chunk(&chunk)
            .with_context(|| "Failed to generate C code")?;

        // Determine output filename
        let output_file = cli.output.clone().unwrap_or_else(|| {
            let mut path = cli.file.clone();
            path.set_extension(if cfg!(windows) { "exe" } else { "" });
            path.file_name()
                .map(|name| PathBuf::from(name))
                .unwrap_or_else(|| PathBuf::from("a.out"))
        });

        // Write C file
        let c_file = output_file.with_extension("c");
        fs::write(&c_file, &c_code)
            .with_context(|| format!("Failed to write C file: {}", c_file.display()))?;

        if cli.verbose {
            println!(
                "{} {}",
                "Generated C file:".blue().bold(),
                c_file.display()
            );
        }

        // Compile C code with optimizations
        let compile_status = if cfg!(target_os = "windows") {
            std::process::Command::new("cl")
                .arg(&c_file)
                .arg(&format!("/Fe:{}", output_file.display()))
                .arg("/O2")
                .status()
        } else {
            // Linux/macOS: use gcc or clang
            std::process::Command::new("cc")
                .arg(&c_file)
                .arg("-o")
                .arg(&output_file)
                .arg("-O3")
                .arg("-march=native")
                .arg("-ffast-math")
                .arg("-lm")
                .status()
        };

        match compile_status {
            Ok(status) if status.success() => {
                if cli.verbose {
                    println!(
                        "{} {}",
                        "Successfully compiled to:".green().bold(),
                        output_file.display()
                    );
                }
                // Clean up C file unless verbose
                if !cli.verbose {
                    let _ = fs::remove_file(&c_file);
                }
                0
            }
            Ok(status) => {
                eprintln!("{} Compilation failed with status: {}", "Error:".red().bold(), status);
                if cli.verbose {
                    eprintln!("The generated C file is at: {}", c_file.display());
                }
                1
            }
            Err(e) => {
                eprintln!("{} Failed to run C compiler: {}", "Error:".red().bold(), e);
                eprintln!("Make sure you have a C compiler (gcc/clang/cl) installed.");
                eprintln!("The generated C file is at: {}", c_file.display());
                1
            }
        }
    } else if cli.bytecode {
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

        // Execute with VM - choose between NaN-boxed or standard optimized VM
        if cli.nanbox {
            // Use NaN-boxed VM for maximum performance
            if cli.verbose {
                println!(
                    "{}",
                    "Executing with NaN-boxed VM (maximum performance)..."
                        .blue()
                        .bold()
                );
                println!();
            }

            let mut vm = NanBoxVM::new();
            if cli.debug_vm {
                vm.set_debug(true);
            }

            vm.execute(chunk)
                .with_context(|| "NaN-boxed VM runtime error")?
        } else {
            // Use standard optimized VM
            if cli.verbose {
                println!("{}", "Executing with optimized VM...".blue().bold());
                println!();
            }

            let mut vm = OptimizedVM::new();
            if cli.debug_vm {
                vm.set_debug(true);
            }

            vm.execute(chunk).with_context(|| "VM runtime error")?
        }
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
