#![allow(dead_code)]

// 3210. Find the Encrypted String
// https://leetcode.com/problems/find-the-encrypted-string/
// https://leetcode.cn/problems/find-the-encrypted-string/
//
// Easy
//
// You are given a string s and an integer k. Encrypt the string using the following algorithm:
//
//     For each character c in s, replace c with the kth character after c in the string (in a cyclic manner).
//
// Return the encrypted string.
//
// Example 1:
//
// Input: s = "dart", k = 3
//
// Output: "tdar"
//
// Explanation:
//
//     For i = 0, the 3rd character after 'd' is 't'.
//     For i = 1, the 3rd character after 'a' is 'd'.
//     For i = 2, the 3rd character after 'r' is 'a'.
//     For i = 3, the 3rd character after 't' is 'r'.
//
// Example 2:
//
// Input: s = "aaa", k = 1
//
// Output: "aaa"
//
// Explanation:
//
// As all the characters are the same, the encrypted string will also be the same.
//
// Constraints:
//
//     1 <= s.length <= 100
//     1 <= k <= 10^4
//     s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut res: String = String::new();
        let s_chars: Vec<char> = s.chars().collect::<Vec<char>>();

        for i in 0..s_chars.len() {
            res.push(s_chars[(i + k as usize) % s_chars.len()]);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_encrypted_string("dart".to_string(), 3), "tdar".to_string());
    assert_eq!(Solution::get_encrypted_string("aaa".to_string(), 1), "aaa".to_string());
}
