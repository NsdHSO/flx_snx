/// Add two integers together.
///
/// # Invariant
/// The result fits within `i32` (no overflow handling; caller must ensure inputs are safe).
#[inline]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_basic() {
        assert_eq!(sum(1, 2), 3);
    }

    #[test]
    fn sum_zero() {
        assert_eq!(sum(0, 0), 0);
    }

    #[test]
    fn sum_negative() {
        assert_eq!(sum(-5, 5), 0);
    }
}
