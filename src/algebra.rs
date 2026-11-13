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

    /// Gödel implication: 3 if x <= y else y
    #[inline]
    pub const fn godel_impl(self, rhs: Self) -> Self {
        let xv = self.value();
        let yv = rhs.value();
        let res = if xv <= yv { 3 } else { yv };
        unsafe { Self::new_unchecked(res) }
    }

    /// Goguen (Product) implication on 0..3 scale: 3 if x <= y else floor(3 * y / x)
    #[inline]
    pub const fn goguen_impl(self, rhs: Self) -> Self {
        let xv = self.value();
        let yv = rhs.value();
        if xv <= yv {
            Self::THREE
        } else {
            let res = (3 * yv) / xv;
            unsafe { Self::new_unchecked(res) }
        }
    }

    /// Kleene-Dienes implication: max(3 - x, y)
    #[inline]
    pub const fn kleene_dienes_impl(self, rhs: Self) -> Self {
        let neg_x = 3 - self.value();
        let yv = rhs.value();
        let res = if neg_x >= yv { neg_x } else { yv };
        unsafe { Self::new_unchecked(res) }
    }

    // ==========================================
    // --- T-Norms & T-Conorms (S-Norms) ---
    // ==========================================

    /// Minimum T-Norm (Standard Gödel T-Norm)
    #[inline]
    pub const fn t_norm_min(self, rhs: Self) -> Self {
        self.min(rhs)
    }

    /// Product T-Norm scaled to quaternary: floor(x * y / 3)
    #[inline]
    pub const fn t_norm_product(self, rhs: Self) -> Self {
        let res = (self.value() * rhs.value()) / 3;
        unsafe { Self::new_unchecked(res) }
    }

    /// Łukasiewicz T-Norm: max(0, x + y - 3)
    #[inline]
    pub const fn t_norm_lukasiewicz(self, rhs: Self) -> Self {
        let sum = (self.value() as i8) + (rhs.value() as i8) - 3;
        let res = if sum > 0 { sum as u8 } else { 0 };
        unsafe { Self::new_unchecked(res) }
    }

    /// Łukasiewicz T-Conorm (S-Norm): min(3, x + y)
    #[inline]
    pub const fn s_norm_lukasiewicz(self, rhs: Self) -> Self {
        let sum = self.value() + rhs.value();
        let res = if sum < 3 { sum } else { 3 };
        unsafe { Self::new_unchecked(res) }
    }

    // ==========================================
    // --- Modal Operators ---
    // ==========================================

    /// Necessity Operator (Box): Returns THREE if strictly THREE (3), else ZERO (0).
    #[inline]
    pub const fn necessity(self) -> Self {
        if self.value() == 3 { Self::THREE } else { Self::ZERO }
    }

    /// Possibility Operator (Diamond): Returns ZERO if strictly ZERO (0), else THREE (3).
    #[inline]
    pub const fn possibility(self) -> Self {
        if self.value() == 0 { Self::ZERO } else { Self::THREE }
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