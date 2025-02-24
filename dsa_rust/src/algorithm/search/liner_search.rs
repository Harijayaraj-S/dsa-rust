// Liner search algorithm

fn algo_liner_search(arr: Vec<i64>, target: i64) -> Option<usize> {
    // Iterate over each element until find the index of target element

    for index in 0..arr.len() {
        // If element is identified return the index
        if arr[index] == target {
            return Some(index);
        }
    }

    // else return none
    return None;
}

mod test {
    use crate::algorithm::search::liner_search::algo_liner_search;

    #[test]
    fn test_algo_liner_search() {
        assert_eq!(algo_liner_search(vec![1, 3, 5, 7, 9], 5), Some(2));
        assert_eq!(algo_liner_search(vec![1, 3, 5, 7, 9], 10), None);
    }
}
