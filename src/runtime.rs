/// Runtime library for natively compiled TopLang programs
///
/// This module provides the runtime support functions that compiled
/// TopLang programs need (print, input, array operations, etc.)

use std::io::{self, Write};

/// Value type for runtime (NaN-boxed for performance)
/// Uses IEEE 754 NaN tagging to pack all types into 64 bits
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Value(u64);

// NaN boxing constants
const QNAN: u64 = 0x7FF8_0000_0000_0000;
const TAG_NULL: u64 = 0x7FF8_0000_0000_0001;
const TAG_TRUE: u64 = 0x7FF8_0000_0000_0002;
const TAG_FALSE: u64 = 0x7FF8_0000_0000_0003;
const TAG_PTR: u64 = 0x7FF8_0000_0000_0000;
const PTR_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

impl Value {
    /// Create a number value
    #[inline]
    pub fn number(n: f64) -> Self {
        Value(n.to_bits())
    }

    /// Create a null value
    #[inline]
    pub fn null() -> Self {
        Value(TAG_NULL)
    }

    /// Create a boolean value
    #[inline]
    pub fn boolean(b: bool) -> Self {
        if b {
            Value(TAG_TRUE)
        } else {
            Value(TAG_FALSE)
        }
    }

    /// Create a pointer value (for strings, arrays, etc.)
    #[inline]
    pub fn ptr(p: *mut u8) -> Self {
        Value(TAG_PTR | (p as u64 & PTR_MASK))
    }

    /// Check if value is a number
    #[inline]
    pub fn is_number(&self) -> bool {
        (self.0 & QNAN) != QNAN
    }

    /// Check if value is null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == TAG_NULL
    }

    /// Check if value is boolean
    #[inline]
    pub fn is_boolean(&self) -> bool {
        self.0 == TAG_TRUE || self.0 == TAG_FALSE
    }

    /// Check if value is a pointer
    #[inline]
    pub fn is_ptr(&self) -> bool {
        (self.0 & QNAN) == QNAN && self.0 != TAG_NULL && self.0 != TAG_TRUE && self.0 != TAG_FALSE
    }

    /// Get number value
    #[inline]
    pub fn as_number(&self) -> f64 {
        f64::from_bits(self.0)
    }

    /// Get boolean value
    #[inline]
    pub fn as_boolean(&self) -> bool {
        self.0 == TAG_TRUE
    }

    /// Get pointer value
    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        (self.0 & PTR_MASK) as *mut u8
    }

    /// Check if value is truthy (for conditionals)
    pub fn is_truthy(&self) -> bool {
        if self.is_null() {
            false
        } else if self.is_boolean() {
            self.as_boolean()
        } else if self.is_number() {
            self.as_number() != 0.0
        } else {
            true
        }
    }
}

// Runtime heap-allocated string type
#[repr(C)]
pub struct RuntimeString {
    data: *mut u8,
    len: usize,
    capacity: usize,
}

// Runtime functions exported for native code

/// Print a value to stdout
#[no_mangle]
pub extern "C" fn toplang_print(val: Value) {
    if val.is_number() {
        let n = val.as_number();
        if n.fract() == 0.0 {
            println!("{}", n as i64);
        } else {
            println!("{}", n);
        }
    } else if val.is_boolean() {
        println!("{}", val.as_boolean());
    } else if val.is_null() {
        println!("null");
    } else if val.is_ptr() {
        unsafe {
            let str_ptr = val.as_ptr() as *const RuntimeString;
            let s = &*str_ptr;
            let slice = std::slice::from_raw_parts(s.data, s.len);
            let string = std::str::from_utf8_unchecked(slice);
            println!("{}", string);
        }
    }
}

/// Read input from stdin
#[no_mangle]
pub extern "C" fn toplang_input(prompt: Value) -> Value {
    // Print prompt if provided
    if !prompt.is_null() && prompt.is_ptr() {
        unsafe {
            let str_ptr = prompt.as_ptr() as *const RuntimeString;
            let s = &*str_ptr;
            let slice = std::slice::from_raw_parts(s.data, s.len);
            let string = std::str::from_utf8_unchecked(slice);
            print!("{}", string);
            io::stdout().flush().unwrap();
        }
    }

    // Read line
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    // Remove trailing newline
    if buffer.ends_with('\n') {
        buffer.pop();
        if buffer.ends_with('\r') {
            buffer.pop();
        }
    }

    // Allocate runtime string
    let len = buffer.len();
    let capacity = buffer.capacity();
    let data = buffer.as_mut_ptr();
    std::mem::forget(buffer);

    let runtime_str = Box::new(RuntimeString {
        data,
        len,
        capacity,
    });

    Value::ptr(Box::into_raw(runtime_str) as *mut u8)
}

/// Create a runtime string from a Rust string (for constants)
#[no_mangle]
pub extern "C" fn toplang_string_new(data: *const u8, len: usize) -> Value {
    unsafe {
        let slice = std::slice::from_raw_parts(data, len);
        let mut buffer = slice.to_vec();

        let len = buffer.len();
        let capacity = buffer.capacity();
        let data = buffer.as_mut_ptr();
        std::mem::forget(buffer);

        let runtime_str = Box::new(RuntimeString {
            data,
            len,
            capacity,
        });

        Value::ptr(Box::into_raw(runtime_str) as *mut u8)
    }
}

/// Add two values
#[no_mangle]
pub extern "C" fn toplang_add(a: Value, b: Value) -> Value {
    if a.is_number() && b.is_number() {
        Value::number(a.as_number() + b.as_number())
    } else if a.is_ptr() && b.is_ptr() {
        // String concatenation
        unsafe {
            let str_a = &*(a.as_ptr() as *const RuntimeString);
            let str_b = &*(b.as_ptr() as *const RuntimeString);

            let slice_a = std::slice::from_raw_parts(str_a.data, str_a.len);
            let slice_b = std::slice::from_raw_parts(str_b.data, str_b.len);

            let mut result = Vec::with_capacity(str_a.len + str_b.len);
            result.extend_from_slice(slice_a);
            result.extend_from_slice(slice_b);

            let len = result.len();
            let capacity = result.capacity();
            let data = result.as_mut_ptr();
            std::mem::forget(result);

            let runtime_str = Box::new(RuntimeString {
                data,
                len,
                capacity,
            });

            Value::ptr(Box::into_raw(runtime_str) as *mut u8)
        }
    } else {
        Value::null()
    }
}

/// Subtract two numbers
#[no_mangle]
pub extern "C" fn toplang_subtract(a: Value, b: Value) -> Value {
    if a.is_number() && b.is_number() {
        Value::number(a.as_number() - b.as_number())
    } else {
        Value::null()
    }
}

/// Multiply two numbers
#[no_mangle]
pub extern "C" fn toplang_multiply(a: Value, b: Value) -> Value {
    if a.is_number() && b.is_number() {
        Value::number(a.as_number() * b.as_number())
    } else {
        Value::null()
    }
}

/// Divide two numbers
#[no_mangle]
pub extern "C" fn toplang_divide(a: Value, b: Value) -> Value {
    if a.is_number() && b.is_number() {
        Value::number(a.as_number() / b.as_number())
    } else {
        Value::null()
    }
}

/// Compare two values for equality
#[no_mangle]
pub extern "C" fn toplang_equal(a: Value, b: Value) -> Value {
    if a.is_number() && b.is_number() {
        Value::boolean(a.as_number() == b.as_number())
    } else if a.is_boolean() && b.is_boolean() {
        Value::boolean(a.as_boolean() == b.as_boolean())
    } else if a.is_null() && b.is_null() {
        Value::boolean(true)
    } else {
        Value::boolean(false)
    }
}

/// Compare two numbers (less than)
#[no_mangle]
pub extern "C" fn toplang_less(a: Value, b: Value) -> Value {
    if a.is_number() && b.is_number() {
        Value::boolean(a.as_number() < b.as_number())
    } else {
        Value::boolean(false)
    }
}

/// Compare two numbers (greater than)
#[no_mangle]
pub extern "C" fn toplang_greater(a: Value, b: Value) -> Value {
    if a.is_number() && b.is_number() {
        Value::boolean(a.as_number() > b.as_number())
    } else {
        Value::boolean(false)
    }
}

/// Logical NOT
#[no_mangle]
pub extern "C" fn toplang_not(a: Value) -> Value {
    Value::boolean(!a.is_truthy())
}

/// Get array length
#[no_mangle]
pub extern "C" fn toplang_array_length(arr: Value) -> Value {
    if arr.is_ptr() {
        unsafe {
            let arr_ptr = arr.as_ptr() as *const RuntimeArray;
            let arr_ref = &*arr_ptr;
            Value::number(arr_ref.len as f64)
        }
    } else {
        Value::null()
    }
}

// Runtime array type
#[repr(C)]
pub struct RuntimeArray {
    data: *mut Value,
    len: usize,
    capacity: usize,
}

/// Create a new array
#[no_mangle]
pub extern "C" fn toplang_array_new(size: i64) -> Value {
    let size = size as usize;
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(Value::null());
    }

    let len = vec.len();
    let capacity = vec.capacity();
    let data = vec.as_mut_ptr();
    std::mem::forget(vec);

    let runtime_arr = Box::new(RuntimeArray {
        data,
        len,
        capacity,
    });

    Value::ptr(Box::into_raw(runtime_arr) as *mut u8)
}

/// Get array element
#[no_mangle]
pub extern "C" fn toplang_array_get(arr: Value, index: Value) -> Value {
    if arr.is_ptr() && index.is_number() {
        unsafe {
            let arr_ptr = arr.as_ptr() as *const RuntimeArray;
            let arr_ref = &*arr_ptr;
            let idx = index.as_number() as usize;
            if idx < arr_ref.len {
                let values = std::slice::from_raw_parts(arr_ref.data, arr_ref.len);
                values[idx]
            } else {
                Value::null()
            }
        }
    } else {
        Value::null()
    }
}

/// Set array element
#[no_mangle]
pub extern "C" fn toplang_array_set(arr: Value, index: Value, val: Value) {
    if arr.is_ptr() && index.is_number() {
        unsafe {
            let arr_ptr = arr.as_ptr() as *mut RuntimeArray;
            let arr_ref = &mut *arr_ptr;
            let idx = index.as_number() as usize;
            if idx < arr_ref.len {
                let values = std::slice::from_raw_parts_mut(arr_ref.data, arr_ref.len);
                values[idx] = val;
            }
        }
    }
}
