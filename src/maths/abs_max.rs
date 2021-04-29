pub fn abs_max<'a, T>(mut x: T) -> i32
where
    T: Iterator<Item = &'a i32>,
{
    let mut max_element: &i32 = x.next().unwrap();
    for element in x {
        if element.abs() > max_element.abs() {
            max_element = element;
        }
    }
    *max_element
}

pub fn abs_max_sort(x: &Vec<i32>) -> i32 {
    let mut x = x.clone();
    x.sort();
    x.iter().map(|x| x.abs()).last().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::{abs_max, abs_max_sort};

    #[test]
    fn test_abs_max() {
        let data: Vec<i32> = vec![1, 2, -11];
        assert_eq!(abs_max(data.iter()), -11);
    }

    #[test]
    fn test_abs_max_sort() {
        let data: Vec<i32> = vec![192, -12, 59, 0];
        assert_eq!(abs_max_sort(&data), 192);
    }
}
