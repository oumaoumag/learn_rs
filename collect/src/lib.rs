pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Sorts an array of integers in ascending order using the bubble sort algorithm
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    
    // Early return for empty or single-element arrays
    if n <= 1 {
        return;
    }
    
    // Flag to track if any swaps were made in a pass
    let mut swapped;
    
    // Outer loop for passes
    for i in 0..n {
        swapped = false;
        
        // Inner loop for comparisons in each pass
        // After each pass, the largest element bubbles to the end,
        // so we can reduce the number of comparisons
        for j in 0..(n - i - 1) {
            // If current element is greater than the next element, swap them
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // If no swaps were made in this pass, the array is already sorted
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_bubble_sort() {
        // Test case 1: Random array
        let mut arr1 = [3, 2, 4, 5, 1, 7];
        bubble_sort(&mut arr1);
        assert_eq!(arr1, [1, 2, 3, 4, 5, 7]);
        
        // Test case 2: Already sorted array
        let mut arr2 = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);
        
        // Test case 3: Reverse sorted array
        let mut arr3 = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr3);
        assert_eq!(arr3, [1, 2, 3, 4, 5]);
        
        // Test case 4: Array with duplicates
        let mut arr4 = [3, 1, 4, 1, 5, 9, 2, 6, 5];
        bubble_sort(&mut arr4);
        assert_eq!(arr4, [1, 1, 2, 3, 4, 5, 5, 6, 9]);
        
        // Test case 5: Empty array
        let mut arr5: [i32; 0] = [];
        bubble_sort(&mut arr5);
        assert_eq!(arr5, []);
        
        // Test case 6: Single element array
        let mut arr6 = [42];
        bubble_sort(&mut arr6);
        assert_eq!(arr6, [42]);
    }
}
