#![allow(dead_code)]

// 3090. Maximum Length Substring With Two Occurrences
// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/
// https://leetcode.cn/problems/maximum-length-substring-with-two-occurrences/
//
// Easy
//
// Given a string s, return the maximum length of a substring such that it contains at most two occurrences of each character.
//
// Example 1:
//
// Input: s = "bcbbbcba"
//
// Output: 4
//
// Explanation:
// The following substring has a length of 4 and contains at most two occurrences of each character: "bcbbbcba".
//
// Example 2:
//
// Input: s = "aaaa"
//
// Output: 2
//
// Explanation:
// The following substring has a length of 2 and contains at most two occurrences of each character: "aaaa".
//
// Constraints:
//
//     2 <= s.length <= 100
//     s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let (mut cnt, mut i, mut best) = (std::collections::HashMap::new(), 0, 0);
        for j in 0..s.len() {
            *cnt.entry(&s[j]).or_insert(0) += 1;
            while 2 < *cnt.entry(&s[j]).or_insert(0) {
                *cnt.entry(&s[i]).or_insert(0) -= 1;
                i += 1;
            }
            best = best.max(j - i + 1);
        }
        best as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_length_substring("bcbbbcba".to_string()), 4);
    assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
}
