#![allow(dead_code)]

// 848. Shifting Letters
// https://leetcode.com/problems/shifting-letters/
// https://leetcode.cn/problems/shifting-letters/
//
// You are given a string s of lowercase English letters and an integer array shifts of the same length.
//
// Call the shift() of a letter, the next letter in the alphabet, (wrapping around so that 'z' becomes 'a').
//
// For example, shift('a') = 'b', shift('t') = 'u', and shift('z') = 'a'.
// Now for each shifts[i] = x, we want to shift the first i + 1 letters of s, x times.
//
// Return the final string after all such shifts to s are applied.
//
//
//
// Example 1:
//
// Input: s = "abc", shifts = [3,5,9]
// Output: "rpl"
// Explanation: We start with "abc".
// After shifting the first 1 letters of s by 3, we have "dbc".
// After shifting the first 2 letters of s by 5, we have "igc".
// After shifting the first 3 letters of s by 9, we have "rpl", the answer.
//
// Example 2:
//
// Input: s = "aaa", shifts = [1,2,3]
// Output: "gfd"
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of lowercase English letters.
// - shifts.length == s.length
// - 0 <= shifts[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut shifts = shifts;
        let mut sum = 0;
        for i in (0..shifts.len()).rev() {
            sum = (sum + shifts[i]) % 26;
            shifts[i] = sum;
        }
        let mut s = s.into_bytes();
        for i in 0..s.len() {
            s[i] = (s[i] - b'a' + shifts[i] as u8) % 26 + b'a';
        }
        String::from_utf8(s).unwrap_or_default()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("abc".to_string(), vec![3, 5, 9], "rpl".to_string()),
        ("aaa".to_string(), vec![1, 2, 3], "gfd".to_string()),
    ];
    for (s, shifts, expected) in cases {
        assert_eq!(Solution::shifting_letters(s, shifts), expected);
    }
}
