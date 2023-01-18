#![allow(dead_code)]

// 1221. Split a String in Balanced Strings
// https://leetcode.com/problems/split-a-string-in-balanced-strings/
// https://leetcode.cn/problems/split-a-string-in-balanced-strings/
//
// Easy
//
// Balanced strings are those that have an equal quantity of 'L' and 'R' characters.
//
// Given a balanced string s, split it into some number of substrings such that:
//
//     Each substring is balanced.
//
// Return the maximum number of balanced strings you can obtain.
//
// Example 1:
//
// Input: s = "RLRRLLRLRL"
// Output: 4
// Explanation: s can be split into "RL", "RRLL", "RL", "RL", each substring contains same number of 'L' and 'R'.
//
// Example 2:
//
// Input: s = "RLRRRLLRLL"
// Output: 2
// Explanation: s can be split into "RL", "RRRLLRLL", each substring contains same number of 'L' and 'R'.
// Note that s cannot be split into "RL", "RR", "RL", "LR", "LL", because the 2nd and 5th substrings are not balanced.
//
// Example 3:
//
// Input: s = "LLLLRRRR"
// Output: 1
// Explanation: s can be split into "LLLLRRRR".
//
// Constraints:
//
// -    2 <= s.length <= 1000
// -    s[i] is either 'L' or 'R'.
// -    s is a balanced string.
//

struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        let mut balance = 0;
        for c in s.chars() {
            if c == 'L' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance == 0 {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![("RLRRLLRLRL", 4), ("RLRRRLLRLL", 2), ("LLLLRRRR", 1)];
    for (s, e) in cases {
        let s = s.to_string();
        assert_eq!(Solution::balanced_string_split(s), e);
    }
}
