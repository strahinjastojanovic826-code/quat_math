use crate::quat::Quat;
use std::ops::{BitAnd, BitOr, BitXor, Not};

impl Not for Quat {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        unsafe { Quat::new_unchecked(!self.value() & 0b11) }
    }
}

impl BitAnd for Quat {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Quat::new_unchecked(self.value() & rhs.value()) }
    }
}

impl BitOr for Quat {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Quat::new_unchecked(self.value() | rhs.value()) }
    }
}

impl BitXor for Quat {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Quat::new_unchecked(self.value() ^ rhs.value()) }
    }
}