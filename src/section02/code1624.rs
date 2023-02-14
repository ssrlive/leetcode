#![allow(dead_code)]

/*

// 1624. Largest Substring Between Two Equal Characters
// https://leetcode.com/problems/largest-substring-between-two-equal-characters/
// https://leetcode.cn/problems/largest-substring-between-two-equal-characters/
//
// Easy
//
// Given a string s, return the length of the longest substring between two equal characters, excluding the two characters. If there is no such substring return -1.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "aa"
Output: 0
Explanation: The optimal substring here is an empty substring between the two 'a's.

Example 2:

Input: s = "abca"
Output: 2
Explanation: The optimal substring here is "bc".

Example 3:

Input: s = "cbzxy"
Output: -1
Explanation: There are no characters that appear twice in s.

Constraints:

    1 <= s.length <= 300
    s contains only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max = -1;
        let mut last = [None; 26];
        for (i, c) in s.chars().enumerate() {
            let c = c as usize - 'a' as usize;
            if let Some(j) = last[c] {
                max = max.max((i - j - 1) as i32);
            } else {
                last[c] = Some(i);
            }
        }
        max
    }
}

#[test]
fn test() {
    let cases = vec![
        ("aa", 0),
        ("abca", 2),
        ("cbzxy", -1),
        ("cabbac", 4),
        ("mgntdygtxrvxjnwksqhxuxtrv", 18),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::max_length_between_equal_characters(s.to_string()), expected);
    }
}
