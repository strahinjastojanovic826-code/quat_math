use crate::quat::Quat;
use crate::error::QuatError;

/// Packs 4 quaternary digits into a single standard `u8`.
/// Layout: [Q3: 7..6] [Q2: 5..4] [Q1: 3..2] [Q0: 1..0]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuatWord(u8);

impl QuatWord {
    #[inline]
    pub const fn new(q0: Quat, q1: Quat, q2: Quat, q3: Quat) -> Self {
        let val = q0.value() 
            | (q1.value() << 2) 
            | (q2.value() << 4) 
            | (q3.value() << 6);
        Self(val)
    }

    #[inline]
    pub const fn from_raw(val: u8) -> Self {
        Self(val)
    }

    #[inline]
    pub fn get(&self, index: usize) -> Result<Quat, QuatError> {
        if index < 4 {
            let shift = (index * 2) as u8;
            let val = (self.0 >> shift) & 0b11;
            Ok(unsafe { Quat::new_unchecked(val) })
        } else {
            Err(QuatError::IndexOutOfBounds { index, max: 3 })
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize, q: Quat) -> Result<(), QuatError> {
        if index < 4 {
            let shift = (index * 2) as u8;
            self.0 &= !(0b11 << shift);
            self.0 |= q.value() << shift;
            Ok(())
        } else {
            Err(QuatError::IndexOutOfBounds { index, max: 3 })
        }
    }

    #[inline]
    pub const fn raw(&self) -> u8 {
        self.0
    }
}

impl From<QuatWord> for u8 {
    #[inline]
    fn from(qw: QuatWord) -> u8 {
        qw.0
    }
}

impl From<u8> for QuatWord {
    #[inline]
    fn from(val: u8) -> Self {
        Self(val)
    }
}