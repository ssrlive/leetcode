#![allow(dead_code)]

// 696. Count Binary Substrings
// https://leetcode.com/problems/count-binary-substrings/
//
// Given a binary string s, return the number of non-empty substrings that have the same number of 0's and 1's, and all the 0's and all the 1's in these substrings are grouped consecutively.
//
// Substrings that occur multiple times are counted the number of times they occur.
//
// Example 1:
//
// Input: s = "00110011"
// Output: 6
// Explanation: There are 6 substrings that have equal number of consecutive 1's and 0's: "0011", "01", "1100", "10", "0011", and "01".
// Notice that some of these substrings repeat and are counted the number of times they occur.
// Also, "00110011" is not a valid substring because all the 0's (and 1's) are not grouped together.
//
// Example 2:
//
// Input: s = "10101"
// Output: 4
// Explanation: There are 4 substrings: "10", "01", "10", "01" that have equal number of consecutive 1's and 0's.
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s[i] is either '0' or '1'.
//

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        Self::_count_binary_substrings(s).unwrap_or(-1)
    }
    pub fn _count_binary_substrings(s: String) -> Option<i32> {
        let mut count = 0;
        let mut prev = 0;
        let mut curr = 1;
        let mut chars = s.chars();
        let mut prev_char = chars.next()?;
        for c in chars {
            if c == prev_char {
                curr += 1;
            } else {
                count += prev.min(curr);
                prev = curr;
                curr = 1;
                prev_char = c;
            }
        }
        Some(count + prev.min(curr))
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
    assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
}
