use crate::error::QuatError;
use std::fmt;

/// Represents a single quaternary digit (quat) packed into 2 bits.
/// Values range from 0 to 3 (binary: 00, 01, 10, 11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Quat(u8);

impl Quat {
    pub const ZERO: Self = Self(0); // 00
    pub const ONE: Self = Self(1);  // 01
    pub const TWO: Self = Self(2);  // 10
    pub const THREE: Self = Self(3);// 11

    /// Creates a new Quat with full boundary validation as a const-compatible check.
    #[inline]
    pub const fn new(val: u8) -> Result<Self, QuatError> {
        if val < 4 {
            Ok(Self(val))
        } else {
            Err(QuatError::InvalidValue(val))
        }
    }

    /// Creates a Quat without bounds checking (masks lower 2 bits).
    #[inline]
    pub const unsafe fn new_unchecked(val: u8) -> Self {
        Self(val & 0b11)
    }

    /// Returns the raw 2-bit value as a u8.
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Quat {
    type Error = QuatError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Quat::new(value)
    }
}

impl From<Quat> for u8 {
    #[inline]
    fn from(q: Quat) -> u8 {
        q.0
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02b}", self.0)
    }
}