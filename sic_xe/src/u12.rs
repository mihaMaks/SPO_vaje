#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U12(u16);

impl U12 {
    /// Creates a new `U12` instance, ensuring the value fits in 12 bits.
    /// Values exceeding 12 bits will be truncated to fit within [0, 4095].
    pub fn new(value: u16) -> Self {
        U12(value & 0x0FFF) // Mask to 12 bits
    }

    /// Creates a new `U12` instance by clamping the value to the range [0, 4095].
    pub fn from_clamped(value: u16) -> Self {
        U12(value.min(0x0FFF)) // Clamp to 4095
    }

    /// Returns the inner value as `u16`.
    pub fn get(self) -> u16 {
        self.0
    }
}

// Implement basic arithmetic operations for U12
impl std::ops::Add for U12 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let result = (self.0 + rhs.0) & 0x0FFF; // Mask to 12 bits
        U12(result)
    }
}

impl std::ops::Sub for U12 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let result = self.0.wrapping_sub(rhs.0) & 0x0FFF; // Handle underflow gracefully, mask to 12 bits
        U12(result)
    }
}

impl std::ops::Mul for U12 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result = (self.0 * rhs.0) & 0x0FFF; // Mask to 12 bits
        U12(result)
    }
}

impl std::ops::Div for U12 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        assert!(rhs.0 != 0, "Division by zero is not allowed");
        let result = self.0 / rhs.0; // No need to mask since division will not exceed 12 bits
        U12(result)
    }
}
