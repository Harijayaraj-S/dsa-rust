// 1512. Number of Good Pairs

// Given an array of integers nums, return the number of good pairs.

// A pair (i, j) is called good if nums[i] == nums[j] and i < j.

// Example 1:

// Input: nums = [1,2,3,1,1,3]
// Output: 4
// Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
// Example 2:

// Input: nums = [1,1,1,1]
// Output: 6
// Explanation: Each pair in the array are good.
// Example 3:

// Input: nums = [1,2,3]
// Output: 0

struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut pairs = 0;
        let len = nums.len();
        for i in 0..len {
            for j in i..len {
                if i < j && nums[i] == nums[j] {
                    pairs += 1;
                }
            }
        }
        return pairs;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(
            Solution::num_identical_pairs([1, 2, 3, 1, 1, 3].to_vec()),
            4
        );

        assert_eq!(Solution::num_identical_pairs([1, 1, 1, 1].to_vec()), 6);
    }
}
