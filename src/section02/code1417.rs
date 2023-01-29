#![allow(dead_code)]

// 1417. Reformat The String
// https://leetcode.com/problems/reformat-the-string/
// https://leetcode.cn/problems/reformat-the-string/
//
// Easy
//
// You are given an alphanumeric string s. (Alphanumeric string is a string consisting of lowercase English letters and digits).
//
// You have to find a permutation of the string where no letter is followed by another letter and no digit is followed by another digit.
// That is, no two adjacent characters have the same type.
//
// Return the reformatted string or return an empty string if it is impossible to reformat the string.
//
// Example 1:
//
// Input: s = "a0b1c2"
// Output: "0a1b2c"
// Explanation: No two adjacent characters have the same type in "0a1b2c". "a0b1c2", "0a1b2c", "0c2a1b" are also valid permutations.
//
// Example 2:
//
// Input: s = "leetcode"
// Output: ""
// Explanation: "leetcode" has only characters so we cannot separate them by digits.
//
// Example 3:
//
// Input: s = "1229857369"
// Output: ""
// Explanation: "1229857369" has only digits so we cannot separate them by characters.
//
// Constraints:
//
// -    1 <= s.length <= 500
// -    s consists of only lowercase English letters and/or digits.
//

struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut chars = Vec::new();
        let mut digits = Vec::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else {
                chars.push(c);
            }
        }
        if (chars.len() as i32 - digits.len() as i32).abs() > 1 {
            return "".to_string();
        }
        let mut ans = Vec::new();
        if chars.len() > digits.len() {
            for i in 0..digits.len() {
                ans.push(chars[i]);
                ans.push(digits[i]);
            }
            ans.push(chars[chars.len() - 1]);
        } else {
            for i in 0..chars.len() {
                ans.push(digits[i]);
                ans.push(chars[i]);
            }
            if digits.len() > chars.len() {
                ans.push(digits[digits.len() - 1]);
            }
        }
        ans.iter().map(|v| v.to_string()).collect::<String>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reformat("a0b1c2".to_string()), "0a1b2c".to_string());
    assert_eq!(Solution::reformat("leetcode".to_string()), "".to_string());
    assert_eq!(Solution::reformat("1229857369".to_string()), "".to_string());
    assert_eq!(Solution::reformat("covid2019".to_string()), "c2o0v1i9d".to_string());
    assert_eq!(Solution::reformat("ab123".to_string()), "1a2b3".to_string());
}
