/// Virtual Machine for executing TopLang bytecode
///
/// This is a stack-based VM that executes bytecode instructions in a tight loop.
/// This approach is much faster than walking the AST tree.
use crate::bytecode::*;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{self, Write};

/// Runtime value types
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Null,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Null => false,
        }
    }

    pub fn as_number(&self) -> Result<f64> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(anyhow!("Expected number, got {:?}", self)),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Err(anyhow!("Expected string, got {:?}", self)),
        }
    }

    pub fn as_array(&self) -> Result<Vec<Value>> {
        match self {
            Value::Array(a) => Ok(a.clone()),
            _ => Err(anyhow!("Expected array, got {:?}", self)),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Null => write!(f, "null"),
        }
    }
}

/// Call frame for function calls
#[derive(Debug, Clone)]
struct CallFrame {
    chunk: Chunk,
    ip: usize,
    stack_base: usize,
}

/// The Virtual Machine
pub struct VM {
    /// Value stack
    stack: Vec<Value>,

    /// Stack pointer (points to next free slot)
    sp: usize,

    /// Global variables
    globals: HashMap<String, Value>,

    /// Call frames for function calls
    frames: Vec<CallFrame>,

    /// Debug mode
    debug: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: vec![Value::Null; 256], // Pre-allocate stack
            sp: 0,
            globals: HashMap::new(),
            frames: Vec::new(),
            debug: false,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Execute a chunk of bytecode
    pub fn execute(&mut self, chunk: Chunk) -> Result<i32> {
        // Create initial frame
        let frame = CallFrame {
            chunk,
            ip: 0,
            stack_base: 0,
        };

        self.frames.push(frame);

        // Main execution loop
        loop {
            // Get instruction and advance IP
            let (instruction, stack_base) = {
                let frame = match self.frames.last_mut() {
                    Some(f) => f,
                    None => return Ok(0), // No frames left, exit
                };

                if frame.ip >= frame.chunk.code.len() {
                    return Err(anyhow!("Instruction pointer out of bounds"));
                }

                let instruction = frame.chunk.code[frame.ip].clone();
                frame.ip += 1;

                if self.debug {
                    println!("Stack: {:?}", &self.stack[0..self.sp]);
                    print!("Execute: ");
                    frame
                        .chunk
                        .disassemble_instruction(&instruction, frame.ip - 1);
                }

                (instruction, frame.stack_base)
            };

            match instruction {
                Instruction::LoadConst(idx) => {
                    let constant = {
                        let frame = self.frames.last().unwrap();
                        frame.chunk.constants[idx].clone()
                    };
                    let value = match constant {
                        Constant::Number(n) => Value::Number(n),
                        Constant::String(s) => Value::String(s),
                        Constant::Boolean(b) => Value::Boolean(b),
                        Constant::Null => Value::Null,
                    };
                    self.push(value);
                }

                Instruction::LoadVar(idx) => {
                    let value = self.stack[stack_base + idx].clone();
                    self.push(value);
                }

                Instruction::StoreVar(idx) => {
                    let value = self.pop(); // Pop the value off the stack
                    self.stack[stack_base + idx] = value;
                }

                Instruction::LoadGlobal(name) => {
                    let value = self
                        .globals
                        .get(&name)
                        .ok_or_else(|| anyhow!("Undefined variable: {}", name))?
                        .clone();
                    self.push(value);
                }

                Instruction::StoreGlobal(name) => {
                    let value = self.peek(0).clone();
                    self.globals.insert(name, value);
                }

                Instruction::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Number(x + y));
                        }
                        (Value::String(x), Value::String(y)) => {
                            self.push(Value::String(format!("{}{}", x, y)));
                        }
                        _ => return Err(anyhow!("Cannot add {:?} and {:?}", a, b)),
                    }
                }

                Instruction::Subtract => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    self.push(Value::Number(a - b));
                }

                Instruction::Multiply => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    self.push(Value::Number(a * b));
                }

                Instruction::Divide => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    if b == 0.0 {
                        return Err(anyhow!("Division by zero"));
                    }
                    self.push(Value::Number(a / b));
                }

                Instruction::Modulo => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    if b == 0.0 {
                        return Err(anyhow!("Modulo by zero"));
                    }
                    self.push(Value::Number(a % b));
                }

                Instruction::Negate => {
                    let a = self.pop().as_number()?;
                    self.push(Value::Number(-a));
                }

                // Fast integer operations - no type checking
                Instruction::AddInt => {
                    if let (Value::Number(b), Value::Number(a)) = (self.pop(), self.pop()) {
                        self.push(Value::Number(a + b));
                    }
                }

                Instruction::SubInt => {
                    if let (Value::Number(b), Value::Number(a)) = (self.pop(), self.pop()) {
                        self.push(Value::Number(a - b));
                    }
                }

                Instruction::MulInt => {
                    if let (Value::Number(b), Value::Number(a)) = (self.pop(), self.pop()) {
                        self.push(Value::Number(a * b));
                    }
                }

                Instruction::LessInt => {
                    if let (Value::Number(b), Value::Number(a)) = (self.pop(), self.pop()) {
                        self.push(Value::Boolean(a < b));
                    }
                }

                Instruction::IncrementInt => {
                    if let Value::Number(a) = self.pop() {
                        self.push(Value::Number(a + 1.0));
                    }
                }

                Instruction::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(self.values_equal(&a, &b)));
                }

                Instruction::NotEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(!self.values_equal(&a, &b)));
                }

                Instruction::Greater => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    self.push(Value::Boolean(a > b));
                }

                Instruction::GreaterEqual => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    self.push(Value::Boolean(a >= b));
                }

                Instruction::Less => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    self.push(Value::Boolean(a < b));
                }

                Instruction::LessEqual => {
                    let b = self.pop().as_number()?;
                    let a = self.pop().as_number()?;
                    self.push(Value::Boolean(a <= b));
                }

                Instruction::And => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a.is_truthy() && b.is_truthy()));
                }

                Instruction::Or => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a.is_truthy() || b.is_truthy()));
                }

                Instruction::Not => {
                    let a = self.pop();
                    self.push(Value::Boolean(!a.is_truthy()));
                }

                Instruction::Jump(target) => {
                    let frame = self.frames.last_mut().unwrap();
                    frame.ip = target;
                }

                Instruction::JumpIfFalse(target) => {
                    let condition = self.pop();
                    if !condition.is_truthy() {
                        let frame = self.frames.last_mut().unwrap();
                        frame.ip = target;
                    }
                }

                Instruction::JumpIfTrue(target) => {
                    let condition = self.pop();
                    if condition.is_truthy() {
                        let frame = self.frames.last_mut().unwrap();
                        frame.ip = target;
                    }
                }

                Instruction::Call(name, arity) => {
                    // Get the function chunk
                    let func_chunk = {
                        let current_frame = self.frames.last().unwrap();
                        current_frame
                            .chunk
                            .functions
                            .get(&name)
                            .ok_or_else(|| anyhow!("Undefined function: {}", name))?
                            .clone()
                    };

                    // Create new call frame
                    let new_frame = CallFrame {
                        chunk: func_chunk,
                        ip: 0,
                        stack_base: self.sp - arity,
                    };

                    self.frames.push(new_frame);
                }

                Instruction::Return => {
                    let return_value = self.pop();

                    // Pop the frame
                    let old_frame = self.frames.pop().unwrap();

                    // Restore stack pointer to before arguments
                    self.sp = old_frame.stack_base;

                    // Push return value
                    self.push(return_value);

                    // If no more frames, we're done
                    if self.frames.is_empty() {
                        let exit_code = match self.pop() {
                            Value::Number(n) => n as i32,
                            _ => 0,
                        };
                        return Ok(exit_code);
                    }
                }

                Instruction::ReturnNull => {
                    // Pop the frame
                    let old_frame = self.frames.pop().unwrap();

                    // Restore stack pointer
                    self.sp = old_frame.stack_base;

                    // Push null
                    self.push(Value::Null);

                    // If no more frames, we're done
                    if self.frames.is_empty() {
                        return Ok(0);
                    }
                }

                Instruction::Pop => {
                    self.pop();
                }

                Instruction::Dup => {
                    let value = self.peek(0).clone();
                    self.push(value);
                }

                Instruction::MakeArray(size) => {
                    let mut elements = Vec::with_capacity(size);
                    for _ in 0..size {
                        elements.push(self.pop());
                    }
                    elements.reverse(); // We popped in reverse order
                    self.push(Value::Array(elements));
                }

                Instruction::GetIndex => {
                    let index = self.pop().as_number()? as usize;
                    let array = self.pop().as_array()?;

                    if index >= array.len() {
                        return Err(anyhow!("Array index out of bounds: {}", index));
                    }

                    self.push(array[index].clone());
                }

                Instruction::SetIndex => {
                    let value = self.pop();
                    let index = self.pop().as_number()? as usize;
                    let mut array = self.pop().as_array()?;

                    if index >= array.len() {
                        return Err(anyhow!("Array index out of bounds: {}", index));
                    }

                    array[index] = value;
                    self.push(Value::Array(array));
                }

                Instruction::Length => {
                    let value = self.pop();
                    let len = match value {
                        Value::String(s) => s.len(),
                        Value::Array(a) => a.len(),
                        _ => {
                            return Err(anyhow!("Length can only be applied to strings or arrays"))
                        }
                    };
                    self.push(Value::Number(len as f64));
                }

                Instruction::Uppercase => {
                    let s = self.pop().as_string()?;
                    self.push(Value::String(s.to_uppercase()));
                }

                Instruction::Substring => {
                    let to = self.pop().as_number()? as usize;
                    let from = self.pop().as_number()? as usize;
                    let s = self.pop().as_string()?;

                    let chars: Vec<char> = s.chars().collect();
                    if from > to || to > chars.len() {
                        return Err(anyhow!("Substring indices out of bounds"));
                    }

                    let result: String = chars[from..to].iter().collect();
                    self.push(Value::String(result));
                }

                Instruction::Print => {
                    let value = self.pop();
                    println!("{}", value);
                }

                Instruction::Input(prompt) => {
                    if let Some(p) = prompt {
                        print!("{}", p);
                        io::stdout().flush()?;
                    }

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim().to_string();

                    // Try to parse as number, otherwise keep as string
                    if let Ok(n) = input.parse::<f64>() {
                        self.push(Value::Number(n));
                    } else {
                        self.push(Value::String(input));
                    }
                }

                Instruction::Halt => {
                    return Ok(0);
                }

                Instruction::Nop => {
                    // Do nothing
                }
            }
        }
    }

    // Stack operations
    fn push(&mut self, value: Value) {
        if self.sp >= self.stack.len() {
            self.stack.resize(self.stack.len() * 2, Value::Null);
        }
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> Value {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp -= 1;
        self.stack[self.sp].clone()
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.sp - 1 - distance]
    }

    #[allow(clippy::only_used_in_recursion)] // False positive - self is needed for method context
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => (x - y).abs() < f64::EPSILON,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Null, Value::Null) => true,
            (Value::Array(x), Value::Array(y)) => {
                if x.len() != y.len() {
                    return false;
                }
                for (a, b) in x.iter().zip(y.iter()) {
                    if !self.values_equal(a, b) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}
