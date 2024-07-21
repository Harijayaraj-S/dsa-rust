
// leetcode 1920. Build Array from Permutation
struct Solution;
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let ans: Vec<i32> = nums.iter().map(|ele| nums[*ele as usize]).collect();
        ans
    }
}

mod tests {
    use super::*;

    #[test]
    fn build_array() {
        assert_eq!(
            Solution::build_array([0, 2, 1, 5, 3, 4].to_vec()),
            [0, 1, 2, 4, 5, 3].to_vec()
        );

        assert_eq!(
            Solution::build_array([5, 0, 1, 2, 3, 4].to_vec()),
            [4, 5, 0, 1, 2, 3].to_vec()
        );
    }
}
