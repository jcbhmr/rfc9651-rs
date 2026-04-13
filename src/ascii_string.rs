use alloc::{
    string::String,
    vec::{Drain, Vec},
};
use core::{
    mem,
    ops::{Deref, RangeBounds},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AsciiString(Vec<u8>);

impl AsciiString {
    pub fn from_string(s: String) -> Result<Self, String> {
        if s.is_ascii() {
            Ok(Self(s.into_bytes()))
        } else {
            Err(s)
        }
    }

    #[inline]
    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> Drain<'_, u8> {
        self.0.drain(range)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.0) }
    }

    #[inline]
    pub fn as_ascii_str(&self) -> &AsciiStr {
        unsafe { AsciiStr::from_ascii_unchecked(&self.0) }
    }
}

impl Deref for AsciiString {
    type Target = AsciiStr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ascii_str()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AsciiStr([u8]);

impl AsciiStr {
    #[inline]
    pub unsafe fn from_ascii_unchecked(s: &[u8]) -> &Self {
        unsafe { mem::transmute(s) }
    }
}
