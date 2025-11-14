/// Peephole optimizer for bytecode
///
/// This module performs post-compilation bytecode optimization by examining
/// small windows of instructions and replacing them with more efficient sequences.
use crate::bytecode::*;

/// Optimize a chunk of bytecode with peephole optimizations
pub fn optimize_chunk(chunk: &mut Chunk) {
    optimize_instructions(&mut chunk.code);

    // Optimize all function chunks recursively
    for (_name, func_chunk) in chunk.functions.iter_mut() {
        optimize_instructions(&mut func_chunk.code);
    }
}

/// Perform peephole optimizations on instruction sequence
fn optimize_instructions(code: &mut Vec<Instruction>) {
    let mut i = 0;

    while i < code.len() {
        // Arithmetic operations - use specialized integer instructions
        // These patterns work for any Load* + Load* + arithmetic op

        if i + 2 < code.len() {
            let is_load_1 = matches!(
                &code[i],
                Instruction::LoadConst(_) | Instruction::LoadVar(_) | Instruction::LoadGlobal(_)
            );
            let is_load_2 = matches!(
                &code[i + 1],
                Instruction::LoadConst(_) | Instruction::LoadVar(_) | Instruction::LoadGlobal(_)
            );

            if is_load_1 && is_load_2 {
                // Replace general arithmetic with fast integer arithmetic
                match &code[i + 2] {
                    Instruction::Add => {
                        code[i + 2] = Instruction::AddInt;
                    }
                    Instruction::Subtract => {
                        code[i + 2] = Instruction::SubInt;
                    }
                    Instruction::Multiply => {
                        code[i + 2] = Instruction::MulInt;
                    }
                    Instruction::Less => {
                        code[i + 2] = Instruction::LessInt;
                    }
                    _ => {}
                }
            }
        }

        // Pattern 7: Remove dead code after Return/Halt
        if matches!(
            code[i],
            Instruction::Return | Instruction::ReturnNull | Instruction::Halt
        ) {
            // Check if there are unreachable instructions after this
            // (except for jump targets - which we can't easily detect here)
            // This is conservative - we skip this optimization for now
        }

        // Pattern 8: Jump to next instruction → Remove
        if let Instruction::Jump(target) = &code[i] {
            if *target == i + 1 {
                code[i] = Instruction::Nop; // Replace with no-op
            }
        }

        // Pattern 9: LoadConst followed immediately by Pop → Remove both
        if i + 1 < code.len() {
            if let (Instruction::LoadConst(_), Instruction::Pop) = (&code[i], &code[i + 1]) {
                code[i] = Instruction::Nop;
                code[i + 1] = Instruction::Nop;
            }
        }

        // Pattern 10: Double negation → Remove both
        if i + 1 < code.len() {
            if let (Instruction::Not, Instruction::Not) = (&code[i], &code[i + 1]) {
                code[i] = Instruction::Nop;
                code[i + 1] = Instruction::Nop;
            }
        }

        i += 1;
    }

    // Remove Nop instructions (compact the code)
    code.retain(|inst| !matches!(inst, Instruction::Nop));
}

/// Optimize arithmetic operations in loops
/// This looks for common loop patterns and optimizes them
#[allow(dead_code)]
pub fn optimize_loops(chunk: &mut Chunk) {
    let code = &mut chunk.code;
    let mut i = 0;

    while i < code.len() {
        // Pattern: Loop counter increment
        // LoadVar(x) + LoadConst(1) + Add + StoreVar(x) => LoadVar(x) + IncrementInt + StoreVar(x)
        if i + 3 < code.len() {
            if let (
                Instruction::LoadVar(var1),
                Instruction::LoadConst(_const_idx),
                Instruction::Add,
                Instruction::StoreVar(var2),
            ) = (&code[i], &code[i + 1], &code[i + 2], &code[i + 3])
            {
                if var1 == var2 {
                    // Check if constant is 1
                    // For now, just use AddInt as it's faster
                    code[i + 2] = Instruction::AddInt;
                }
            }
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_optimization() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Constant::Number(5.0));
        chunk.add_constant(Constant::Number(3.0));

        chunk.emit(Instruction::LoadConst(0), 1);
        chunk.emit(Instruction::LoadConst(1), 1);
        chunk.emit(Instruction::Add, 1);

        optimize_chunk(&mut chunk);

        // Should have replaced Add with AddInt
        assert_eq!(chunk.code[2], Instruction::AddInt);
    }

    #[test]
    fn test_dead_const_pop() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Constant::Number(42.0));

        chunk.emit(Instruction::LoadConst(0), 1);
        chunk.emit(Instruction::Pop, 1);
        chunk.emit(Instruction::Halt, 1);

        optimize_chunk(&mut chunk);

        // LoadConst and Pop should be removed
        assert_eq!(chunk.code.len(), 1);
        assert_eq!(chunk.code[0], Instruction::Halt);
    }
}
