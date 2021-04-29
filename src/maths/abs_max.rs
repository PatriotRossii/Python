pub fn abs_max<'a, T>(mut x: T) -> Option<i32>
where
    T: Iterator<Item = &'a i32>,
{
    if let Some(mut max_element) = x.next() {
        for element in x {
            if element.abs() > max_element.abs() {
                max_element = element;
            }
        }
        Some(*max_element)
    } else {
        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::abs_max;

    #[test]
    fn test_abs_max() {
        let data: Vec<i32> = vec![1, 2, -11];
        let empty_vec = vec![];

        assert_eq!(abs_max(data.iter()), Some(-11));
        assert_eq!(abs_max(empty_vec.iter()), None);
    }
}
