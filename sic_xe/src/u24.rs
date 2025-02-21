#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U24(u64);

impl U24 {
    /// Creates a new `U24` instance, ensuring the value fits in 24 bits.
    /// Panics if the value exceeds 24 bits.
    pub fn new(value: u64) -> Self {
        assert!(
            value <= 0xFFFFFF,
            "Value out of bounds for U24: {:#X}",
            value
        );
        U24(value)
    }

    /// Returns the inner value as `u64`.
    pub fn get(self) -> u32 {
        let my_u32: u32 = (self.0 & 0xFFFFFFFF) as u32;
        return my_u32;
    }
}

impl std::ops::Add for U24 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let result = (self.0 + rhs.0) & 0xFFFFFF; // Mask the result to 24 bits
        U24(result)
    }
}

impl std::ops::Sub for U24 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let result = (self.0.wrapping_sub(rhs.0)) & 0xFFFFFF; // Handle underflow gracefully
        U24(result)
    }
}

impl std::ops::Mul for U24 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result = (self.0 * rhs.0) & 0xFFFFFF; // Mask the result to 24 bits
        U24(result)
    }
}

impl std::ops::Div for U24 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        assert!(rhs.0 != 0, "Division by zero is not allowed");
        let result = (self.0 / rhs.0) & 0xFFFFFF; // Division result masked to 24 bits
        U24(result)
    }
}
