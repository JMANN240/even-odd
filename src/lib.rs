/// A trait for providing a type with the concept of even-ness.
/// Implementations are provided for primitive numeric types.
pub trait IsEven {
    /// Returns true if self is even.
    /// ```
    /// use even_odd::IsEven;
    ///
    /// assert!(0.0_f32.is_even());
    /// assert!(!1.0_f32.is_even());
    /// assert!(2.0_f32.is_even());
    /// assert!(!3.0_f32.is_even());
    /// ```
    fn is_even(&self) -> bool;
}

/// A trait for providing a type with the concept of odd-ness.
/// Implementations are provided for primitive numeric types.
pub trait IsOdd {
    /// Returns true if self is odd.
    /// ```
    /// use even_odd::IsOdd;
    ///
    /// assert!(!0.0_f32.is_odd());
    /// assert!(1.0_f32.is_odd());
    /// assert!(!2.0_f32.is_odd());
    /// assert!(3.0_f32.is_odd());
    /// ```
    fn is_odd(&self) -> bool;
}

macro_rules! integer_implementation {
    ($($t:tt)*) => {
        $(
            impl IsOdd for $t {
                fn is_odd(&self) -> bool {
                    self & 1 != 0
                }
            }

            impl IsEven for $t {
                fn is_even(&self) -> bool {
                    self & 1 == 0
                }
            }
        )*
    };
}

integer_implementation!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);

macro_rules! floating_point_implementation {
    ($($t:tt)*) => {
        $(
            impl IsEven for $t {
                fn is_even(&self) -> bool {
                    if self.is_finite() && self.fract() == 0.0 {
                        self % 2.0 == 0.0
                    } else {
                        false
                    }
                }
            }

            impl IsOdd for $t {
                fn is_odd(&self) -> bool {
                    if self.is_finite() && self.fract() == 0.0 {
                        self % 2.0 == 1.0
                    } else {
                        false
                    }
                }
            }
        )*
    };
}

floating_point_implementation!(f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(0.is_even());
        assert!(!0.is_odd());

        assert!(!1.is_even());
        assert!(1.is_odd());

        assert!(2.is_even());
        assert!(!2.is_odd());

        assert!(!3.is_even());
        assert!(3.is_odd());

        assert!(0.0.is_even());
        assert!(!0.0.is_odd());

        assert!(!1.0.is_even());
        assert!(1.0.is_odd());

        assert!(2.0.is_even());
        assert!(!2.0.is_odd());

        assert!(!3.0.is_even());
        assert!(3.0.is_odd());
    }
}
