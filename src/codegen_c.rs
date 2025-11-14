/// C code generator for TopLang
///
/// Transpiles bytecode to C for native compilation
use crate::bytecode::*;
use anyhow::Result;
use std::fmt::Write as FmtWrite;

pub struct CCodeGen {
    output: String,
}

impl CCodeGen {
    pub fn new() -> Self {
        CCodeGen {
            output: String::new(),
        }
    }

    pub fn compile_chunk(&mut self, chunk: &Chunk) -> Result<String> {
        // Generate C header
        writeln!(&mut self.output, "// Generated C code from TopLang").unwrap();
        writeln!(&mut self.output, "#include <stdio.h>").unwrap();
        writeln!(&mut self.output, "#include <stdlib.h>").unwrap();
        writeln!(&mut self.output, "#include <string.h>").unwrap();
        writeln!(&mut self.output, "#include <stdint.h>").unwrap();
        writeln!(&mut self.output).unwrap();

        // Value type
        writeln!(&mut self.output, "typedef uint64_t Value;").unwrap();
        writeln!(&mut self.output, "typedef struct {{ const char* data; }} String;").unwrap();
        writeln!(&mut self.output).unwrap();

        // NaN boxing constants
        writeln!(&mut self.output, "#define TAG_NULL 0x7FF8000000000001ULL").unwrap();
        writeln!(&mut self.output, "#define TAG_TRUE 0x7FF8000000000002ULL").unwrap();
        writeln!(&mut self.output, "#define TAG_FALSE 0x7FF8000000000003ULL").unwrap();
        writeln!(&mut self.output, "#define TAG_PTR 0x7FF8000000000000ULL").unwrap();
        writeln!(&mut self.output, "#define PTR_MASK 0x0000FFFFFFFFFFFFULL").unwrap();
        writeln!(&mut self.output).unwrap();

        self.generate_helpers()?;

        // Forward declare functions
        for name in chunk.functions.keys() {
            writeln!(&mut self.output, "Value func_{}(void);", name.replace("-", "_")).unwrap();
        }
        writeln!(&mut self.output).unwrap();

        // Compile functions
        for (name, func_chunk) in &chunk.functions {
            self.compile_function(name, func_chunk)?;
        }

        // Main
        writeln!(&mut self.output, "int main(void) {{").unwrap();
        writeln!(&mut self.output, "    func_main();").unwrap();
        writeln!(&mut self.output, "    return 0;").unwrap();
        writeln!(&mut self.output, "}}").unwrap();

        Ok(self.output.clone())
    }

    fn generate_helpers(&mut self) -> Result<()> {
        writeln!(&mut self.output, "static inline Value make_number(double n) {{").unwrap();
        writeln!(&mut self.output, "    union {{ double d; uint64_t u; }} v = {{ .d = n }};").unwrap();
        writeln!(&mut self.output, "    return v.u;").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        writeln!(&mut self.output, "static inline Value make_string(const char* s) {{").unwrap();
        writeln!(&mut self.output, "    String* str = malloc(sizeof(String));").unwrap();
        writeln!(&mut self.output, "    str->data = s;").unwrap();
        writeln!(&mut self.output, "    return TAG_PTR | ((uint64_t)str & PTR_MASK);").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        writeln!(&mut self.output, "static inline double as_number(Value v) {{").unwrap();
        writeln!(&mut self.output, "    union {{ uint64_t u; double d; }} r = {{ .u = v }};").unwrap();
        writeln!(&mut self.output, "    return r.d;").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        writeln!(&mut self.output, "static inline String* as_string(Value v) {{").unwrap();
        writeln!(&mut self.output, "    return (String*)(v & PTR_MASK);").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        writeln!(&mut self.output, "static inline int is_number(Value v) {{").unwrap();
        writeln!(&mut self.output, "    return (v & 0x7FF8000000000000ULL) != 0x7FF8000000000000ULL;").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        writeln!(&mut self.output, "static inline int is_string(Value v) {{").unwrap();
        writeln!(&mut self.output, "    return (v & TAG_PTR) == TAG_PTR && v != TAG_NULL && v != TAG_TRUE && v != TAG_FALSE;").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        // Print function
        writeln!(&mut self.output, "void value_print(Value v) {{").unwrap();
        writeln!(&mut self.output, "    if (is_number(v)) {{").unwrap();
        writeln!(&mut self.output, "        double n = as_number(v);").unwrap();
        writeln!(&mut self.output, "        if (n == (long long)n) {{").unwrap();
        writeln!(&mut self.output, "            printf(\"%lld\\n\", (long long)n);").unwrap();
        writeln!(&mut self.output, "        }} else {{").unwrap();
        writeln!(&mut self.output, "            printf(\"%g\\n\", n);").unwrap();
        writeln!(&mut self.output, "        }}").unwrap();
        writeln!(&mut self.output, "    }} else if (is_string(v)) {{").unwrap();
        writeln!(&mut self.output, "        printf(\"%s\\n\", as_string(v)->data);").unwrap();
        writeln!(&mut self.output, "    }} else if (v == TAG_TRUE) {{").unwrap();
        writeln!(&mut self.output, "        printf(\"true\\n\");").unwrap();
        writeln!(&mut self.output, "    }} else if (v == TAG_FALSE) {{").unwrap();
        writeln!(&mut self.output, "        printf(\"false\\n\");").unwrap();
        writeln!(&mut self.output, "    }} else if (v == TAG_NULL) {{").unwrap();
        writeln!(&mut self.output, "        printf(\"null\\n\");").unwrap();
        writeln!(&mut self.output, "    }}").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        Ok(())
    }

    fn compile_function(&mut self, name: &str, chunk: &Chunk) -> Result<()> {
        let safe_name = name.replace("-", "_");
        writeln!(&mut self.output, "Value func_{}(void) {{", safe_name).unwrap();

        // Stack and locals
        writeln!(&mut self.output, "    Value stack[256];").unwrap();
        writeln!(&mut self.output, "    int sp = 0;").unwrap();
        writeln!(&mut self.output, "    Value locals[64] = {{0}};").unwrap();
        writeln!(&mut self.output).unwrap();

        // Find jump targets
        let mut jump_targets = std::collections::HashSet::new();
        for (_ip, instr) in chunk.code.iter().enumerate() {
            match instr {
                Instruction::Jump(target)
                | Instruction::JumpIfFalse(target)
                | Instruction::JumpIfTrue(target) => {
                    jump_targets.insert(*target);
                }
                _ => {}
            }
        }

        // Generate instructions
        for (ip, instr) in chunk.code.iter().enumerate() {
            if jump_targets.contains(&ip) {
                writeln!(&mut self.output, "  L{}:", ip).unwrap();
            }

            match instr {
                Instruction::LoadConst(idx) => {
                    match &chunk.constants[*idx] {
                        Constant::Number(n) => {
                            writeln!(&mut self.output, "    stack[sp++] = make_number({});", n).unwrap();
                        }
                        Constant::String(s) => {
                            let escaped = s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n");
                            writeln!(&mut self.output, "    stack[sp++] = make_string(\"{}\");", escaped).unwrap();
                        }
                        Constant::Boolean(true) => {
                            writeln!(&mut self.output, "    stack[sp++] = TAG_TRUE;").unwrap();
                        }
                        Constant::Boolean(false) => {
                            writeln!(&mut self.output, "    stack[sp++] = TAG_FALSE;").unwrap();
                        }
                        Constant::Null => {
                            writeln!(&mut self.output, "    stack[sp++] = TAG_NULL;").unwrap();
                        }
                    }
                }

                Instruction::LoadVar(idx) => {
                    writeln!(&mut self.output, "    stack[sp++] = locals[{}];", idx).unwrap();
                }

                Instruction::StoreVar(idx) => {
                    writeln!(&mut self.output, "    locals[{}] = stack[--sp];", idx).unwrap();
                }

                Instruction::Add | Instruction::AddInt => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value b = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        stack[sp++] = make_number(as_number(a) + as_number(b));").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Subtract | Instruction::SubInt => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value b = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        stack[sp++] = make_number(as_number(a) - as_number(b));").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Multiply | Instruction::MulInt => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value b = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        stack[sp++] = make_number(as_number(a) * as_number(b));").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Divide => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value b = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        stack[sp++] = make_number(as_number(a) / as_number(b));").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Equal => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value b = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        if (is_number(a) && is_number(b)) {{").unwrap();
                    writeln!(&mut self.output, "            stack[sp++] = (as_number(a) == as_number(b)) ? TAG_TRUE : TAG_FALSE;").unwrap();
                    writeln!(&mut self.output, "        }} else {{").unwrap();
                    writeln!(&mut self.output, "            stack[sp++] = (a == b) ? TAG_TRUE : TAG_FALSE;").unwrap();
                    writeln!(&mut self.output, "        }}").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Less | Instruction::LessInt => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value b = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        stack[sp++] = (as_number(a) < as_number(b)) ? TAG_TRUE : TAG_FALSE;").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Greater => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value b = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        stack[sp++] = (as_number(a) > as_number(b)) ? TAG_TRUE : TAG_FALSE;").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Not => {
                    writeln!(&mut self.output, "    {{").unwrap();
                    writeln!(&mut self.output, "        Value a = stack[--sp];").unwrap();
                    writeln!(&mut self.output, "        int is_truthy = (a != TAG_FALSE && a != TAG_NULL);").unwrap();
                    writeln!(&mut self.output, "        stack[sp++] = is_truthy ? TAG_FALSE : TAG_TRUE;").unwrap();
                    writeln!(&mut self.output, "    }}").unwrap();
                }

                Instruction::Print => {
                    writeln!(&mut self.output, "    value_print(stack[--sp]);").unwrap();
                }

                Instruction::Pop => {
                    writeln!(&mut self.output, "    sp--;").unwrap();
                }

                Instruction::Jump(target) => {
                    writeln!(&mut self.output, "    goto L{};", target).unwrap();
                }

                Instruction::JumpIfFalse(target) => {
                    writeln!(&mut self.output, "    if (stack[--sp] == TAG_FALSE || stack[sp] == TAG_NULL) goto L{};", target).unwrap();
                }

                Instruction::JumpIfTrue(target) => {
                    writeln!(&mut self.output, "    if (stack[--sp] == TAG_TRUE) goto L{};", target).unwrap();
                }

                Instruction::Return => {
                    writeln!(&mut self.output, "    return stack[--sp];").unwrap();
                }

                Instruction::ReturnNull => {
                    writeln!(&mut self.output, "    return TAG_NULL;").unwrap();
                }

                Instruction::IncrementInt => {
                    writeln!(&mut self.output, "    stack[sp-1] = make_number(as_number(stack[sp-1]) + 1.0);").unwrap();
                }

                _ => {}
            }
        }

        writeln!(&mut self.output, "    return TAG_NULL;").unwrap();
        writeln!(&mut self.output, "}}").unwrap();
        writeln!(&mut self.output).unwrap();

        Ok(())
    }
}
