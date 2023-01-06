#![allow(dead_code)]

// 1003. Check If Word Is Valid After Substitutions
//
// Given a string s, determine if it is valid.
//
// A string s is valid if, starting with an empty string t = "", you can transform t into s
// after performing the following operation any number of times:
//
// Insert string "abc" into any position in t. More formally, t becomes tleft + "abc" + tright,
// where t == tleft + tright. Note that tleft and tright may be empty.
// Return true if s is a valid string, otherwise, return false.
//
// Example 1:
//
// Input: s = "aabcbc"
// Output: true
// Explanation:
// "" -> "abc" -> "aabcbc"
// Thus, "aabcbc" is valid.
//
// Example 2:
//
// Input: s = "abcabcababcc"
// Output: true
// Explanation:
// "" -> "abc" -> "abcabc" -> "abcabcabc" -> "abcabcababcc"
// Thus, "abcabcababcc" is valid.
//
// Example 3:
//
// Input: s = "abccba"
// Output: false
// Explanation: It is impossible to get "abccba" using the operation.
//
// Constraints:
//
// - 1 <= s.length <= 2 * 10^4
// - s consists of letters 'a', 'b', and 'c'
//

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk = Vec::new();
        let mut idx = 0;
        while idx < s.len() {
            let ch = s.chars().nth(idx).unwrap();
            if ch == 'a' || ch == 'b' {
                stk.push(ch);
            } else if ch == 'c' && stk.len() >= 2 {
                let ch2 = stk.pop().unwrap();
                let ch1 = stk.pop().unwrap();
                if !(ch1 == 'a' && ch2 == 'b' && ch == 'c') {
                    stk.push(ch1);
                    stk.push(ch2);
                    stk.push(ch);
                }
            }
            idx += 1;
        }
        if stk.is_empty() && idx >= s.len() {
            return true;
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid("aabcbc".to_string()), true);
    assert_eq!(Solution::is_valid("abcabcababcc".to_string()), true);
    assert_eq!(Solution::is_valid("abccba".to_string()), false);
}
