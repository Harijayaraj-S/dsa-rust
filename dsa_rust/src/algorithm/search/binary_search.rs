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

// Problems solved with binary search algorithm

/*
1.First and Last Position of an Element in Sorted Array
Given a sorted array with duplicate elements, find the first and last occurrence of a target element.
Eg:
    Output:
    arr = [2, 4, 4, 4, 6, 9, 9]
    target = 4
    Output:
    (1, 3)

    Input:
    arr = [1, 2, 3, 3, 3, 3, 5, 8]
    target = 3
    Output:
    (2, 5)
*/

// We have to find two index so that we can use two step
// Step 1 find the fist
fn find_first(arr: &Vec<i64>, target: i64) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    let mut first = None;
    while start <= end {
        let middle = (start + end) / 2;
        if arr[middle] == target {
            first = Some(middle);
            if middle == 0 {
                return Some(0);
            }
            end = middle - 1;
        } else if target > arr[middle] {
            start = middle + 1;
        } else {
            end = middle - 1
        }
    }

    return first;
}

// Step 2 find the last
fn find_last(arr: &Vec<i64>, target: i64) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    let mut last = None;
    while start <= end {
        let middle = (start + end) / 2;
        if arr[middle] == target {
            last = Some(middle);
            start = middle + 1
        } else if target > arr[middle] {
            start = middle + 1;
        } else {
            if middle == 0 {
                return Some(0);
            }
            end = middle - 1
        }
    }

    return last;
}

fn first_and_last_position(arr: Vec<i64>, target: i64) -> (Option<usize>, Option<usize>) {
    let first = find_first(&arr, target);
    let last = find_last(&arr, target);

    (first, last)
}

/*2. Find the Smallest Number Greater Than or Equal to Target
Given a sorted array, find the index of the smallest number that is greater than or equal to the target.

Example:
Input: nums = [1, 3, 5, 7, 9], target = 6
Output: 3
*/

fn smallest_num_greater_than_or_equal(arr: Vec<i64>, target: i64) -> Option<usize> {
    let (mut start, mut end) = (0, arr.len() as isize - 1);
    let mut ans = None;

    while start <= end {
        let mid = start + (end - start) / 2;
        if arr[mid as usize] >= target {
            ans = Some(mid as usize);
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    ans
}

/*  Find the Largest Number Less Than or Equal to Target
Given a sorted array, find the index of the largest number that is less than or equal to the target.
Example:
Input: nums = [1, 3, 5, 7, 9], target = 6
Output: 2
*/

fn largest_num_lesser_than_or_equal(arr: Vec<i64>, target: i64) -> Option<usize> {
    let (mut start, mut end) = (0, arr.len() as isize - 1);
    let mut ans = None;

    while start <= end {
        let mid = start + (end - start) / 2;
        if arr[mid as usize] <= target {
            ans = Some(mid as usize);
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    ans
}

mod test {
    use crate::algorithm::search::binary_search::{
        algo_binary_search, first_and_last_position, largest_num_lesser_than_or_equal,
        smallest_num_greater_than_or_equal,
    };

    #[test]
    fn test_algo_binary_search() {
        assert_eq!(algo_binary_search(vec![1, 3, 5, 7, 9], 5), Some(2));
        assert_eq!(algo_binary_search(vec![1, 3, 5, 7, 9], 1), Some(0));
        assert_eq!(algo_binary_search(vec![1, 3, 5, 7, 9], 10), None);
    }

    #[test]
    fn test_first_and_last_position() {
        assert_eq!(
            first_and_last_position(vec![2, 4, 4, 4, 6, 9, 9], 4),
            (Some(1), Some(3))
        );

        assert_eq!(
            first_and_last_position(vec![1, 2, 3, 3, 3, 3, 5, 8], 3),
            (Some(2), Some(5))
        );

        assert_eq!(
            first_and_last_position(vec![1, 2, 3, 3, 3, 3, 5, 8], 11),
            (None, None)
        );

        assert_eq!(
            first_and_last_position(vec![1, 2, 3, 3, 3, 3, 5, 8], 1),
            (Some(0), Some(0))
        );
    }

    #[test]
    fn test_smallest_num_greater_than_or_equal() {
        assert_eq!(
            largest_num_lesser_than_or_equal(vec![1, 3, 5, 7, 9], 6),
            Some(2)
        );
        assert_eq!(
            largest_num_lesser_than_or_equal(vec![2, 4, 6, 8, 10], 5),
            Some(1)
        );
        assert_eq!(
            largest_num_lesser_than_or_equal(vec![10, 20, 30], 10),
            Some(0)
        );
    }
}
