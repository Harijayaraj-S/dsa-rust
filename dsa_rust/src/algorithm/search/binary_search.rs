// Binary search

// Algorithm
fn algo_binary_search(arr: Vec<i64>, target: i64) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;

    while start <= end {
        let middle = (start + end) / 2;
        if arr[middle] == target {
            return Some(middle);
        } else if middle as i64 > target {
            end = middle - 1;
        } else {
            start = middle + 1
        }
    }

    None
}

mod test {
    use crate::algorithm::search::binary_search::algo_binary_search;

    #[test]
    fn test_algo_liner_search() {
        assert_eq!(algo_binary_search(vec![1, 3, 5, 7, 9], 5), Some(2));
        assert_eq!(algo_binary_search(vec![1, 3, 5, 7, 9], 1), Some(0));
        assert_eq!(algo_binary_search(vec![1, 3, 5, 7, 9], 10), None);
    }
}
