#![allow(dead_code)]

// 1433. Check If a String Can Break Another String
// https://leetcode.com/problems/check-if-a-string-can-break-another-string/
// https://leetcode.cn/problems/check-if-a-string-can-break-another-string/
//
// Medium
//
// Given two strings: s1 and s2 with the same size, check if some permutation of string s1 can break
// some permutation of string s2 or vice-versa. In other words s2 can break s1 or vice-versa.
//
// A string x can break string y (both of size n) if x[i] >= y[i] (in alphabetical order) for all i between 0 and n-1.
//
// Example 1:
//
// Input: s1 = "abc", s2 = "xya"
// Output: true
// Explanation: "ayx" is a permutation of s2="xya" which can break to string "abc" which is a permutation of s1="abc".
//
// Example 2:
//
// Input: s1 = "abe", s2 = "acd"
// Output: false
// Explanation: All permutations for s1="abe" are: "abe", "aeb", "bae", "bea", "eab" and "eba" and all permutation
// for s2="acd" are: "acd", "adc", "cad", "cda", "dac" and "dca". However, there is not any permutation
// from s1 which can break some permutation from s2 and vice-versa.
//
// Example 3:
//
// Input: s1 = "leetcodee", s2 = "interview"
// Output: true
//
// Constraints:
//
// -    s1.length == n
// -    s2.length == n
// -    1 <= n <= 10^5
// -    All strings consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();
        s1.sort();
        s2.sort();
        let mut s1_bigger = true;
        let mut s2_bigger = true;
        for i in 0..s1.len() {
            if s1[i] > s2[i] {
                s2_bigger = false;
            }
            if s1[i] < s2[i] {
                s1_bigger = false;
            }
        }
        s1_bigger || s2_bigger
    }
}

#[test]
fn test() {
    let cases = vec![("abc", "xya", true), ("abe", "acd", false), ("leetcodee", "interview", true)];
    for (s1, s2, expected) in cases {
        assert_eq!(Solution::check_if_can_break(s1.to_string(), s2.to_string()), expected);
    }
}
