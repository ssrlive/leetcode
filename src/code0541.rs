#![allow(dead_code)]

// 541. Reverse String II
// https://leetcode.com/problems/reverse-string-ii/
// https://leetcode.cn/problems/reverse-string-ii/
//
// Given a string s and an integer k, reverse the first k characters for every 2k characters counting from the start of the string.
//
// If there are fewer than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and leave the other as original.
//
// Example 1:
//
// Input: s = "abcdefg", k = 2
// Output: "bacdfeg"
//
// Example 2:
//
// Input: s = "abcd", k = 2
// Output: "bacd"
//
// Constraints:
//
// - 1 <= s.length <= 10^4
// - s consists of only lowercase English letters.
// - 1 <= k <= 10^4
//

struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let k = k as usize;
        for i in (0..s.len()).step_by(2 * k) {
            let j = i + k - 1;
            let j = if j < s.len() { j } else { s.len() - 1 };
            for k in 0..(j - i + 1) / 2 {
                s.swap(i + k, j - k);
            }
        }
        String::from_utf8(s).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg".to_string());
    assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd".to_string());
}
