#![allow(dead_code)]

// 58. Length of Last Word
// https://leetcode.com/problems/length-of-last-word/
// https://leetcode.cn/problems/length-of-last-word/
//
// Given a string s consisting of words and spaces, return the length of the last word in the string.
//
// A word is a maximal substring consisting of non-space characters only.
//
// Example 1:
//
// Input: s = "Hello World"
// Output: 5
// Explanation: The last word is "World" with length 5.
//
// Example 2:
//
// Input: s = "   fly me   to   the moon  "
// Output: 4
// Explanation: The last word is "moon" with length 4.
//
// Example 3:
//
// Input: s = "luffy is still joyboy"
// Output: 6
// Explanation: The last word is "joyboy" with length 6.
//
// Constraints:
//
// 1 <= s.length <= 104
// s consists of only English letters and spaces ' '.
// There will be at least one word in s.
//

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().chars().rev().take_while(|c| c.is_alphabetic()).count() as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        ("Hello World", 5),
        ("   fly me   to   the moon  ", 4),
        ("luffy is still joyboy", 6),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::length_of_last_word(s.to_string()), expected);
    }
}
