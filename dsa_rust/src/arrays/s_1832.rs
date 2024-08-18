// 1832. Check if the Sentence Is Pangram

// A pangram is a sentence where every letter of the English alphabet appears at least once.

// Given a string sentence containing only lowercase English letters, return true if sentence is a pangram, or false otherwise.

// Example 1:

// Input: sentence = "thequickbrownfoxjumpsoverthelazydog"
// Output: true
// Explanation: sentence contains at least one of every letter of the English alphabet.
// Example 2:

// Input: sentence = "leetcode"
// Output: false

struct Solution;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut alpha_array = vec![0; 26];
        for letter in sentence.chars() {
            let idx = letter.to_ascii_lowercase() as i32 - 97;
            alpha_array[idx as usize] = 1;
        }
        !alpha_array.contains(&0)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
        assert!(!Solution::check_if_pangram("leetcode".to_string()));
    }
}
