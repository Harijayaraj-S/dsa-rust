// Given an n x n binary matrix image, flip the image horizontally, then invert it, and return the resulting image.

// To flip an image horizontally means that each row of the image is reversed.

// For example, flipping [1,1,0] horizontally results in [0,1,1].
// To invert an image means that each 0 is replaced by 1, and each 1 is replaced by 0.

// For example, inverting [0,1,1] results in [1,0,0].

// Example 1:

// Input: image = [[1,1,0],[1,0,1],[0,0,0]]
// Output: [[1,0,0],[0,1,0],[1,1,1]]
// Explanation: First reverse each row: [[0,1,1],[1,0,1],[0,0,0]].
// Then, invert the image: [[1,0,0],[0,1,0],[1,1,1]]
// Example 2:

// Input: image = [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
// Output: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
// Explanation: First reverse each row: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]].
// Then invert the image: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
struct Solution;

impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn reverse_and_invert(mut arr: Vec<i32>) -> Vec<i32> {
            arr.reverse();
            for i in 0..arr.len() {
                if arr[i] == 0 {
                    arr[i] = 1;
                } else {
                    arr[i] = 0;
                }
            }
            arr
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        for img in image {
            ans.push(reverse_and_invert(img))
        }
        ans
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(
            Solution::flip_and_invert_image(
                [[1, 1, 0].to_vec(), [1, 0, 1].to_vec(), [0, 0, 0].to_vec()].to_vec()
            ),
            [vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]].to_vec()
        );
    }
}
