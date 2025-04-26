pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return vec![];
    }

    let mut result = vec![1; arr.len()];

    let mut left_product = 1;
    for i in 0..arr.len() {
        result[i] *= left_product;
        left_product *= arr[i];
    }

    let mut right_product = 1;
    for i in (0..arr.len()).rev() {
        result[i] *= right_product;
        right_product *= arr[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_products() {
        let arr = vec![1, 7, 3, 4];
        let result = get_products(arr);
        assert_eq!(result, vec![84, 12, 28, 21]);

        let arr = vec![2, 3, 4];
        let result = get_products(arr);
        assert_eq!(result, vec![12, 8, 6]);

        let arr = vec![5];
        let result = get_products(arr);
        assert_eq!(result, vec![1]);
    }
    #[test]
    fn test_empty_array() {
        let arr: Vec<usize> = vec![];
        let result = get_products(arr);
        assert_eq!(result, vec![]);
    }
    #[test]
    fn test_single_element_array() {
        let arr = vec![10];
        let result = get_products(arr);
        assert_eq!(result, vec![1]);
    }
}