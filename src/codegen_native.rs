/// Native code generator for TopLang
///
/// Compiles bytecode to native machine code using Cranelift
/// Supports AOT compilation to standalone executables

use crate::bytecode::*;
use anyhow::{anyhow, Result};
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::collections::HashMap;
use target_lexicon::Triple;

/// Native code generator
pub struct NativeCodeGen {
    module: ObjectModule,
    ctx: codegen::Context,
    builder_context: FunctionBuilderContext,
    /// Maps function names to their compiled function IDs
    functions: HashMap<String, cranelift_module::FuncId>,
    /// Runtime function declarations
    runtime_funcs: RuntimeFunctions,
}

/// Runtime function signatures
struct RuntimeFunctions {
    print: cranelift_module::FuncId,
    input: cranelift_module::FuncId,
    add: cranelift_module::FuncId,
    subtract: cranelift_module::FuncId,
    multiply: cranelift_module::FuncId,
    divide: cranelift_module::FuncId,
    equal: cranelift_module::FuncId,
    less: cranelift_module::FuncId,
    greater: cranelift_module::FuncId,
    not: cranelift_module::FuncId,
    string_new: cranelift_module::FuncId,
    array_new: cranelift_module::FuncId,
    array_get: cranelift_module::FuncId,
    array_set: cranelift_module::FuncId,
    array_length: cranelift_module::FuncId,
}

impl NativeCodeGen {
    pub fn new() -> Result<Self> {
        // Create module for target platform
        let target_triple = Triple::host();
        let isa_builder = cranelift_native::builder()
            .map_err(|e| anyhow!("Failed to create ISA builder: {:?}", e))?;
        let isa = isa_builder
            .finish(settings::Flags::new(settings::builder()))
            .map_err(|e| anyhow!("Failed to create ISA: {:?}", e))?;

        let builder = ObjectBuilder::new(
            isa,
            "toplang_program".to_string(),
            cranelift_module::default_libcall_names(),
        )
        .map_err(|e| anyhow!("Failed to create object builder: {:?}", e))?;
        let module = ObjectModule::new(builder);

        let mut ctx = codegen::Context::new();
        ctx.func.signature.call_conv = module.target_config().default_call_conv;

        let builder_context = FunctionBuilderContext::new();

        let mut codegen = NativeCodeGen {
            module,
            ctx,
            builder_context,
            functions: HashMap::new(),
            runtime_funcs: RuntimeFunctions {
                print: cranelift_module::FuncId::from_u32(0),
                input: cranelift_module::FuncId::from_u32(0),
                add: cranelift_module::FuncId::from_u32(0),
                subtract: cranelift_module::FuncId::from_u32(0),
                multiply: cranelift_module::FuncId::from_u32(0),
                divide: cranelift_module::FuncId::from_u32(0),
                equal: cranelift_module::FuncId::from_u32(0),
                less: cranelift_module::FuncId::from_u32(0),
                greater: cranelift_module::FuncId::from_u32(0),
                not: cranelift_module::FuncId::from_u32(0),
                string_new: cranelift_module::FuncId::from_u32(0),
                array_new: cranelift_module::FuncId::from_u32(0),
                array_get: cranelift_module::FuncId::from_u32(0),
                array_set: cranelift_module::FuncId::from_u32(0),
                array_length: cranelift_module::FuncId::from_u32(0),
            },
        };

        // Declare runtime functions
        codegen.runtime_funcs = codegen.declare_runtime_functions()?;

        Ok(codegen)
    }

    /// Declare all runtime functions
    fn declare_runtime_functions(&mut self) -> Result<RuntimeFunctions> {
        let ptr_type = self.module.target_config().pointer_type();

        // Value type is 64-bit (NaN-boxed)
        let val_type = types::I64;

        // print(Value) -> void
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        let print = self
            .module
            .declare_function("toplang_print", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare print: {:?}", e))?;

        // input(Value) -> Value
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        sig.returns.push(AbiParam::new(val_type));
        let input = self
            .module
            .declare_function("toplang_input", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare input: {:?}", e))?;

        // add(Value, Value) -> Value
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        sig.params.push(AbiParam::new(val_type));
        sig.returns.push(AbiParam::new(val_type));
        let add = self
            .module
            .declare_function("toplang_add", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare add: {:?}", e))?;

        // Similar for other arithmetic operations
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        sig.params.push(AbiParam::new(val_type));
        sig.returns.push(AbiParam::new(val_type));
        let subtract = self
            .module
            .declare_function("toplang_subtract", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare subtract: {:?}", e))?;

        let multiply = self
            .module
            .declare_function("toplang_multiply", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare multiply: {:?}", e))?;

        let divide = self
            .module
            .declare_function("toplang_divide", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare divide: {:?}", e))?;

        // Comparison operations
        let equal = self
            .module
            .declare_function("toplang_equal", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare equal: {:?}", e))?;

        let less = self
            .module
            .declare_function("toplang_less", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare less: {:?}", e))?;

        let greater = self
            .module
            .declare_function("toplang_greater", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare greater: {:?}", e))?;

        // Logical NOT
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        sig.returns.push(AbiParam::new(val_type));
        let not = self
            .module
            .declare_function("toplang_not", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare not: {:?}", e))?;

        // string_new(ptr, len) -> Value
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(ptr_type));
        sig.params.push(AbiParam::new(ptr_type));
        sig.returns.push(AbiParam::new(val_type));
        let string_new = self
            .module
            .declare_function("toplang_string_new", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare string_new: {:?}", e))?;

        // array_new(size) -> Value
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(types::I64));
        sig.returns.push(AbiParam::new(val_type));
        let array_new = self
            .module
            .declare_function("toplang_array_new", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare array_new: {:?}", e))?;

        // array_get(array, index) -> Value
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        sig.params.push(AbiParam::new(val_type));
        sig.returns.push(AbiParam::new(val_type));
        let array_get = self
            .module
            .declare_function("toplang_array_get", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare array_get: {:?}", e))?;

        // array_set(array, index, value) -> void
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        sig.params.push(AbiParam::new(val_type));
        sig.params.push(AbiParam::new(val_type));
        let array_set = self
            .module
            .declare_function("toplang_array_set", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare array_set: {:?}", e))?;

        // array_length(array) -> Value
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(val_type));
        sig.returns.push(AbiParam::new(val_type));
        let array_length = self
            .module
            .declare_function("toplang_array_length", Linkage::Import, &sig)
            .map_err(|e| anyhow!("Failed to declare array_length: {:?}", e))?;

        Ok(RuntimeFunctions {
            print,
            input,
            add,
            subtract,
            multiply,
            divide,
            equal,
            less,
            greater,
            not,
            string_new,
            array_new,
            array_get,
            array_set,
            array_length,
        })
    }

    /// Compile a chunk to native code
    pub fn compile_chunk(mut self, chunk: &Chunk) -> Result<Vec<u8>> {
        // First, compile all functions
        for (name, func_chunk) in &chunk.functions {
            self.compile_function(name, func_chunk)?;
        }

        // Compile the main entry point
        self.compile_main(chunk)?;

        // Finalize and get object code
        let product = self.module.finish();
        Ok(product.emit().map_err(|e| anyhow!("Failed to emit object code: {:?}", e))?)
    }

    /// Compile a function
    fn compile_function(&mut self, name: &str, chunk: &Chunk) -> Result<()> {
        // Create function signature (no params for now, returns Value)
        let val_type = types::I64;
        self.ctx.func.signature.returns.push(AbiParam::new(val_type));

        // Declare the function
        let func_id = self
            .module
            .declare_function(name, Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| anyhow!("Failed to declare function {}: {:?}", name, e))?;

        self.functions.insert(name.to_string(), func_id);

        // Build function body
        {
            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Compile bytecode instructions - inline the logic to avoid borrowing issues
            let result = Self::compile_instructions_static(
                &mut builder,
                chunk,
                &self.runtime_funcs,
                &self.module,
            )?;

            // Return the result
            builder.ins().return_(&[result]);

            // Finalize
            builder.finalize();
        }

        // Define the function
        self.module
            .define_function(func_id, &mut self.ctx)
            .map_err(|e| anyhow!("Failed to define function {}: {:?}", name, e))?;

        // Clear context for next function
        self.module.clear_context(&mut self.ctx);

        Ok(())
    }

    /// Compile main entry point
    fn compile_main(&mut self, _chunk: &Chunk) -> Result<()> {
        // Create main signature: () -> i32
        let mut sig = self.module.make_signature();
        sig.returns.push(AbiParam::new(types::I32));

        let main_id = self
            .module
            .declare_function("main", Linkage::Export, &sig)
            .map_err(|e| anyhow!("Failed to declare main: {:?}", e))?;

        self.ctx.func.signature = sig;

        {
            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let entry_block = builder.create_block();
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Call the user's main function if it exists
            if let Some(user_main_id) = self.functions.get("main") {
                let local_callee = self.module.declare_func_in_func(*user_main_id, builder.func);
                builder.ins().call(local_callee, &[]);
            }

            // Return 0
            let zero = builder.ins().iconst(types::I32, 0);
            builder.ins().return_(&[zero]);

            builder.finalize();
        }

        self.module
            .define_function(main_id, &mut self.ctx)
            .map_err(|e| anyhow!("Failed to define main: {:?}", e))?;

        self.module.clear_context(&mut self.ctx);

        Ok(())
    }

    /// Compile bytecode instructions to Cranelift IR
    fn compile_instructions_static(
        builder: &mut FunctionBuilder,
        chunk: &Chunk,
        runtime_funcs: &RuntimeFunctions,
        module: &ObjectModule,
    ) -> Result<cranelift::prelude::Value> {
        let val_type = types::I64;

        // Stack simulation (we'll use Cranelift values directly)
        let mut stack: Vec<cranelift::prelude::Value> = Vec::new();

        // Local variables (indexed)
        let mut locals: Vec<cranelift::prelude::Value> = Vec::new();

        // Constants pool
        let mut constants: Vec<cranelift::prelude::Value> = Vec::new();
        for constant in &chunk.constants {
            let val = match constant {
                Constant::Number(n) => {
                    // Encode as NaN-boxed value
                    let bits = n.to_bits();
                    builder.ins().iconst(val_type, bits as i64)
                }
                Constant::Boolean(true) => {
                    builder.ins().iconst(val_type, 0x7FF8_0000_0000_0002u64 as i64)
                }
                Constant::Boolean(false) => {
                    builder.ins().iconst(val_type, 0x7FF8_0000_0000_0003u64 as i64)
                }
                Constant::Null => {
                    builder.ins().iconst(val_type, 0x7FF8_0000_0000_0001u64 as i64)
                }
                Constant::String(_s) => {
                    // TODO: Create string constant in data section
                    builder.ins().iconst(val_type, 0x7FF8_0000_0000_0001u64 as i64)
                }
            };
            constants.push(val);
        }

        // Block map for jumps
        let mut blocks: HashMap<usize, Block> = HashMap::new();

        // First pass: create all blocks for jump targets
        for (_ip, instr) in chunk.code.iter().enumerate() {
            match instr {
                Instruction::Jump(target)
                | Instruction::JumpIfFalse(target)
                | Instruction::JumpIfTrue(target) => {
                    if !blocks.contains_key(target) {
                        blocks.insert(*target, builder.create_block());
                    }
                }
                _ => {}
            }
        }

        // Create a default fallback block
        let fallback_block = builder.create_block();
        builder.switch_to_block(fallback_block);

        // Second pass: compile instructions
        let mut ip = 0;

        while ip < chunk.code.len() {
            // Check if this instruction is a jump target
            if let Some(block) = blocks.get(&ip) {
                builder.ins().jump(*block, &[]);
                builder.switch_to_block(*block);
            }

            let instr = &chunk.code[ip];
            match instr {
                Instruction::LoadConst(idx) => {
                    stack.push(constants[*idx]);
                }

                Instruction::LoadVar(idx) => {
                    while locals.len() <= *idx {
                        let null = builder.ins().iconst(val_type, 0x7FF8_0000_0000_0001u64 as i64);
                        locals.push(null);
                    }
                    stack.push(locals[*idx]);
                }

                Instruction::StoreVar(idx) => {
                    let val = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    while locals.len() <= *idx {
                        let null = builder.ins().iconst(val_type, 0x7FF8_0000_0000_0001u64 as i64);
                        locals.push(null);
                    }
                    locals[*idx] = val;
                }

                Instruction::Add | Instruction::AddInt => {
                    let b = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let add_ref = module.declare_func_in_func(runtime_funcs.add, builder.func);
                    let call = builder.ins().call(add_ref, &[a, b]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Subtract | Instruction::SubInt => {
                    let b = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let sub_ref = module.declare_func_in_func(runtime_funcs.subtract, builder.func);
                    let call = builder.ins().call(sub_ref, &[a, b]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Multiply | Instruction::MulInt => {
                    let b = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let mul_ref = module.declare_func_in_func(runtime_funcs.multiply, builder.func);
                    let call = builder.ins().call(mul_ref, &[a, b]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Divide => {
                    let b = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let div_ref = module.declare_func_in_func(runtime_funcs.divide, builder.func);
                    let call = builder.ins().call(div_ref, &[a, b]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Equal => {
                    let b = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let eq_ref = module.declare_func_in_func(runtime_funcs.equal, builder.func);
                    let call = builder.ins().call(eq_ref, &[a, b]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Less | Instruction::LessInt => {
                    let b = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let less_ref = module.declare_func_in_func(runtime_funcs.less, builder.func);
                    let call = builder.ins().call(less_ref, &[a, b]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Greater => {
                    let b = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let gt_ref = module.declare_func_in_func(runtime_funcs.greater, builder.func);
                    let call = builder.ins().call(gt_ref, &[a, b]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Not => {
                    let a = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let not_ref = module.declare_func_in_func(runtime_funcs.not, builder.func);
                    let call = builder.ins().call(not_ref, &[a]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                Instruction::Print => {
                    let val = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;
                    let print_ref = module.declare_func_in_func(runtime_funcs.print, builder.func);
                    builder.ins().call(print_ref, &[val]);
                }

                Instruction::Pop => {
                    stack.pop();
                }

                Instruction::Return => {
                    let val = stack.pop().unwrap_or_else(|| {
                        builder.ins().iconst(val_type, 0x7FF8_0000_0000_0001u64 as i64)
                    });
                    return Ok(val);
                }

                Instruction::ReturnNull => {
                    let null = builder.ins().iconst(val_type, 0x7FF8_0000_0000_0001u64 as i64);
                    return Ok(null);
                }

                Instruction::JumpIfFalse(target) => {
                    let condition = stack.pop().ok_or_else(|| anyhow!("Stack underflow"))?;

                    // Check if value is truthy (not null, not false, not 0)
                    let tag_true = builder.ins().iconst(val_type, 0x7FF8_0000_0000_0002u64 as i64);
                    let is_true = builder.ins().icmp(IntCC::Equal, condition, tag_true);

                    let then_block = blocks[target];
                    let else_block = builder.create_block();

                    builder.ins().brif(is_true, else_block, &[], then_block, &[]);
                    builder.switch_to_block(else_block);
                    builder.seal_block(else_block);
                }

                Instruction::Jump(target) => {
                    let target_block = blocks[target];
                    builder.ins().jump(target_block, &[]);
                    // Create unreachable block for subsequent instructions
                    let unreachable = builder.create_block();
                    builder.switch_to_block(unreachable);
                }

                _ => {
                    // Other instructions not yet implemented
                    eprintln!("Warning: Instruction {:?} not yet implemented in native codegen", instr);
                }
            }

            ip += 1;
        }

        // If we get here without returning, return null
        let null = builder.ins().iconst(val_type, 0x7FF8_0000_0000_0001u64 as i64);
        Ok(null)
    }
}
