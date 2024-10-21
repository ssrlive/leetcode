#![allow(dead_code)]

// 3325. Count Substrings With K-Frequency Characters I
// https://leetcode.com/problems/count-substrings-with-k-frequency-characters-i/
// https://leetcode.cn/problems/count-substrings-with-k-frequency-characters-i/
//
// Medium
//
// Given a string s and an integer k, return the total number of substrings  of s where at least one character appears at least k times.
//
// Example 1:
//
// Input: s = "abacb", k = 2
//
// Output: 4
//
// Explanation:
//
// The valid substrings are:
//
// "aba" (character 'a' appears 2 times).
// "abac" (character 'a' appears 2 times).
// "abacb" (character 'a' appears 2 times).
// "bacb" (character 'b' appears 2 times).
//
// Example 2:
//
// Input: s = "abcde", k = 1
//
// Output: 15
//
// Explanation:
//
// All substrings are valid because every character appears at least once.
//
// Constraints:
//
// 1 <= s.length <= 3000
// 1 <= k <= s.length
// s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let n = s.len() as i32;
        let mut res = (n + 1) * n / 2;
        let mut count = std::collections::HashMap::new();
        let s = s.as_bytes();
        let mut i = 0;
        for j in 0..n {
            *count.entry(s[j as usize]).or_insert(0) += 1;
            while *count.get(&s[j as usize]).unwrap_or(&0) >= k as usize {
                *count.get_mut(&s[i as usize]).unwrap() -= 1;
                i += 1;
            }
            res -= j - i + 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_substrings("abacb".to_string(), 2), 4);
    assert_eq!(Solution::number_of_substrings("abcde".to_string(), 1), 15);
}
