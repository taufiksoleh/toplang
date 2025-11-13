/// Bytecode instruction set for TopLang VM
///
/// This defines a stack-based bytecode format that is much faster to execute
/// than walking the AST tree. Each instruction operates on a value stack.

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Constants and Variables
    /// Push a constant value onto the stack
    LoadConst(usize),

    /// Load a variable value onto the stack
    LoadVar(usize),

    /// Store top of stack into a variable
    StoreVar(usize),

    /// Load a global variable onto the stack
    LoadGlobal(String),

    /// Store top of stack into a global variable
    StoreGlobal(String),

    // Arithmetic Operations
    /// Pop two values, add them, push result
    Add,

    /// Pop two values, subtract them, push result
    Subtract,

    /// Pop two values, multiply them, push result
    Multiply,

    /// Pop two values, divide them, push result
    Divide,

    /// Pop two values, modulo them, push result
    Modulo,

    /// Negate top of stack
    Negate,

    // Comparison Operations
    /// Pop two values, compare equality, push boolean
    Equal,

    /// Pop two values, compare inequality, push boolean
    NotEqual,

    /// Pop two values, compare greater than, push boolean
    Greater,

    /// Pop two values, compare greater or equal, push boolean
    GreaterEqual,

    /// Pop two values, compare less than, push boolean
    Less,

    /// Pop two values, compare less or equal, push boolean
    LessEqual,

    // Logical Operations
    /// Pop two values, logical AND, push boolean
    And,

    /// Pop two values, logical OR, push boolean
    Or,

    /// Pop value, logical NOT, push boolean
    Not,

    // Control Flow
    /// Unconditional jump to instruction at offset
    Jump(usize),

    /// Pop value, jump if false
    JumpIfFalse(usize),

    /// Pop value, jump if true
    JumpIfTrue(usize),

    /// Call function with N arguments (pops N values from stack)
    Call(String, usize),

    /// Return from function (optionally with value on stack)
    Return,

    /// Return null value
    ReturnNull,

    // Stack Operations
    /// Pop and discard top of stack
    Pop,

    /// Duplicate top of stack
    Dup,

    // Array Operations
    /// Pop N values and create an array
    MakeArray(usize),

    /// Pop index and array, push array[index]
    GetIndex,

    /// Pop value, index, and array, set array[index] = value
    SetIndex,

    // String Operations
    /// Pop string, push length
    Length,

    /// Pop string, push uppercase
    Uppercase,

    /// Pop to, from, string, push substring
    Substring,

    // I/O Operations
    /// Pop value and print it
    Print,

    /// Read input with optional prompt (push string onto stack)
    Input(Option<String>),

    // Special
    /// Halt execution
    Halt,

    /// No operation
    Nop,
}

/// A chunk of bytecode with associated constant pool
#[derive(Debug, Clone)]
pub struct Chunk {
    /// The bytecode instructions
    pub code: Vec<Instruction>,

    /// Constant pool (for numbers, strings, etc.)
    pub constants: Vec<Constant>,

    /// Function chunks (name -> chunk)
    pub functions: std::collections::HashMap<String, Chunk>,

    /// Line number information for debugging
    pub lines: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            functions: std::collections::HashMap::new(),
            lines: Vec::new(),
        }
    }

    /// Add a constant to the pool and return its index
    pub fn add_constant(&mut self, constant: Constant) -> usize {
        // Check if constant already exists to save space
        for (i, c) in self.constants.iter().enumerate() {
            if c == &constant {
                return i;
            }
        }

        self.constants.push(constant);
        self.constants.len() - 1
    }

    /// Add an instruction to the chunk
    pub fn emit(&mut self, instruction: Instruction, line: usize) {
        self.code.push(instruction);
        self.lines.push(line);
    }

    /// Get the current instruction pointer (for jump targets)
    pub fn current_position(&self) -> usize {
        self.code.len()
    }

    /// Patch a jump instruction at the given position
    pub fn patch_jump(&mut self, position: usize, target: usize) {
        match &mut self.code[position] {
            Instruction::Jump(ref mut offset) |
            Instruction::JumpIfFalse(ref mut offset) |
            Instruction::JumpIfTrue(ref mut offset) => {
                *offset = target;
            }
            _ => panic!("Attempted to patch non-jump instruction"),
        }
    }

    /// Disassemble the chunk for debugging
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        for (i, instruction) in self.code.iter().enumerate() {
            print!("{:04} ", i);
            if i > 0 && self.lines[i] == self.lines[i - 1] {
                print!("   | ");
            } else {
                print!("{:4} ", self.lines[i]);
            }
            self.disassemble_instruction(instruction, i);
        }
    }

    pub fn disassemble_instruction(&self, instruction: &Instruction, _offset: usize) {
        match instruction {
            Instruction::LoadConst(idx) => {
                println!("LoadConst {:4} '{:?}'", idx, self.constants[*idx]);
            }
            Instruction::LoadVar(idx) => println!("LoadVar {}", idx),
            Instruction::StoreVar(idx) => println!("StoreVar {}", idx),
            Instruction::LoadGlobal(name) => println!("LoadGlobal '{}'", name),
            Instruction::StoreGlobal(name) => println!("StoreGlobal '{}'", name),
            Instruction::Add => println!("Add"),
            Instruction::Subtract => println!("Subtract"),
            Instruction::Multiply => println!("Multiply"),
            Instruction::Divide => println!("Divide"),
            Instruction::Modulo => println!("Modulo"),
            Instruction::Negate => println!("Negate"),
            Instruction::Equal => println!("Equal"),
            Instruction::NotEqual => println!("NotEqual"),
            Instruction::Greater => println!("Greater"),
            Instruction::GreaterEqual => println!("GreaterEqual"),
            Instruction::Less => println!("Less"),
            Instruction::LessEqual => println!("LessEqual"),
            Instruction::And => println!("And"),
            Instruction::Or => println!("Or"),
            Instruction::Not => println!("Not"),
            Instruction::Jump(target) => println!("Jump -> {:04}", target),
            Instruction::JumpIfFalse(target) => println!("JumpIfFalse -> {:04}", target),
            Instruction::JumpIfTrue(target) => println!("JumpIfTrue -> {:04}", target),
            Instruction::Call(name, arity) => println!("Call '{}' ({})", name, arity),
            Instruction::Return => println!("Return"),
            Instruction::ReturnNull => println!("ReturnNull"),
            Instruction::Pop => println!("Pop"),
            Instruction::Dup => println!("Dup"),
            Instruction::MakeArray(size) => println!("MakeArray {}", size),
            Instruction::GetIndex => println!("GetIndex"),
            Instruction::SetIndex => println!("SetIndex"),
            Instruction::Length => println!("Length"),
            Instruction::Uppercase => println!("Uppercase"),
            Instruction::Substring => println!("Substring"),
            Instruction::Print => println!("Print"),
            Instruction::Input(prompt) => {
                if let Some(p) = prompt {
                    println!("Input '{}'", p);
                } else {
                    println!("Input");
                }
            }
            Instruction::Halt => println!("Halt"),
            Instruction::Nop => println!("Nop"),
        }
    }
}
