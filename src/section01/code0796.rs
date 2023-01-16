#![allow(dead_code)]

// 796. Rotate String
// https://leetcode.com/problems/rotate-string/
// https://leetcode.cn/problems/rotate-string/
//
// Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.
//
// A shift on s consists of moving the leftmost character of s to the rightmost position.
//
// For example, if s = "abcde", then it will be "bcdea" after one shift.
//
// Example 1:
//
// Input: s = "abcde", goal = "cdeab"
// Output: true
//
// Example 2:
//
// Input: s = "abcde", goal = "abced"
// Output: false
//
// Constraints:
//
// - 1 <= s.length, goal.length <= 100
// - s and goal consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let len = s.len();
        if len != goal.len() {
            return false;
        }
        (1..len).any(|i| s[i..] == goal[..len - i] && s[..i] == goal[len - i..])
    }

    pub fn rotate_string2(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        s.repeat(2).contains(&goal)
    }

    pub fn rotate_string3(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let s = s.as_bytes();
        let goal = goal.as_bytes();

        let len = s.len();

        for i in 0..len {
            if s[i] == goal[0] {
                let mut j = 0;
                while j < len && s[(i + j) % len] == goal[j] {
                    j += 1;
                }
                if j == len {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::rotate_string("abcde".to_string(), "cdeab".to_string()), true);
    assert_eq!(Solution::rotate_string("abcde".to_string(), "abced".to_string()), false);
}
