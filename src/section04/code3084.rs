#![allow(dead_code)]

// 3084. Count Substrings Starting and Ending with Given Character
// https://leetcode.com/problems/count-substrings-starting-and-ending-with-given-character/
// https://leetcode.cn/problems/count-substrings-starting-and-ending-with-given-character/
//
// Medium
//
// You are given a string s and a character c. Return the total number of substrings of s that start and end with c.
//
// Example 1:
//
// Input: s = "abada", c = "a"
//
// Output: 6
//
// Explanation: Substrings starting and ending with "a" are: "abada", "abada", "abada", "abada", "abada", "abada".
//
// Example 2:
//
// Input: s = "zzz", c = "z"
//
// Output: 6
//
// Explanation: There are a total of 6 substrings in s and all start and end with "z".
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     s and c consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn count_substrings(s: String, c: char) -> i64 {
        let mut cnt = 0_i64;
        for cc in s.chars() {
            if cc.eq(&c) {
                cnt += 1;
            }
        }
        cnt * (cnt + 1) / 2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_substrings("abada".to_string(), 'a'), 6);
    assert_eq!(Solution::count_substrings("zzz".to_string(), 'z'), 6);
}
