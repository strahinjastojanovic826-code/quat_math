use crate::quat::Quat;
use crate::error::QuatError;

/// A high-performance dynamic vector storing quaternary values with 4-to-1 byte packing ratio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuatVec {
    data: Vec<u8>,
    len: usize,
}

impl QuatVec {
    /// Creates a new, empty `QuatVec`.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    #[inline]
pub fn raw_data(&self) -> &[u8] {
    &self.data
}

    /// Creates a `QuatVec` with preallocated capacity for a given number of quats.
    pub fn with_capacity(capacity: usize) -> Self {
        let byte_capacity = (capacity + 3) / 4;
        Self {
            data: Vec::with_capacity(byte_capacity),
            len: 0,
        }
    }

    /// Returns the number of quats stored in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector contains no quats.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Appends a Quat to the back of the vector.
    pub fn push(&mut self, q: Quat) {
        let byte_idx = self.len / 4;
        let quat_idx = self.len % 4;
        let shift = (quat_idx * 2) as u8;

        if byte_idx >= self.data.len() {
            self.data.push(0);
        }

        self.data[byte_idx] |= q.value() << shift;
        self.len += 1;
    }

    /// Retrieves a Quat at the specified index safely.
    pub fn get(&self, index: usize) -> Result<Quat, QuatError> {
        if index >= self.len {
            return Err(QuatError::IndexOutOfBounds { 
                index, 
                max: self.len.saturating_sub(1) 
            });
        }
        let byte_idx = index / 4;
        let quat_idx = index % 4;
        let shift = (quat_idx * 2) as u8;
        let val = (self.data[byte_idx] >> shift) & 0b11;
        Ok(unsafe { Quat::new_unchecked(val) })
    }

    /// Updates a Quat at the specified index.
    pub fn set(&mut self, index: usize, q: Quat) -> Result<(), QuatError> {
        if index >= self.len {
            return Err(QuatError::IndexOutOfBounds { 
                index, 
                max: self.len.saturating_sub(1) 
            });
        }
        let byte_idx = index / 4;
        let quat_idx = index % 4;
        let shift = (quat_idx * 2) as u8;

        self.data[byte_idx] &= !(0b11 << shift);
        self.data[byte_idx] |= q.value() << shift;
        Ok(())
    }
}

impl Default for QuatVec {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<Quat> for QuatVec {
    fn from_iter<T: IntoIterator<Item = Quat>>(iter: T) -> Self {
        let mut vec = Self::new();
        for q in iter {
            vec.push(q);
        }
        vec
    }
}