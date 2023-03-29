#![allow(dead_code)]

/*

// 2370. Longest Ideal Subsequence
// https://leetcode.com/problems/longest-ideal-subsequence/
// https://leetcode.cn/problems/longest-ideal-subsequence/
//
// Medium
//
// You are given a string s consisting of lowercase letters and an integer k. We call a string t ideal if the following conditions are satisfied:

    t is a subsequence of the string s.
    The absolute difference in the alphabet order of every two adjacent letters in t is less than or equal to k.

Return the length of the longest ideal string.

A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

Note that the alphabet order is not cyclic. For example, the absolute difference in the alphabet order of 'a' and 'z' is 25, not 1.

Example 1:

Input: s = "acfgbd", k = 2
Output: 4
Explanation: The longest ideal string is "acbd". The length of this string is 4, so 4 is returned.
Note that "acfgbd" is not ideal because 'c' and 'f' have a difference of 3 in alphabet order.

Example 2:

Input: s = "abcd", k = 3
Output: 4
Explanation: The longest ideal string is "abcd". The length of this string is 4, so 4 is returned.

Constraints:

    1 <= s.length <= 10^5
    0 <= k <= 25
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut count = [0; 26];
        for c in s.as_bytes() {
            let i = *c as usize - 'a' as usize;
            count[i] = (i.saturating_sub(k)..=(25.min(i + k))).map(|x| count[x]).max().unwrap() + 1;
        }
        *count.iter().max().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_ideal_string("acfgbd".to_string(), 2), 4);
    assert_eq!(Solution::longest_ideal_string("abcd".to_string(), 3), 4);
}
