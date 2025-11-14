#![allow(
    dead_code,
    non_snake_case,
    clippy::bad_bit_mask,
    clippy::non_canonical_clone_impl
)]

//! NaN-boxed value representation for high performance
//!
//! All values fit in a single 64-bit word by exploiting the IEEE 754 NaN representation.
//! This eliminates enum overhead and enables much faster stack operations.
//!
//! Encoding scheme:
//! - Normal numbers: Standard IEEE 754 f64
//! - Special values use NaN bit patterns:
//!   - Null:  0x7FF8_0000_0000_0000
//!   - False: 0x7FF8_0000_0000_0001
//!   - True:  0x7FF8_0000_0000_0002
//!   - Pointer: 0x7FF8_xxxx_xxxx_xxxx (48-bit pointer in lower bits)
//!
//! NOTE: This is the original implementation. The production version is in nanbox_safe.rs

use anyhow::{anyhow, Result};

// NaN mask: exponent all 1s, mantissa non-zero
const QNAN: u64 = 0x7FF8_0000_0000_0000;

// Special value tags
const TAG_NULL: u64 = QNAN;
const TAG_FALSE: u64 = QNAN | 1;
const TAG_TRUE: u64 = QNAN | 2;
const TAG_STRING: u64 = QNAN | 3;
const TAG_ARRAY: u64 = QNAN | 4;

// Mask for extracting pointer (lower 48 bits)
const POINTER_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

/// A NaN-boxed value - all types fit in 64 bits
/// Note: We implement Clone manually to handle heap-allocated data properly
pub struct Value(u64);

impl Clone for Value {
    fn clone(&self) -> Self {
        // For NaN-boxed values with pointers, we just copy the bits
        // This means multiple Values can point to the same heap data
        // We rely on the VM to manage lifetimes properly
        Value(self.0)
    }
}

impl Copy for Value {}

impl Value {
    // ===== Constructors =====

    #[inline]
    pub fn Number(n: f64) -> Self {
        Value(n.to_bits())
    }

    #[inline]
    pub fn Null() -> Self {
        Value(TAG_NULL)
    }

    // Constant for stack initialization
    pub const NULL_VALUE: Value = Value(TAG_NULL);

    #[inline]
    pub fn Boolean(b: bool) -> Self {
        if b {
            Value(TAG_TRUE)
        } else {
            Value(TAG_FALSE)
        }
    }

    #[inline]
    pub fn String(s: String) -> Self {
        let ptr = Box::into_raw(Box::new(s)) as u64;
        Value(TAG_STRING | (ptr & POINTER_MASK))
    }

    #[inline]
    pub fn Array(arr: Vec<Value>) -> Self {
        let ptr = Box::into_raw(Box::new(arr)) as u64;
        Value(TAG_ARRAY | (ptr & POINTER_MASK))
    }

    // Lowercase aliases for compatibility
    #[inline]
    pub fn number(n: f64) -> Self {
        Self::Number(n)
    }

    #[inline]
    pub fn null() -> Self {
        Self::Null()
    }

    #[inline]
    pub fn boolean(b: bool) -> Self {
        Self::Boolean(b)
    }

    #[inline]
    pub fn string(s: String) -> Self {
        Self::String(s)
    }

    #[inline]
    pub fn array(arr: Vec<Value>) -> Self {
        Self::Array(arr)
    }

    // ===== Type checks =====

    #[inline]
    pub fn is_number(&self) -> bool {
        // If it's not a QNAN, it's a number
        (self.0 & QNAN) != QNAN
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == TAG_NULL
    }

    #[inline]
    pub fn is_boolean(&self) -> bool {
        self.0 == TAG_TRUE || self.0 == TAG_FALSE
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        (self.0 & !POINTER_MASK) == TAG_STRING
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        (self.0 & !POINTER_MASK) == TAG_ARRAY
    }

    // ===== Extractors (with error handling for VM compatibility) =====

    #[inline]
    pub fn as_number(&self) -> Result<f64> {
        if self.is_number() {
            Ok(f64::from_bits(self.0))
        } else {
            Err(anyhow!("Expected number, got {:?}", self))
        }
    }

    #[inline]
    pub fn as_boolean(&self) -> Result<bool> {
        if self.0 == TAG_TRUE {
            Ok(true)
        } else if self.0 == TAG_FALSE {
            Ok(false)
        } else {
            Err(anyhow!("Expected boolean, got {:?}", self))
        }
    }

    #[inline]
    pub fn as_string(&self) -> Result<String> {
        if self.is_string() {
            let ptr = (self.0 & POINTER_MASK) as *const String;
            Ok(unsafe { (*ptr).clone() })
        } else {
            Err(anyhow!("Expected string, got {:?}", self))
        }
    }

    #[inline]
    pub fn as_string_ref(&self) -> Result<&String> {
        if self.is_string() {
            let ptr = (self.0 & POINTER_MASK) as *const String;
            Ok(unsafe { &*ptr })
        } else {
            Err(anyhow!("Expected string, got {:?}", self))
        }
    }

    #[inline]
    pub fn as_array(&self) -> Result<Vec<Value>> {
        if self.is_array() {
            let ptr = (self.0 & POINTER_MASK) as *const Vec<Value>;
            Ok(unsafe { (*ptr).clone() })
        } else {
            Err(anyhow!("Expected array, got {:?}", self))
        }
    }

    #[inline]
    pub fn as_array_ref(&self) -> Result<&Vec<Value>> {
        if self.is_array() {
            let ptr = (self.0 & POINTER_MASK) as *const Vec<Value>;
            Ok(unsafe { &*ptr })
        } else {
            Err(anyhow!("Expected array, got {:?}", self))
        }
    }

    // ===== Truthiness =====

    #[inline]
    pub fn is_truthy(&self) -> bool {
        if self.is_null() || self.0 == TAG_FALSE {
            false
        } else if self.is_number() {
            let n = f64::from_bits(self.0);
            n != 0.0
        } else if self.is_string() {
            !self.as_string().unwrap().is_empty()
        } else if self.is_array() {
            !self.as_array().unwrap().is_empty()
        } else {
            true
        }
    }

    // ===== Equality =====

    pub fn equals(&self, other: &Value) -> bool {
        // Fast path: exact bit match
        if self.0 == other.0 {
            return true;
        }

        // Type-specific comparison
        if self.is_number() && other.is_number() {
            let a = f64::from_bits(self.0);
            let b = f64::from_bits(other.0);
            (a - b).abs() < f64::EPSILON
        } else if self.is_string() && other.is_string() {
            if let (Ok(a), Ok(b)) = (self.as_string_ref(), other.as_string_ref()) {
                a == b
            } else {
                false
            }
        } else if self.is_array() && other.is_array() {
            if let (Ok(a), Ok(b)) = (self.as_array_ref(), other.as_array_ref()) {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(x, y)| x.equals(y))
            } else {
                false
            }
        } else {
            false
        }
    }

    // ===== Memory management =====

    /// Clone the value (deep copy for heap-allocated types)
    pub fn deep_clone(&self) -> Value {
        if self.is_string() {
            Value::String(self.as_string().unwrap_or_default())
        } else if self.is_array() {
            let arr = self.as_array().unwrap_or_default();
            let cloned: Vec<Value> = arr.iter().map(|v| v.deep_clone()).collect();
            Value::Array(cloned)
        } else {
            // Numbers, booleans, null are just copied
            *self
        }
    }

    /// Drop heap-allocated data
    pub unsafe fn drop_in_place(&mut self) {
        if self.is_string() {
            let ptr = (self.0 & POINTER_MASK) as *mut String;
            drop(Box::from_raw(ptr));
        } else if self.is_array() {
            let ptr = (self.0 & POINTER_MASK) as *mut Vec<Value>;
            // Recursively drop array elements
            let mut arr = Box::from_raw(ptr);
            for val in arr.iter_mut() {
                val.drop_in_place();
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "null")
        } else if let Ok(b) = self.as_boolean() {
            write!(f, "{}", b)
        } else if let Ok(n) = self.as_number() {
            if n.fract() == 0.0 && n.is_finite() {
                write!(f, "{}", n as i64)
            } else {
                write!(f, "{}", n)
            }
        } else if let Ok(s) = self.as_string_ref() {
            write!(f, "{}", s)
        } else if let Ok(arr) = self.as_array_ref() {
            write!(f, "[")?;
            for (i, val) in arr.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", val)?;
            }
            write!(f, "]")
        } else {
            write!(f, "<unknown>")
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "Null")
        } else if let Ok(b) = self.as_boolean() {
            write!(f, "Boolean({})", b)
        } else if let Ok(n) = self.as_number() {
            write!(f, "Number({})", n)
        } else if let Ok(s) = self.as_string_ref() {
            write!(f, "String({:?})", s)
        } else if let Ok(arr) = self.as_array_ref() {
            write!(f, "Array({:?})", arr)
        } else {
            write!(f, "Unknown(0x{:016x})", self.0)
        }
    }
}

/* Tests disabled for deprecated nanbox implementation - use nanbox_safe.rs instead

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let v = Value::number(42.0);
        assert!(v.is_number());
        assert_eq!(v.as_number(), Some(42.0));
        assert_eq!(format!("{}", v), "42");
    }

    #[test]
    fn test_boolean() {
        let t = Value::boolean(true);
        let f = Value::boolean(false);
        assert!(t.is_boolean());
        assert!(f.is_boolean());
        assert_eq!(t.as_boolean(), Some(true));
        assert_eq!(f.as_boolean(), Some(false));
    }

    #[test]
    fn test_null() {
        let v = Value::null();
        assert!(v.is_null());
        assert!(!v.is_truthy());
    }

    #[test]
    fn test_string() {
        let v = Value::string("hello".to_string());
        assert!(v.is_string());
        assert_eq!(v.as_string(), Some(&"hello".to_string()));
        assert_eq!(format!("{}", v), "hello");
    }

    #[test]
    fn test_array() {
        let arr = vec![Value::number(1.0), Value::number(2.0), Value::number(3.0)];
        let v = Value::array(arr);
        assert!(v.is_array());
        assert_eq!(v.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_size() {
        // Verify that Value is exactly 64 bits
        assert_eq!(std::mem::size_of::<Value>(), 8);
    }

    #[test]
    fn test_equality() {
        let a = Value::number(42.0);
        let b = Value::number(42.0);
        assert!(a.equals(&b));

        let s1 = Value::string("test".to_string());
        let s2 = Value::string("test".to_string());
        assert!(s1.equals(&s2));
    }
}
*/
