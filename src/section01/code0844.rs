#![allow(dead_code)]

// 844. Backspace String Compare
// https://leetcode.com/problems/backspace-string-compare/
// https://leetcode.cn/problems/backspace-string-compare/
//
// Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a backspace character.
//
// Note that after backspacing an empty text, the text will continue empty.
//
// Example 1:
//
// Input: s = "ab#c", t = "ad#c"
// Output: true
// Explanation: Both s and t become "ac".
//
// Example 2:
//
// Input: s = "ab##", t = "c#d#"
// Output: true
// Explanation: Both s and t become "".
//
// Example 3:
//
// Input: s = "a#c", t = "b"
// Output: false
// Explanation: s becomes "c" while t becomes "b".
//
// Constraints:
//
// - 1 <= s.length, t.length <= 200
// - s and t only contain lowercase letters and '#' characters.
//

struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut src = String::new();
        let mut tgt = String::new();
        for i in s.chars() {
            if i == '#' {
                if !src.is_empty() {
                    src.pop();
                }
            } else {
                src.push(i);
            }
        }
        for j in t.chars() {
            if j == '#' {
                if !tgt.is_empty() {
                    tgt.pop();
                }
            } else {
                tgt.push(j);
            }
        }
        src == tgt
    }
}

#[test]
fn test() {
    assert_eq!(Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()), true);
    assert_eq!(Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()), true);
    assert_eq!(Solution::backspace_compare("a#c".to_string(), "b".to_string()), false);
}
