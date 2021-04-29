pub fn abs_min<'a, T>(mut x: T) -> Option<i32>
where
    T: Iterator<Item = &'a i32>,
{
    if let Some(mut min_element) = x.next() {
        for element in x {
            if element.abs() > min_element.abs() {
                min_element = element;
            }
        }
        Some(*min_element)
    } else {
        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::abs_min;

    #[test]
    fn test_abs_min() {
        let data: Vec<i32> = vec![1, 2, -11];
        let empty_vec = vec![];

        assert_eq!(abs_min(data.iter()), Some(-11));
        assert_eq!(abs_min(empty_vec.iter()), None);
    }
}
