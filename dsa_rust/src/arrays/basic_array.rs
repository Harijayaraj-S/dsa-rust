// leetcode 1920. Build Array from Permutation
struct Solution;
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let ans: Vec<i32> = nums.iter().map(|ele| nums[*ele as usize]).collect();
        ans
    }
}

// Leedcode 1929. Concatenation of Array
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

// Leedcode 1480. Running Sum of 1d Array
struct RunnungSumSolution;
impl RunnungSumSolution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut sum = 0;
        for index in 0..nums.len() {
            for sum_index in 0..index + 1 {
                sum += nums[sum_index];
            }
            ans.push(sum);
            sum = 0;
        }
        ans
    }
}

// Leedcode 1672. Richest Customer Wealth
struct MaximumWealthSolution;
impl MaximumWealthSolution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|i| i.iter().sum()).max().unwrap()
    }
}

// leedcode 1470. Shuffle the Array
struct ShuffleArraySolution;
impl ShuffleArraySolution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = Vec::new();
        for i in 0..n {
            ans.push(nums[i]);
            ans.push(nums[i + n]);
        }
        ans
    }
}

// 1431. Kids With the Greatest Number of Candies
struct GreatestNumberCandiesSolution;
impl GreatestNumberCandiesSolution{
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap();
        let mut candiesiter = candies.iter();
        let mut result:Vec<bool> = Vec::new();
        while let Some(candy) = candiesiter.next() {
            if candy + extra_candies >= *max {
                result.push(true);
            }
            else {
                result.push(false);
            }
        }
        result
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

    #[test]
    fn test_running_sum() {
        assert_eq!(
            RunnungSumSolution::running_sum([1, 2, 3, 4].to_vec()),
            [1, 3, 6, 10].to_vec()
        );

        assert_eq!(
            RunnungSumSolution::running_sum([1, 1, 1, 1, 1].to_vec()),
            [1, 2, 3, 4, 5].to_vec()
        );
    }

    #[test]
    fn test_maximum_wealth() {
        assert_eq!(
            MaximumWealthSolution::maximum_wealth(
                [[1, 2, 3].to_vec(), [3, 2, 1].to_vec()].to_vec()
            ),
            6
        );

        assert_eq!(
            MaximumWealthSolution::maximum_wealth(
                [[1, 5].to_vec(), [7, 3].to_vec(), [3, 5].to_vec()].to_vec()
            ),
            10
        );

        assert_eq!(
            MaximumWealthSolution::maximum_wealth(
                [[2, 8, 7].to_vec(), [7, 1, 3].to_vec(), [1, 9, 5].to_vec()].to_vec()
            ),
            17
        );
    }

    #[test]
    fn test_shuffle() {
        assert_eq!(
            ShuffleArraySolution::shuffle([2, 5, 1, 3, 4, 7].to_vec(), 3),
            [2, 3, 5, 4, 1, 7].to_vec()
        );

        assert_eq!(
            ShuffleArraySolution::shuffle([1, 2, 3, 4, 4, 3, 2, 1].to_vec(), 4),
            [1, 4, 2, 3, 3, 2, 4, 1].to_vec()
        );
    }
}
