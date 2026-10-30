use crate::quat::Quat;
use crate::packed::QuatWord;

/// Advanced Quaternary Logic Operations (Post, Łukasiewicz, Kleene, etc.)
impl Quat {
    /// Minimum operation (Quaternary AND equivalent in Kleene/Łukasiewicz logic)
    #[inline]
    pub const fn min(self, rhs: Self) -> Self {
        if self.value() <= rhs.value() { self } else { rhs }
    }

    /// Maximum operation (Quaternary OR equivalent in Kleene/Łukasiewicz logic)
    #[inline]
    pub const fn max(self, rhs: Self) -> Self {
        if self.value() >= rhs.value() { self } else { rhs }
    }

    /// Post Cyclic Shift (Increment modulo 4): C(x) = (x + 1) mod 4
    #[inline]
    pub const fn cycle(self) -> Self {
        // Safe because (val + 1) % 4 is always in 0..=3 range
        unsafe { Self::new_unchecked((self.value() + 1) % 4) }
    }

    /// Post Inversion / Diametrical Negation: N(x) = (3 - x)
    #[inline]
    pub const fn post_neg(self) -> Self {
        unsafe { Self::new_unchecked(3 - self.value()) }
    }

    /// Łukasiewicz Implication: x -> y = min(3, 3 - x + y)
    #[inline]
    pub const fn lukasiewicz_impl(self, rhs: Self) -> Self {
        let x = self.value() as i8;
        let y = rhs.value() as i8;
        let val = if 3 < (3 - x + y) { 3 } else { 3 - x + y };
        unsafe { Self::new_unchecked(val as u8) }
    }

    /// Quaternary Delta / Literal operator: J_k(x) = 3 if x == k else 0
    #[inline]
    pub const fn literal(self, target: u8) -> Self {
        if self.value() == target {
            Self::THREE
        } else {
            Self::ZERO
        }
    }
}

/// Advanced parallel logic on 4-quat packed words
impl QuatWord {
    /// Parallel Minimum on all 4 quats packed in QuatWord
    #[inline]
    pub fn parallel_min(self, rhs: Self) -> Self {
        let mut result = 0u8;
        for i in 0..4 {
            let q1 = self.get(i).unwrap();
            let q2 = rhs.get(i).unwrap();
            let min_q = q1.min(q2);
            result |= min_q.value() << (i * 2);
        }
        Self::from_raw(result)
    }

    /// Parallel Maximum on all 4 quats packed in QuatWord
    #[inline]
    pub fn parallel_max(self, rhs: Self) -> Self {
        let mut result = 0u8;
        for i in 0..4 {
            let q1 = self.get(i).unwrap();
            let q2 = rhs.get(i).unwrap();
            let max_q = q1.max(q2);
            result |= max_q.value() << (i * 2);
        }
        Self::from_raw(result)
    }

    /// Parallel Post Cyclic Shift on all 4 packed quats
    #[inline]
    pub fn parallel_cycle(self) -> Self {
        let mut result = 0u8;
        for i in 0..4 {
            let q = self.get(i).unwrap().cycle();
            result |= q.value() << (i * 2);
        }
        Self::from_raw(result)
    }
}