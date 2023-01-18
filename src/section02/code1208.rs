#![allow(dead_code)]

// 1208. Get Equal Substrings Within Budget
// https://leetcode.com/problems/get-equal-substrings-within-budget/
// https://leetcode.cn/problems/get-equal-substrings-within-budget/
//
// Medium
//
// You are given two strings s and t of the same length and an integer maxCost.
//
// You want to change s to t. Changing the ith character of s to ith character of t costs |s[i] - t[i]| (i.e., the absolute difference between the ASCII values of the characters).
//
// Return the maximum length of a substring of s that can be changed to be the same as the corresponding substring of t with a cost less than or equal to maxCost. If there is no substring from s that can be changed to its corresponding substring from t, return 0.
//
// Example 1:
//
// Input: s = "abcd", t = "bcdf", maxCost = 3
// Output: 3
// Explanation: "abc" of s can change to "bcd".
// That costs 3, so the maximum length is 3.
//
// Example 2:
//
// Input: s = "abcd", t = "cdef", maxCost = 3
// Output: 1
// Explanation: Each character in s costs 2 to change to character in t,  so the maximum length is 1.
//
// Example 3:
//
// Input: s = "abcd", t = "acde", maxCost = 0
// Output: 1
// Explanation: You cannot make any change, so the maximum length is 1.
//
// Constraints:
//
// -    1 <= s.length <= 10^5
// -    t.length == s.length
// -    0 <= maxCost <= 10^6
// -    s and t consist of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let mut cost = 0;
        let mut max = 0;
        let mut start = 0;
        let mut end = 0;
        let s = s.as_bytes();
        let t = t.as_bytes();
        while end < s.len() {
            cost += (s[end] as i32 - t[end] as i32).abs();
            while cost > max_cost {
                cost -= (s[start] as i32 - t[start] as i32).abs();
                start += 1;
            }
            max = std::cmp::max(max, end as i32 - start as i32 + 1);
            end += 1;
        }
        max
    }
}

#[test]
fn test() {
    let cases = vec![("abcd", "bcdf", 3, 3), ("abcd", "cdef", 3, 1), ("abcd", "acde", 0, 1)];
    for (s, t, max_cost, expected) in cases {
        let (s, t) = (s.to_string(), t.to_string());
        assert_eq!(Solution::equal_substring(s, t, max_cost), expected);
    }
}
