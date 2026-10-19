use std::fmt;

/// Advanced error types for quaternary operations and containers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuatError {
    /// The provided raw value exceeds the 2-bit range (must be 0..=3).
    InvalidValue(u8),
    /// The requested index is out of bounds for the container.
    IndexOutOfBounds { index: usize, max: usize },
    /// Length mismatch encountered during operations.
    LengthMismatch { expected: usize, found: usize },
}

impl fmt::Display for QuatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuatError::InvalidValue(val) => {
                write!(f, "Invalid quaternary value: {}. Must be between 0 and 3 (00 to 11).", val)
            }
            QuatError::IndexOutOfBounds { index, max } => {
                write!(f, "Index out of bounds: index {} is out of maximum valid range {}.", index, max)
            }
            QuatError::LengthMismatch { expected, found } => {
                write!(f, "Length mismatch: expected {} elements, but found {}.", expected, found)
            }
        }
    }
}

impl std::error::Error for QuatError {}