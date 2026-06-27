// src/diagnostics/time_format/nanoseconds_str.rs : `NanosecondsStr`

use std::{
    borrow::Borrow,
    fmt as std_fmt,
    ops::Deref,
};


const INLINE_CAP : usize = 15;


/// Compact storage for a formatted nanosecond duration string.
///
/// Obtain values from [`nanoseconds_to_string`]. Most outputs fit in
/// [`INLINE_CAP`] UTF-8 bytes and are stored inline without heap
/// allocation. Longer results use a [`String`] variant.
#[derive(Clone)]
#[derive(Eq)]
pub struct NanosecondsStr {
    inner : NanosecondsStrInner,
}

#[derive(Clone)]
#[derive(Eq, PartialEq)]
enum NanosecondsStrInner {
    Inline {
        len : u8,
        bytes : [u8; INLINE_CAP],
    },
    Heap(String),
}


// API functions

impl NanosecondsStr {
    pub(in crate::diagnostics::time_format) fn from_buffer(buf : &[u8]) -> Self {
        debug_assert!(std::str::from_utf8(buf).is_ok());

        if buf.len() <= INLINE_CAP {
            let mut bytes = [0u8; INLINE_CAP];

            bytes[..buf.len()].copy_from_slice(buf);

            Self {
                inner : NanosecondsStrInner::Inline {
                    len : buf.len() as u8,
                    bytes,
                },
            }
        } else {
            Self {
                inner : NanosecondsStrInner::Heap(String::from_utf8(buf.to_vec()).unwrap()),
            }
        }
    }
}


// Mutating methods

impl NanosecondsStr {
}


// Nonmutating methods

impl NanosecondsStr {
    /// Borrows the formatted UTF-8 string.
    pub fn as_str(&self) -> &str {
        match &self.inner {
            NanosecondsStrInner::Inline {
                len,
                bytes,
            } => {
                let len = *len as usize;

                // SAFETY: `bytes` holds valid UTF-8 written by this module.
                unsafe { std::str::from_utf8_unchecked(&bytes[..len]) }
            },
            NanosecondsStrInner::Heap(s) => s.as_str(),
        }
    }

    #[cfg(test)]
    fn is_heap(&self) -> bool {
        matches!(self.inner, NanosecondsStrInner::Heap(_))
    }
}


// Trait implementations

impl AsRef<str> for NanosecondsStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}


impl Borrow<str> for NanosecondsStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}


impl std_fmt::Debug for NanosecondsStr {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        std_fmt::Debug::fmt(self.as_str(), f)
    }
}


impl Deref for NanosecondsStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}


impl std_fmt::Display for NanosecondsStr {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        f.write_str(self.as_str())
    }
}


impl From<NanosecondsStr> for String {
    fn from(value : NanosecondsStr) -> Self {
        match value.inner {
            NanosecondsStrInner::Inline {
                len,
                bytes,
            } => {
                let len = len as usize;

                String::from_utf8(bytes[..len].to_vec()).unwrap()
            },
            NanosecondsStrInner::Heap(s) => s,
        }
    }
}


impl PartialEq<NanosecondsStr> for &str {
    fn eq(
        &self,
        other : &NanosecondsStr,
    ) -> bool {
        *self == other.as_str()
    }
}


impl PartialEq<&str> for NanosecondsStr {
    fn eq(
        &self,
        other : &&str,
    ) -> bool {
        self.as_str() == *other
    }
}


impl PartialEq for NanosecondsStr {
    fn eq(
        &self,
        other : &Self,
    ) -> bool {
        self.as_str() == other.as_str()
    }
}


impl PartialEq<str> for NanosecondsStr {
    fn eq(
        &self,
        other : &str,
    ) -> bool {
        self.as_str() == other
    }
}


impl PartialEq<NanosecondsStr> for str {
    fn eq(
        &self,
        other : &NanosecondsStr,
    ) -> bool {
        self == other.as_str()
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::{
        NanosecondsStr,
        INLINE_CAP,
    };

    use super::super::format::nanoseconds_to_string;


    fn heap_from_bytes(bytes : &[u8]) -> NanosecondsStr {
        NanosecondsStr::from_buffer(bytes)
    }


    #[test]
    fn TEST_NanosecondsStr_INLINE_STORAGE() {
        let s = nanoseconds_to_string(123_456_789, "");

        assert!(!s.is_heap());
        assert!(s.as_str().len() <= INLINE_CAP);
        assert_eq!("123.4ms", s);
        assert_eq!("123.4ms", s.as_str());
        assert_eq!("123.4ms", &*s);
        assert_eq!("123.4ms", s.as_ref());
        assert_eq!("123.4ms", String::from(s.clone()));
    }


    #[test]
    fn TEST_NanosecondsStr_INLINE_AT_INLINE_CAP() {
        let expected = "a".repeat(INLINE_CAP);
        let s = heap_from_bytes(expected.as_bytes());

        assert!(!s.is_heap());
        assert_eq!(expected.as_str(), s);
        assert_eq!(expected, String::from(s));
    }


    #[test]
    fn TEST_NanosecondsStr_HEAP_AT_INLINE_CAP_PLUS_ONE() {
        let expected = "b".repeat(INLINE_CAP + 1);
        let s = heap_from_bytes(expected.as_bytes());

        assert!(s.is_heap());
        assert_eq!(expected.as_str(), s);
    }


    #[test]
    fn TEST_NanosecondsStr_HEAP_STORAGE() {
        let expected = "x".repeat(INLINE_CAP + 4);
        let s = heap_from_bytes(expected.as_bytes());

        assert!(s.is_heap());
        assert_eq!(expected, s.as_str());
        assert_eq!(expected, &*s);
        assert_eq!(expected, s.as_ref());
        assert_eq!(expected, format!("{s}"));
        assert_eq!(format!("{expected:?}"), format!("{s:?}"));
        assert_eq!(expected, String::from(s));
    }


    #[test]
    fn TEST_NanosecondsStr_HEAP_Clone_AND_String() {
        let expected = "overflow-duration-value";
        let original = heap_from_bytes(expected.as_bytes());
        let cloned = original.clone();

        assert!(original.is_heap());
        assert!(cloned.is_heap());
        assert_eq!(expected, cloned.as_str());
        assert_eq!(expected, String::from(cloned));
    }


    #[test]
    fn TEST_nanoseconds_to_string_ALL_PARITY_CASES_INLINE() {
        #[rustfmt::skip]
        let nanoseconds = [
                                 0_i64,
                                 9,
                                89,
                               789,
                             6_789,
                            56_789,
                           456_789,
                         3_456_789,
                        23_456_789,
                       123_456_789,
                     9_123_456_789,
                    89_123_456_789,
                   789_123_456_789,
                                80,
                               700,
                             6_000,
                            50_000,
                           400_000,
                         3_000_000,
                        20_000_000,
                       100_000_000,
                     9_000_000_000,
                    10_000_000_000,
                   200_000_000_000,
                 3_000_000_000_000,
                40_000_000_000_000,
               500_000_000_000_000,
             6_000_000_000_000_000,
            70_000_000_000_000_000,
                    11_111_111_111,
                   222_222_222_222,
                 3_333_333_333_333,
                44_444_444_444_444,
               555_555_555_555_555,
             6_666_666_666_666_666,
            77_777_777_777_777_777,
                                -9,
                               -89,
                              -789,
                            -6_789,
                           -56_789,
                          -456_789,
                        -3_456_789,
                       -23_456_789,
                      -123_456_789,
                    -9_123_456_789,
                       999_772_000,
                       999_800_000,
                       999_974_000,
                      -999_772_000,
                      -999_800_000,
                      -999_974_000,
        ];

        for value in nanoseconds {
            let s = nanoseconds_to_string(value, "");

            assert!(
                !s.is_heap(),
                "expected inline storage for {value}, got {:?}",
                s.as_str(),
            );
            assert!(s.as_str().len() <= INLINE_CAP);
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
