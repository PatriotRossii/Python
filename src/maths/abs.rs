pub fn abs(val: i32) -> i32 {
    if val >= 0 {
        val
    } else {
        -val
    }
}

#[cfg(test)]
pub mod tests {
    use super::abs;

    #[test]
    fn abs_test() {
        assert_eq!(abs(-120), 120);
        assert_eq!(abs(0), 0);
        assert_eq!(abs(5), 5);
    }
}