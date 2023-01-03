#![allow(dead_code)]

// 925. Long Pressed Name
// https://leetcode.com/problems/long-pressed-name/
// https://leetcode.cn/problems/long-pressed-name/
//
// Your friend is typing his name into a keyboard. Sometimes, when typing a character c,
// the key might get long pressed, and the character will be typed 1 or more times.
//
// You examine the typed characters of the keyboard. Return True if it is possible that it was your friends name,
// with some characters (possibly none) being long pressed.
//
// Example 1:
//
// Input: name = "alex", typed = "aaleex"
// Output: true
// Explanation: 'a' and 'e' in 'alex' were long pressed.
//
// Example 2:
//
// Input: name = "saeed", typed = "ssaaedd"
// Output: false
// Explanation: 'e' must have been pressed twice, but it was not in the typed output.
//
// Constraints:
//
// - 1 <= name.length, typed.length <= 1000
// - name and typed consist of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let nb = name.as_bytes();
        let mut i = 0;
        for &b in typed.as_bytes() {
            if i < name.len() && b == nb[i] {
                i += 1;
            } else if b != nb[i.saturating_sub(1)] {
                return false;
            }
        }
        i == name.len()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()),
        true
    );
    assert_eq!(
        Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()),
        false
    );
    assert_eq!(
        Solution::is_long_pressed_name("leelee".to_string(), "lleeelee".to_string()),
        true
    );
    assert_eq!(
        Solution::is_long_pressed_name("laiden".to_string(), "laiden".to_string()),
        true
    );
}
