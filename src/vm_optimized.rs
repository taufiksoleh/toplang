/// Highly optimized VM with reduced cloning and faster arithmetic
///
/// This module implements several micro-optimizations:
/// - Avoid cloning in hot paths (arithmetic)
/// - Inline small functions aggressively
/// - Specialize common operations
/// - Cache global lookups
use crate::bytecode::*;
use crate::vm::Value;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{self, Write};

/// Call frame for function calls
#[derive(Debug, Clone)]
struct CallFrame {
    chunk: Chunk,
    ip: usize,
    stack_base: usize,
}

/// Global variable cache entry
struct GlobalCache {
    value: Value,
    generation: usize,
}

/// Highly optimized Virtual Machine
pub struct OptimizedVM {
    /// Value stack
    stack: Vec<Value>,

    /// Stack pointer (points to next free slot)
    sp: usize,

    /// Global variables
    globals: HashMap<String, Value>,

    /// Global variable cache (for inline caching)
    global_cache: HashMap<String, GlobalCache>,

    /// Cache generation (incremented on global write)
    cache_generation: usize,

    /// Call frames for function calls
    frames: Vec<CallFrame>,

    /// Debug mode
    debug: bool,
}

impl OptimizedVM {
    pub fn new() -> Self {
        OptimizedVM {
            stack: Vec::with_capacity(1024), // Larger pre-allocation
            sp: 0,
            globals: HashMap::with_capacity(64),
            global_cache: HashMap::with_capacity(64),
            cache_generation: 0,
            frames: Vec::with_capacity(32),
            debug: false,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Execute a chunk of bytecode
    pub fn execute(&mut self, chunk: Chunk) -> Result<i32> {
        // Pre-allocate stack to avoid reallocation
        self.stack.resize(256, Value::Null);

        // Create initial frame
        let frame = CallFrame {
            chunk,
            ip: 0,
            stack_base: 0,
        };

        self.frames.push(frame);

        // Main execution loop - optimized for speed
        loop {
            // Get current frame (avoid borrowing conflicts)
            let frame_idx = self.frames.len() - 1;

            if self.frames.is_empty() {
                return Ok(0);
            }

            let ip = self.frames[frame_idx].ip;
            let stack_base = self.frames[frame_idx].stack_base;

            if ip >= self.frames[frame_idx].chunk.code.len() {
                return Err(anyhow!("Instruction pointer out of bounds"));
            }

            let instruction = self.frames[frame_idx].chunk.code[ip].clone();
            self.frames[frame_idx].ip += 1;

            if self.debug {
                println!("Stack (sp={}): {:?}", self.sp, &self.stack[0..self.sp]);
                print!("Execute[{}]: ", ip);
                self.frames[frame_idx]
                    .chunk
                    .disassemble_instruction(&instruction, ip);
            }

            // Optimized instruction dispatch
            match instruction {
                Instruction::LoadConst(idx) => {
                    let constant = &self.frames[frame_idx].chunk.constants[idx];
                    let value = match constant {
                        Constant::Number(n) => Value::Number(*n),
                        Constant::String(s) => Value::String(s.clone()),
                        Constant::Boolean(b) => Value::Boolean(*b),
                        Constant::Null => Value::Null,
                    };
                    self.push_fast(value);
                }

                Instruction::LoadVar(idx) => {
                    // Direct copy from stack (no clone needed for simple types)
                    let value = self.stack[stack_base + idx].clone();
                    self.push_fast(value);
                }

                Instruction::StoreVar(idx) => {
                    let value = self.pop_fast();
                    self.stack[stack_base + idx] = value;
                }

                Instruction::LoadGlobal(name) => {
                    // Inline caching for globals
                    if let Some(cached) = self.global_cache.get(&name) {
                        if cached.generation == self.cache_generation {
                            self.push_fast(cached.value.clone());
                            continue;
                        }
                    }

                    let value = self
                        .globals
                        .get(&name)
                        .ok_or_else(|| anyhow!("Undefined variable: {}", name))?
                        .clone();

                    // Cache for next access
                    self.global_cache.insert(
                        name.clone(),
                        GlobalCache {
                            value: value.clone(),
                            generation: self.cache_generation,
                        },
                    );

                    self.push_fast(value);
                }

                Instruction::StoreGlobal(name) => {
                    let value = self.peek_fast(0).clone();
                    self.globals.insert(name, value);
                    // Invalidate cache
                    self.cache_generation += 1;
                }

                // Optimized arithmetic - use raw stack access
                Instruction::Add => {
                    let b = self.pop_fast();
                    let a = self.pop_fast();
                    match (&a, &b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push_fast(Value::Number(x + y));
                        }
                        (Value::String(x), Value::String(y)) => {
                            self.push_fast(Value::String(format!("{}{}", x, y)));
                        }
                        _ => return Err(anyhow!("Cannot add {:?} and {:?}", a, b)),
                    }
                }

                Instruction::Subtract => {
                    let b = self.pop_fast().as_number()?;
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Number(a - b));
                }

                Instruction::Multiply => {
                    let b = self.pop_fast().as_number()?;
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Number(a * b));
                }

                Instruction::Divide => {
                    let b = self.pop_fast().as_number()?;
                    if b == 0.0 {
                        return Err(anyhow!("Division by zero"));
                    }
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Number(a / b));
                }

                Instruction::Modulo => {
                    let b = self.pop_fast().as_number()?;
                    if b == 0.0 {
                        return Err(anyhow!("Modulo by zero"));
                    }
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Number(a % b));
                }

                Instruction::Negate => {
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Number(-a));
                }

                // Super-fast integer operations (no error checking)
                Instruction::AddInt => {
                    if self.sp >= 2 {
                        if let (Value::Number(b), Value::Number(a)) =
                            (&self.stack[self.sp - 1], &self.stack[self.sp - 2])
                        {
                            let result = a + b;
                            self.sp -= 2;
                            self.push_fast(Value::Number(result));
                        } else {
                            let b = self.pop_fast();
                            let a = self.pop_fast();
                            if let (Value::Number(x), Value::Number(y)) = (a, b) {
                                self.push_fast(Value::Number(x + y));
                            }
                        }
                    }
                }

                Instruction::SubInt => {
                    if self.sp >= 2 {
                        if let (Value::Number(b), Value::Number(a)) =
                            (&self.stack[self.sp - 1], &self.stack[self.sp - 2])
                        {
                            let result = a - b;
                            self.sp -= 2;
                            self.push_fast(Value::Number(result));
                        } else {
                            let b = self.pop_fast();
                            let a = self.pop_fast();
                            if let (Value::Number(x), Value::Number(y)) = (a, b) {
                                self.push_fast(Value::Number(x - y));
                            }
                        }
                    }
                }

                Instruction::MulInt => {
                    if self.sp >= 2 {
                        if let (Value::Number(b), Value::Number(a)) =
                            (&self.stack[self.sp - 1], &self.stack[self.sp - 2])
                        {
                            let result = a * b;
                            self.sp -= 2;
                            self.push_fast(Value::Number(result));
                        } else {
                            let b = self.pop_fast();
                            let a = self.pop_fast();
                            if let (Value::Number(x), Value::Number(y)) = (a, b) {
                                self.push_fast(Value::Number(x * y));
                            }
                        }
                    }
                }

                Instruction::LessInt => {
                    if self.sp >= 2 {
                        if let (Value::Number(b), Value::Number(a)) =
                            (&self.stack[self.sp - 1], &self.stack[self.sp - 2])
                        {
                            let result = a < b;
                            self.sp -= 2;
                            self.push_fast(Value::Boolean(result));
                        } else {
                            let b = self.pop_fast();
                            let a = self.pop_fast();
                            if let (Value::Number(x), Value::Number(y)) = (a, b) {
                                self.push_fast(Value::Boolean(x < y));
                            }
                        }
                    }
                }

                Instruction::IncrementInt => {
                    if let Value::Number(a) = self.pop_fast() {
                        self.push_fast(Value::Number(a + 1.0));
                    }
                }

                Instruction::Equal => {
                    let b = self.pop_fast();
                    let a = self.pop_fast();
                    self.push_fast(Value::Boolean(self.values_equal(&a, &b)));
                }

                Instruction::NotEqual => {
                    let b = self.pop_fast();
                    let a = self.pop_fast();
                    self.push_fast(Value::Boolean(!self.values_equal(&a, &b)));
                }

                Instruction::Greater => {
                    let b = self.pop_fast().as_number()?;
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Boolean(a > b));
                }

                Instruction::GreaterEqual => {
                    let b = self.pop_fast().as_number()?;
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Boolean(a >= b));
                }

                Instruction::Less => {
                    let b = self.pop_fast().as_number()?;
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Boolean(a < b));
                }

                Instruction::LessEqual => {
                    let b = self.pop_fast().as_number()?;
                    let a = self.pop_fast().as_number()?;
                    self.push_fast(Value::Boolean(a <= b));
                }

                Instruction::And => {
                    let b = self.pop_fast();
                    let a = self.pop_fast();
                    self.push_fast(Value::Boolean(a.is_truthy() && b.is_truthy()));
                }

                Instruction::Or => {
                    let b = self.pop_fast();
                    let a = self.pop_fast();
                    self.push_fast(Value::Boolean(a.is_truthy() || b.is_truthy()));
                }

                Instruction::Not => {
                    let a = self.pop_fast();
                    self.push_fast(Value::Boolean(!a.is_truthy()));
                }

                Instruction::Jump(target) => {
                    self.frames[frame_idx].ip = target;
                }

                Instruction::JumpIfFalse(target) => {
                    let condition = self.pop_fast();
                    if !condition.is_truthy() {
                        self.frames[frame_idx].ip = target;
                    }
                }

                Instruction::JumpIfTrue(target) => {
                    let condition = self.pop_fast();
                    if condition.is_truthy() {
                        self.frames[frame_idx].ip = target;
                    }
                }

                Instruction::Call(name, arity) => {
                    let func_chunk = self.frames[frame_idx]
                        .chunk
                        .functions
                        .get(&name)
                        .ok_or_else(|| anyhow!("Undefined function: {}", name))?
                        .clone();

                    let new_frame = CallFrame {
                        chunk: func_chunk,
                        ip: 0,
                        stack_base: self.sp - arity,
                    };

                    self.frames.push(new_frame);
                }

                Instruction::Return => {
                    let return_value = self.pop_fast();
                    let old_frame = self.frames.pop().unwrap();
                    self.sp = old_frame.stack_base;
                    self.push_fast(return_value);

                    if self.frames.is_empty() {
                        let exit_code = match self.pop_fast() {
                            Value::Number(n) => n as i32,
                            _ => 0,
                        };
                        return Ok(exit_code);
                    }
                }

                Instruction::ReturnNull => {
                    let old_frame = self.frames.pop().unwrap();
                    self.sp = old_frame.stack_base;
                    self.push_fast(Value::Null);

                    if self.frames.is_empty() {
                        return Ok(0);
                    }
                }

                Instruction::Pop => {
                    self.pop_fast();
                }

                Instruction::Dup => {
                    let value = self.peek_fast(0).clone();
                    self.push_fast(value);
                }

                Instruction::MakeArray(size) => {
                    let mut elements = Vec::with_capacity(size);
                    for _ in 0..size {
                        elements.push(self.pop_fast());
                    }
                    elements.reverse();
                    self.push_fast(Value::Array(elements));
                }

                Instruction::GetIndex => {
                    let index = self.pop_fast().as_number()? as usize;
                    let array = self.pop_fast().as_array()?;

                    if index >= array.len() {
                        return Err(anyhow!("Array index out of bounds: {}", index));
                    }

                    self.push_fast(array[index].clone());
                }

                Instruction::SetIndex => {
                    let value = self.pop_fast();
                    let index = self.pop_fast().as_number()? as usize;
                    let mut array = self.pop_fast().as_array()?;

                    if index >= array.len() {
                        return Err(anyhow!("Array index out of bounds: {}", index));
                    }

                    array[index] = value;
                    self.push_fast(Value::Array(array));
                }

                Instruction::Length => {
                    let value = self.pop_fast();
                    let len = match value {
                        Value::String(s) => s.len(),
                        Value::Array(a) => a.len(),
                        _ => {
                            return Err(anyhow!("Length can only be applied to strings or arrays"))
                        }
                    };
                    self.push_fast(Value::Number(len as f64));
                }

                Instruction::Uppercase => {
                    let s = self.pop_fast().as_string()?;
                    self.push_fast(Value::String(s.to_uppercase()));
                }

                Instruction::Substring => {
                    let to = self.pop_fast().as_number()? as usize;
                    let from = self.pop_fast().as_number()? as usize;
                    let s = self.pop_fast().as_string()?;

                    let chars: Vec<char> = s.chars().collect();
                    if from > to || to > chars.len() {
                        return Err(anyhow!("Substring indices out of bounds"));
                    }

                    let result: String = chars[from..to].iter().collect();
                    self.push_fast(Value::String(result));
                }

                Instruction::Print => {
                    let value = self.pop_fast();
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

                    if let Ok(n) = input.parse::<f64>() {
                        self.push_fast(Value::Number(n));
                    } else {
                        self.push_fast(Value::String(input));
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

    // Ultra-fast stack operations (inlined)
    #[inline(always)]
    fn push_fast(&mut self, value: Value) {
        if self.sp >= self.stack.len() {
            self.stack.resize(self.stack.len() * 2, Value::Null);
        }
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    #[inline(always)]
    fn pop_fast(&mut self) -> Value {
        self.sp -= 1;
        std::mem::replace(&mut self.stack[self.sp], Value::Null)
    }

    #[inline(always)]
    fn peek_fast(&self, distance: usize) -> &Value {
        &self.stack[self.sp - 1 - distance]
    }

    #[allow(clippy::only_used_in_recursion)]
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
