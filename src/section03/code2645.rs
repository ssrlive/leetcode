#![allow(dead_code)]

/*
// 2645. Minimum Additions to Make Valid String
// https://leetcode.com/problems/minimum-additions-to-make-valid-string/
// https://leetcode.cn/problems/minimum-additions-to-make-valid-string/
//
// Medium
//
// Given a string word to which you can insert letters "a", "b" or "c" anywhere and any number of times, return the minimum number of letters that must be inserted so that word becomes valid.

A string is called valid if it can be formed by concatenating the string "abc" several times.

Example 1:

Input: word = "b"
Output: 2
Explanation: Insert the letter "a" right before "b", and the letter "c" right next to "a" to obtain the valid string "abc".
Example 2:

Input: word = "aaa"
Output: 6
Explanation: Insert letters "b" and "c" next to each "a" to obtain the valid string "abcabcabc".
Example 3:

Input: word = "abc"
Output: 0
Explanation: word is already valid. No modifications are needed.

Constraints:

1 <= word.length <= 50
word consists of letters "a", "b" and "c" only.
*/

struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let words: Vec<char> = word.chars().collect();
        let mut dp = vec![2; words.len()];
        for i in 1..words.len() {
            if words[i] > words[i - 1] {
                dp[i] = dp[i - 1] - 1;
            } else {
                dp[i] = dp[i - 1] + 2;
            }
        }
        dp[dp.len() - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::add_minimum("b".to_string()), 2);
    assert_eq!(Solution::add_minimum("aaa".to_string()), 6);
    assert_eq!(Solution::add_minimum("abc".to_string()), 0);
}
