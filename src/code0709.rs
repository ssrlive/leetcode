#![allow(dead_code)]

// 709. To Lower Case
// https://leetcode.com/problems/to-lower-case/
// https://leetcode.cn/problems/to-lower-case/
//
// Given a string s, return the string after replacing every uppercase letter with the same lowercase letter.
//
// Example 1:
//
// Input: s = "Hello"
// Output: "hello"
//
// Example 2:
//
// Input: s = "here"
// Output: "here"
//
// Example 3:
//
// Input: s = "LOVELY"
// Output: "lovely"
//
// Constraints:
//
// - 1 <= s.length <= 100
// - s consists of printable ASCII characters.
//

struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars().map(|c| c.to_ascii_lowercase()).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello");
    assert_eq!(Solution::to_lower_case("here".to_string()), "here");
    assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely");
}
