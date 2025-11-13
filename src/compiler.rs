/// Compiler that converts AST to bytecode
///
/// This compiler performs a single pass over the AST and generates
/// efficient bytecode instructions for the VM to execute.
use crate::ast::*;
use crate::bytecode::*;
use crate::optimizer;
use crate::peephole;
use anyhow::{anyhow, Result};

/// Local variable information
#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
}

/// Compiler state
pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
    current_line: usize,
    loop_starts: Vec<usize>,
    loop_exits: Vec<Vec<usize>>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
            current_line: 1,
            loop_starts: Vec::new(),
            loop_exits: Vec::new(),
        }
    }

    /// Compile a program (multiple functions) into bytecode
    pub fn compile(&mut self, program: Program) -> Result<Chunk> {
        // First: optimize the program (constant folding, etc.)
        let optimized_program = optimizer::optimize_program(&program);

        // Then compile all functions
        for function in &optimized_program.functions {
            let func_chunk = self.compile_function(function)?;
            self.chunk
                .functions
                .insert(function.name.clone(), func_chunk);
        }

        // Generate main entry point that calls main()
        self.chunk.emit(Instruction::Call("main".to_string(), 0), 1);
        self.chunk.emit(Instruction::Halt, 1);

        // Apply peephole optimizations
        peephole::optimize_chunk(&mut self.chunk);

        Ok(self.chunk.clone())
    }

    /// Compile a single function into its own chunk
    fn compile_function(&mut self, function: &Function) -> Result<Chunk> {
        // Save current state
        let saved_chunk = self.chunk.clone();
        let saved_locals = self.locals.clone();
        let saved_scope_depth = self.scope_depth;

        // Create new chunk for this function
        self.chunk = Chunk::new();
        self.locals.clear();
        self.scope_depth = 0;

        // Create locals for parameters
        self.begin_scope();
        for param in &function.params {
            self.add_local(param.clone())?;
        }

        // Compile function body
        for stmt in &function.body {
            self.compile_stmt(stmt)?;
        }

        // Ensure function returns null if no explicit return
        if self.chunk.code.is_empty()
            || !matches!(
                self.chunk.code.last(),
                Some(Instruction::Return) | Some(Instruction::ReturnNull)
            )
        {
            self.chunk.emit(Instruction::ReturnNull, self.current_line);
        }

        // Get the compiled chunk
        let func_chunk = self.chunk.clone();

        // Restore state
        self.chunk = saved_chunk;
        self.locals = saved_locals;
        self.scope_depth = saved_scope_depth;

        Ok(func_chunk)
    }

    /// Compile a statement
    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::VarDecl {
                name,
                value,
                is_const: _,
            } => {
                self.compile_expr(value)?;

                if self.scope_depth == 0 {
                    // Global variable
                    self.chunk
                        .emit(Instruction::StoreGlobal(name.clone()), self.current_line);
                } else {
                    // Local variable
                    self.add_local(name.clone())?;
                }
                Ok(())
            }

            Stmt::Assignment { name, value } => {
                self.compile_expr(value)?;

                // Try to find as local first
                if let Some(local_idx) = self.resolve_local(name) {
                    self.chunk
                        .emit(Instruction::StoreVar(local_idx), self.current_line);
                } else {
                    // Global variable
                    self.chunk
                        .emit(Instruction::StoreGlobal(name.clone()), self.current_line);
                }
                Ok(())
            }

            Stmt::IndexAssignment {
                array,
                index,
                value,
            } => {
                // Compile array expression
                self.compile_expr(array)?;
                // Compile index expression
                self.compile_expr(index)?;
                // Compile value expression
                self.compile_expr(value)?;
                // Set the index
                self.chunk.emit(Instruction::SetIndex, self.current_line);
                Ok(())
            }

            Stmt::Print(expr) => {
                self.compile_expr(expr)?;
                self.chunk.emit(Instruction::Print, self.current_line);
                Ok(())
            }

            Stmt::Ask { name, prompt } => {
                // For simplicity, we expect a string literal
                let prompt_str = if let Some(Expr::String(s)) = prompt {
                    Some(s.clone())
                } else {
                    None
                };

                self.chunk
                    .emit(Instruction::Input(prompt_str), self.current_line);

                // Store the result
                if let Some(local_idx) = self.resolve_local(name) {
                    self.chunk
                        .emit(Instruction::StoreVar(local_idx), self.current_line);
                } else {
                    self.chunk
                        .emit(Instruction::StoreGlobal(name.clone()), self.current_line);
                }
                Ok(())
            }

            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                // Compile condition
                self.compile_expr(condition)?;

                // Jump if false to else block (or end if no else)
                let jump_to_else = self.chunk.current_position();
                self.chunk
                    .emit(Instruction::JumpIfFalse(0), self.current_line);

                // Compile then block
                self.begin_scope();
                for stmt in then_block {
                    self.compile_stmt(stmt)?;
                }
                self.end_scope();

                if let Some(else_stmts) = else_block {
                    // Jump over else block from then block
                    let jump_to_end = self.chunk.current_position();
                    self.chunk.emit(Instruction::Jump(0), self.current_line);

                    // Patch jump to else
                    let else_start = self.chunk.current_position();
                    self.chunk.patch_jump(jump_to_else, else_start);

                    // Compile else block
                    self.begin_scope();
                    for stmt in else_stmts {
                        self.compile_stmt(stmt)?;
                    }
                    self.end_scope();

                    // Patch jump to end
                    let end = self.chunk.current_position();
                    self.chunk.patch_jump(jump_to_end, end);
                } else {
                    // No else block, just patch the jump
                    let end = self.chunk.current_position();
                    self.chunk.patch_jump(jump_to_else, end);
                }

                Ok(())
            }

            Stmt::While { condition, body } => {
                let loop_start = self.chunk.current_position();
                self.loop_starts.push(loop_start);
                self.loop_exits.push(Vec::new());

                // Compile condition
                self.compile_expr(condition)?;

                // Jump to end if false
                let exit_jump = self.chunk.current_position();
                self.chunk
                    .emit(Instruction::JumpIfFalse(0), self.current_line);

                // Compile body
                self.begin_scope();
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }
                self.end_scope();

                // Jump back to start
                self.chunk
                    .emit(Instruction::Jump(loop_start), self.current_line);

                // Patch exit jump
                let end = self.chunk.current_position();
                self.chunk.patch_jump(exit_jump, end);

                // Patch all break jumps
                if let Some(exits) = self.loop_exits.pop() {
                    for exit_pos in exits {
                        self.chunk.patch_jump(exit_pos, end);
                    }
                }
                self.loop_starts.pop();

                Ok(())
            }

            Stmt::For {
                init,
                condition,
                increment,
                body,
            } => {
                self.begin_scope();

                // Compile initialization
                self.compile_stmt(init)?;

                let loop_start = self.chunk.current_position();
                self.loop_starts.push(loop_start);
                self.loop_exits.push(Vec::new());

                // Compile condition
                self.compile_expr(condition)?;

                // Jump to end if false
                let exit_jump = self.chunk.current_position();
                self.chunk
                    .emit(Instruction::JumpIfFalse(0), self.current_line);

                // Compile body
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }

                // Compile increment
                self.compile_stmt(increment)?;

                // Jump back to condition
                self.chunk
                    .emit(Instruction::Jump(loop_start), self.current_line);

                // Patch exit jump
                let end = self.chunk.current_position();
                self.chunk.patch_jump(exit_jump, end);

                // Patch all break jumps
                if let Some(exits) = self.loop_exits.pop() {
                    for exit_pos in exits {
                        self.chunk.patch_jump(exit_pos, end);
                    }
                }
                self.loop_starts.pop();

                self.end_scope();
                Ok(())
            }

            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.compile_expr(e)?;
                    self.chunk.emit(Instruction::Return, self.current_line);
                } else {
                    self.chunk.emit(Instruction::ReturnNull, self.current_line);
                }
                Ok(())
            }

            Stmt::Break => {
                let jump_pos = self.chunk.current_position();
                self.chunk.emit(Instruction::Jump(0), self.current_line);

                // Add to current loop's exit list
                if let Some(exits) = self.loop_exits.last_mut() {
                    exits.push(jump_pos);
                }
                Ok(())
            }

            Stmt::Continue => {
                if let Some(&loop_start) = self.loop_starts.last() {
                    self.chunk
                        .emit(Instruction::Jump(loop_start), self.current_line);
                }
                Ok(())
            }

            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
                self.chunk.emit(Instruction::Pop, self.current_line);
                Ok(())
            }
        }
    }

    /// Compile an expression
    fn compile_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Number(n) => {
                let const_idx = self.chunk.add_constant(Constant::Number(*n));
                self.chunk
                    .emit(Instruction::LoadConst(const_idx), self.current_line);
                Ok(())
            }

            Expr::String(s) => {
                let const_idx = self.chunk.add_constant(Constant::String(s.clone()));
                self.chunk
                    .emit(Instruction::LoadConst(const_idx), self.current_line);
                Ok(())
            }

            Expr::Boolean(b) => {
                let const_idx = self.chunk.add_constant(Constant::Boolean(*b));
                self.chunk
                    .emit(Instruction::LoadConst(const_idx), self.current_line);
                Ok(())
            }

            Expr::Identifier(name) => {
                // Try local first
                if let Some(local_idx) = self.resolve_local(name) {
                    self.chunk
                        .emit(Instruction::LoadVar(local_idx), self.current_line);
                } else {
                    // Global variable
                    self.chunk
                        .emit(Instruction::LoadGlobal(name.clone()), self.current_line);
                }
                Ok(())
            }

            Expr::Binary { left, op, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;

                let instruction = match op {
                    BinaryOp::Add => Instruction::Add,
                    BinaryOp::Subtract => Instruction::Subtract,
                    BinaryOp::Multiply => Instruction::Multiply,
                    BinaryOp::Divide => Instruction::Divide,
                    BinaryOp::Modulo => Instruction::Modulo,
                    BinaryOp::Equals => Instruction::Equal,
                    BinaryOp::NotEquals => Instruction::NotEqual,
                    BinaryOp::Greater => Instruction::Greater,
                    BinaryOp::GreaterOrEquals => Instruction::GreaterEqual,
                    BinaryOp::Less => Instruction::Less,
                    BinaryOp::LessOrEquals => Instruction::LessEqual,
                    BinaryOp::And => Instruction::And,
                    BinaryOp::Or => Instruction::Or,
                };

                self.chunk.emit(instruction, self.current_line);
                Ok(())
            }

            Expr::Unary { op, operand } => {
                self.compile_expr(operand)?;

                let instruction = match op {
                    UnaryOp::Not => Instruction::Not,
                    UnaryOp::Negate => Instruction::Negate,
                    UnaryOp::Length => Instruction::Length,
                    UnaryOp::Uppercase => Instruction::Uppercase,
                };

                self.chunk.emit(instruction, self.current_line);
                Ok(())
            }

            Expr::Call { name, args } => {
                // Compile arguments (they'll be on the stack)
                for arg in args {
                    self.compile_expr(arg)?;
                }

                self.chunk.emit(
                    Instruction::Call(name.clone(), args.len()),
                    self.current_line,
                );
                Ok(())
            }

            Expr::Array(elements) => {
                // Compile all elements onto the stack
                for elem in elements {
                    self.compile_expr(elem)?;
                }

                self.chunk
                    .emit(Instruction::MakeArray(elements.len()), self.current_line);
                Ok(())
            }

            Expr::Index { array, index } => {
                self.compile_expr(array)?;
                self.compile_expr(index)?;
                self.chunk.emit(Instruction::GetIndex, self.current_line);
                Ok(())
            }

            Expr::Substring { string, from, to } => {
                self.compile_expr(string)?;
                self.compile_expr(from)?;
                self.compile_expr(to)?;
                self.chunk.emit(Instruction::Substring, self.current_line);
                Ok(())
            }
        }
    }

    // Scope management
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // Pop all locals from this scope
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            self.chunk.emit(Instruction::Pop, self.current_line);
            self.locals.pop();
        }
    }

    fn add_local(&mut self, name: String) -> Result<()> {
        // Check for duplicate in current scope
        for local in self.locals.iter().rev() {
            if local.depth < self.scope_depth {
                break;
            }
            if local.name == name {
                return Err(anyhow!(
                    "Variable '{}' already declared in this scope",
                    name
                ));
            }
        }

        self.locals.push(Local {
            name,
            depth: self.scope_depth,
        });
        Ok(())
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        // Search backwards through locals
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i);
            }
        }
        None
    }
}
