#![allow(dead_code)]

/*

// 2124. Check if All A's Appears Before All B's
// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/
// https://leetcode.cn/problems/check-if-all-as-appears-before-all-bs/
//
// Easy
//
// Given a string s consisting of only the characters 'a' and 'b', return true if every 'a' appears before every 'b' in the string. Otherwise, return false.

Example 1:

Input: s = "aaabbb"
Output: true
Explanation:
The 'a's are at indices 0, 1, and 2, while the 'b's are at indices 3, 4, and 5.
Hence, every 'a' appears before every 'b' and we return true.

Example 2:

Input: s = "abab"
Output: false
Explanation:
There is an 'a' at index 2 and a 'b' at index 1.
Hence, not every 'a' appears before every 'b' and we return false.

Example 3:

Input: s = "bbb"
Output: true
Explanation:
There are no 'a's, hence, every 'a' appears before every 'b' and we return true.

Constraints:

    1 <= s.length <= 100
    s[i] is either 'a' or 'b'.
*/

struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let (mut ind_a, mut ind_b) = (0, usize::MAX);
        for (ind, c) in s.char_indices() {
            match c {
                'a' => ind_a = ind,
                'b' if ind_b == usize::MAX => ind_b = ind,
                _ => (),
            };
            if ind_a > ind_b {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_string("aaabbb".to_string()), true);
    assert_eq!(Solution::check_string("abab".to_string()), false);
    assert_eq!(Solution::check_string("bbb".to_string()), true);
}
