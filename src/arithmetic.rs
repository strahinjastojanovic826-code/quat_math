use crate::quat::Quat;
use std::ops::{Add, Mul, Sub};

impl Add for Quat {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.value() + rhs.value();
        unsafe { Quat::new_unchecked(sum % 4) }
    }
}

impl Sub for Quat {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let diff = (self.value() as i8 - rhs.value() as i8).rem_euclid(4) as u8;
        unsafe { Quat::new_unchecked(diff) }
    }
}

impl Mul for Quat {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let prod = self.value() * rhs.value();
        unsafe { Quat::new_unchecked(prod % 4) }
    }
}