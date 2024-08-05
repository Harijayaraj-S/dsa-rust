// leetcode 1920. Build Array from Permutation
struct Solution;
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let ans: Vec<i32> = nums.iter().map(|ele| nums[*ele as usize]).collect();
        ans
    }
}

// 1929. Concatenation of Array
struct ConcatenationSolution;
impl ConcatenationSolution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for index in 0..(nums.len() * 2) {
            if index < nums.len() {
                ans.push(nums[index])
            } else {
                ans.push(nums[index - nums.len()])
            }
        }
        ans
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_build_array() {
        assert_eq!(
            Solution::build_array([0, 2, 1, 5, 3, 4].to_vec()),
            [0, 1, 2, 4, 5, 3].to_vec()
        );

        assert_eq!(
            Solution::build_array([5, 0, 1, 2, 3, 4].to_vec()),
            [4, 5, 0, 1, 2, 3].to_vec()
        );
    }

    #[test]
    fn test_get_concatenation() {
        assert_eq!(
            ConcatenationSolution::get_concatenation([1, 2, 1].to_vec()),
            [1, 2, 1, 1, 2, 1].to_vec()
        );

        assert_eq!(
            ConcatenationSolution::get_concatenation([1, 3, 2, 1].to_vec()),
            [1, 3, 2, 1, 1, 3, 2, 1].to_vec()
        );
    }
}
