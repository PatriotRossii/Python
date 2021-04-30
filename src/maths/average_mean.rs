use std::iter::Sum;

pub fn average<'a, T>(a: &'a [T]) -> f64
where
    T: Into<f64> + Sum<&'a T>,
{
    <T as Into<f64>>::into(a.iter().sum::<T>()) / (a.iter().len() as f64)
}

#[cfg(test)]
pub mod tests {
    use super::average;

    #[test]
    fn test_average() {
        let data: Vec<i32> = vec![3, 2];

        assert_eq!(average(&data), 2.5_f64,);

        let data: Vec<i32> = vec![2, 5, 81, 12];

        assert_eq!(average(&data), 25_f64);
    }
}
