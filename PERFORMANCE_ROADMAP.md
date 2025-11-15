# TopLang Performance Optimization Roadmap

## Current Status
- VM vs Python: 2.0-2.5x slower
- Goal: Match or exceed Python performance

## Implemented Optimizations

### 1. Bytecode VM (vs Tree-Walking Interpreter)
- Status: Complete
- Speedup: 1.65x over interpreter
- Impact: Foundation for all further optimizations

### 2. Constant Folding Optimizer
- Status: Complete
- Impact: Reduces bytecode size, eliminates runtime computation
- Example: `5 + 3` → `8` at compile time

### 3. Specialized Integer Instructions
- Status: Complete (AddInt, SubInt, MulInt, LessInt)
- Impact: Small improvement, mainly cleaner bytecode
- Limitation: Still uses enum-based Value type

### 4. Peephole Bytecode Optimization
- Status: Complete
- Impact: Aggressively converts to specialized instructions
- Patterns optimized:
  - `LoadVar + LoadVar + Add` → `LoadVar + LoadVar + AddInt`
  - Dead code elimination
  - Redundant operation removal

---

## High-Impact Optimizations (To Implement)

### 5. NaN Boxing for Value Representation
- Priority: HIGH
- Estimated Speedup: 1.4-1.8x
- Complexity: Medium

Current Problem:
```rust
enum Value {
    Number(f64),    // 16 bytes (8 for tag + 8 for value)
    String(String), // 32+ bytes
    Boolean(bool),  // 16 bytes
    ...
}
```

Solution - NaN Boxing:
Pack all values into 64-bit:
```rust
// Single 64-bit value, no enum overhead
struct Value(u64);

// Encoding scheme:
// Numbers: Normal f64 (except NaN range)
// Null:    0x7FF8000000000000
// True:    0x7FF8000000000001
// False:   0x7FF8000000000002
// Pointer: 0x7FF8... + 48-bit pointer
```

Benefits:
- Faster stack operations (just copy u64)
- Better cache locality
- Less memory allocation
- Faster arithmetic (direct f64 operations)

### 6. Direct Threaded Dispatch
- Priority: HIGH
- Estimated Speedup: 1.3-1.5x
- Complexity: Medium (requires unsafe Rust)

Current Problem:
```rust
match instruction {
    Instruction::Add => { ... }     // Indirect jump via match table
    Instruction::Sub => { ... }     // Not cache-friendly
    ...
}
```

Solution - Computed Goto:
```rust
// Use function pointers and jump directly
static DISPATCH_TABLE: [fn(&mut VM); 256] = [...];
loop {
    let op = code[ip];
    DISPATCH_TABLE[op](self);  // Direct jump, better branch prediction
}
```

Benefits:
- Eliminates match overhead
- Better CPU branch prediction
- How CPython and Lua achieve speed

---

### 7. Register-Based VM (vs Stack-Based)
- Priority: MEDIUM-HIGH
- Estimated Speedup: 1.5-2.0x
- Complexity: High (requires VM rewrite)

Current Problem:
Every operation requires stack manipulation:
```
LoadVar 0     ; push local 0
LoadVar 1     ; push local 1
Add           ; pop 2, push result
StoreVar 2    ; pop and store
```

Solution - Registers:
```
AddInt r2, r0, r1   ; r2 = r0 + r1 (single instruction!)
```

Benefits:
- Less memory traffic
- Fewer instructions
- More efficient code
- How LuaJIT achieves speed

### 8. Inline Caching
- Priority: MEDIUM
- Estimated Speedup: 1.2-1.4x
- Complexity: Medium

Cache variable lookups:
```rust
// First lookup
LoadGlobal("count") → HashMap lookup (slow)
                   → Cache result

// Subsequent lookups
LoadGlobal("count") → Check cache (fast!)
```

Benefits:
- Eliminate HashMap lookups in hot loops
- Faster global variable access
- Faster function calls

### 9. JIT Compilation with Cranelift
- Priority: HIGH (biggest win)
- Estimated Speedup: 3-10x for compute-heavy code
- Complexity: High

Approach:
1. Start in interpreter mode
2. Profile hot functions/loops
3. Compile hot code to native machine code
4. Execute native code directly

Benefits:
- Native CPU speed
- Register allocation by compiler
- CPU-specific optimizations
- How PyPy achieves 5x+ speedups

## Quick Wins (Next Steps)

### Immediate Actions

1. Implement NaN Boxing (1-2 days)
   - Replace `Value` enum with packed 64-bit representation
   - Update VM to use packed values
   - Expected: 1.4-1.8x speedup

2. Add Direct Threaded Dispatch (1 day)
   - Use function pointer dispatch table
   - Requires unsafe Rust
   - Expected: 1.3-1.5x speedup

3. Inline Caching (1 day)
   - Cache HashMap lookups
   - Track type information
   - Expected: 1.2-1.4x speedup

Combined Quick Wins: 2.2-3.7x faster, potentially exceeding Python performance.

## Advanced Optimizations (Future)

### 10. Escape Analysis
- Allocate values on stack when possible
- Reduces GC pressure

### 11. Loop Unrolling
- Detect simple counted loops
- Unroll for better CPU pipelining

### 12. Type Speculation
- Assume types based on profiling
- Generate specialized code paths
- Deoptimize if assumption wrong

### 13. SIMD Operations
- Use CPU vector instructions for array operations
- Massive speedup for numerical code

## Implementation Priority

Phase 1: Quick Wins (Current)
1. Completed: Specialized integer instructions
2. Completed: Peephole optimization
3. Next: NaN boxing
4. Next: Direct threaded dispatch

Phase 2: Major Performance
1. JIT compilation (Cranelift)
2. Inline caching
3. Register-based VM

Phase 3: Advanced
1. Type speculation
2. SIMD
3. Escape analysis

## Benchmarking Strategy

After each optimization:
1. Run full benchmark suite
2. Compare with Python
3. Profile to find next bottleneck
4. Iterate

Target: Exceed Python performance by Phase 2 completion.

## References

- NaN Boxing: [IEEE 754 NaN tagging](https://anniecherkaev.com/the-secret-life-of-nan)
- Direct Threading: [CPython VM internals](https://github.com/python/cpython/blob/main/Python/ceval.c)
- JIT: [Cranelift code generator](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)
- Register VM: [Lua 5.0 VM design](https://www.lua.org/doc/jucs05.pdf)
