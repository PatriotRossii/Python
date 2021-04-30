pub fn median(a: &mut [i32]) -> f64 {
    a.sort();

    let length = a.len();
    let middle = length / 2;
    
    if length % 2 == 0 {
        (a[middle] + a[middle - 1]) as f64 / 2_f64
    } else {
        a[middle].into()
    }
}

#[cfg(test)]
pub mod tests {
    use super::median;

    #[test]
    fn test_median() {
        let mut data: [i32; 7] = [2, 70, 6, 50, 20, 8, 4];
        assert_eq!(
            median(&mut data),
            8_f64,
        );

        let mut data: [i32; 4] = [4, 1, 3, 2];
        assert_eq!(
            median(&mut data),
            2.5_f64,
        );

        let mut data: [i32; 1] = [0];
        assert_eq!(
            median(&mut data),
            0_f64
        );
    }
}