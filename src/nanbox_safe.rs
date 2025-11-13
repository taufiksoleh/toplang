/// Safe NaN-boxed value representation using Rc for heap types
///
/// All values fit in a single 64-bit word by exploiting IEEE 754 NaN representation.
/// Heap-allocated types (String, Array) use Rc for safe automatic memory management.
///
/// Encoding scheme:
/// - Normal numbers: Standard IEEE 754 f64
/// - Special values use NaN bit patterns:
///   - Null:  0x7FF8_0000_0000_0000
///   - False: 0x7FF8_0000_0000_0001
///   - True:  0x7FF8_0000_0000_0002
///   - String: 0x7FF8_0000_0000_0003 + 48-bit Rc pointer
///   - Array:  0x7FF8_0000_0000_0004 + 48-bit Rc pointer

use std::rc::Rc;

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

// Mask for checking type tags (upper 16 bits + lower 4 bits)
const TYPE_MASK: u64 = 0xFFFF_0000_0000_000F;

/// A NaN-boxed value - all types fit in 64 bits
/// Uses Rc for safe heap memory management
pub struct NanValue(u64);

impl NanValue {
    // ===== Constructors =====

    #[inline]
    pub fn number(n: f64) -> Self {
        NanValue(n.to_bits())
    }

    #[inline]
    pub fn null() -> Self {
        NanValue(TAG_NULL)
    }

    #[inline]
    pub fn boolean(b: bool) -> Self {
        if b {
            NanValue(TAG_TRUE)
        } else {
            NanValue(TAG_FALSE)
        }
    }

    #[inline]
    pub fn string(s: String) -> Self {
        let rc = Rc::new(s);
        let ptr = Rc::into_raw(rc) as u64;
        NanValue(TAG_STRING | (ptr & POINTER_MASK))
    }

    #[inline]
    pub fn array(arr: Vec<NanValue>) -> Self {
        let rc = Rc::new(arr);
        let ptr = Rc::into_raw(rc) as u64;
        NanValue(TAG_ARRAY | (ptr & POINTER_MASK))
    }

    // Constant for stack initialization
    pub const NULL_VALUE: NanValue = NanValue(TAG_NULL);

    // ===== Type checks =====

    #[inline]
    pub fn is_number(&self) -> bool {
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
        (self.0 & TYPE_MASK) == TAG_STRING
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        (self.0 & TYPE_MASK) == TAG_ARRAY
    }

    // ===== Extractors =====

    #[inline]
    pub fn as_number(&self) -> Option<f64> {
        if self.is_number() {
            Some(f64::from_bits(self.0))
        } else {
            None
        }
    }

    #[inline]
    pub fn as_boolean(&self) -> Option<bool> {
        if self.0 == TAG_TRUE {
            Some(true)
        } else if self.0 == TAG_FALSE {
            Some(false)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_string(&self) -> Option<Rc<String>> {
        if self.is_string() {
            // Extract pointer by masking out tag bits (keep only middle 45 bits for pointer)
            let ptr = ((self.0 & POINTER_MASK) & !0xF) as *const String;
            // Clone the Rc to increment reference count
            unsafe {
                Rc::increment_strong_count(ptr);
                Some(Rc::from_raw(ptr))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn as_array(&self) -> Option<Rc<Vec<NanValue>>> {
        if self.is_array() {
            // Extract pointer by masking out tag bits (keep only middle 45 bits for pointer)
            let ptr = ((self.0 & POINTER_MASK) & !0xF) as *const Vec<NanValue>;
            // Clone the Rc to increment reference count
            unsafe {
                Rc::increment_strong_count(ptr);
                Some(Rc::from_raw(ptr))
            }
        } else {
            None
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
            if let Some(s) = self.as_string() {
                !s.is_empty()
            } else {
                false
            }
        } else if self.is_array() {
            if let Some(a) = self.as_array() {
                !a.is_empty()
            } else {
                false
            }
        } else {
            true
        }
    }

    // ===== Equality =====

    pub fn equals(&self, other: &NanValue) -> bool {
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
            if let (Some(a), Some(b)) = (self.as_string(), other.as_string()) {
                *a == *b
            } else {
                false
            }
        } else if self.is_array() && other.is_array() {
            if let (Some(a), Some(b)) = (self.as_array(), other.as_array()) {
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
}

// Implement Clone manually to properly handle Rc reference counting
impl Clone for NanValue {
    fn clone(&self) -> Self {
        if self.is_string() {
            let ptr = ((self.0 & POINTER_MASK) & !0xF) as *const String;
            unsafe {
                // Increment reference count for the new clone
                Rc::increment_strong_count(ptr);
            }
            NanValue(self.0)
        } else if self.is_array() {
            let ptr = ((self.0 & POINTER_MASK) & !0xF) as *const Vec<NanValue>;
            unsafe {
                // Increment reference count for the new clone
                Rc::increment_strong_count(ptr);
            }
            NanValue(self.0)
        } else {
            // Numbers, booleans, null are just copied
            NanValue(self.0)
        }
    }
}

// Implement Drop to properly decrement Rc when value goes out of scope
impl Drop for NanValue {
    fn drop(&mut self) {
        if self.is_string() {
            let ptr = ((self.0 & POINTER_MASK) & !0xF) as *const String;
            unsafe {
                // Decrement reference count (and free if zero)
                drop(Rc::from_raw(ptr));
            }
        } else if self.is_array() {
            let ptr = ((self.0 & POINTER_MASK) & !0xF) as *const Vec<NanValue>;
            unsafe {
                // Decrement reference count (and free if zero)
                drop(Rc::from_raw(ptr));
            }
        }
    }
}

impl std::fmt::Display for NanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "null")
        } else if let Some(b) = self.as_boolean() {
            write!(f, "{}", b)
        } else if let Some(n) = self.as_number() {
            if n.fract() == 0.0 && n.is_finite() {
                write!(f, "{}", n as i64)
            } else {
                write!(f, "{}", n)
            }
        } else if let Some(s) = self.as_string() {
            write!(f, "{}", *s)
        } else if let Some(arr) = self.as_array() {
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

impl std::fmt::Debug for NanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "Null")
        } else if let Some(b) = self.as_boolean() {
            write!(f, "Boolean({})", b)
        } else if let Some(n) = self.as_number() {
            write!(f, "Number({})", n)
        } else if let Some(s) = self.as_string() {
            write!(f, "String({:?})", *s)
        } else if let Some(arr) = self.as_array() {
            write!(f, "Array({:?})", *arr)
        } else {
            write!(f, "Unknown(0x{:016x})", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let v = NanValue::number(42.0);
        assert!(v.is_number());
        assert_eq!(v.as_number(), Some(42.0));
    }

    #[test]
    fn test_boolean() {
        let t = NanValue::boolean(true);
        let f = NanValue::boolean(false);
        assert!(t.is_boolean());
        assert!(f.is_boolean());
        assert_eq!(t.as_boolean(), Some(true));
        assert_eq!(f.as_boolean(), Some(false));
    }

    #[test]
    fn test_null() {
        let v = NanValue::null();
        assert!(v.is_null());
        assert!(!v.is_truthy());
    }

    #[test]
    fn test_string() {
        let v = NanValue::string("hello".to_string());
        assert!(v.is_string());
        if let Some(s) = v.as_string() {
            assert_eq!(s.as_str(), "hello");
        } else {
            panic!("Expected string");
        }
    }

    #[test]
    fn test_string_clone() {
        let v1 = NanValue::string("test".to_string());
        let v2 = v1.clone();
        assert!(v1.is_string());
        assert!(v2.is_string());
        if let (Some(s1), Some(s2)) = (v1.as_string(), v2.as_string()) {
            assert_eq!(s1.as_str(), s2.as_str());
        }
    }

    #[test]
    fn test_array() {
        let arr = vec![NanValue::number(1.0), NanValue::number(2.0), NanValue::number(3.0)];
        let v = NanValue::array(arr);
        assert!(v.is_array());
        if let Some(a) = v.as_array() {
            assert_eq!(a.len(), 3);
        }
    }

    #[test]
    fn test_size() {
        // Verify that NanValue is exactly 64 bits
        assert_eq!(std::mem::size_of::<NanValue>(), 8);
    }

    #[test]
    fn test_equality() {
        let a = NanValue::number(42.0);
        let b = NanValue::number(42.0);
        assert!(a.equals(&b));

        let s1 = NanValue::string("test".to_string());
        let s2 = NanValue::string("test".to_string());
        assert!(s1.equals(&s2));

        let n1 = NanValue::null();
        let n2 = NanValue::null();
        assert!(n1.equals(&n2));
    }
}
