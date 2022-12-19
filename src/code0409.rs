#![allow(dead_code)]

// 409. Longest Palindrome
// https://leetcode.com/problems/longest-palindrome/
// https://leetcode.cn/problems/longest-palindrome/
//
// Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
//
// Letters are case sensitive, for example, "Aa" is not considered a palindrome here.
//
// Example 1:
//
// Input: s = "abccccdd"
// Output: 7
// Explanation:
// One longest palindrome that can be built is "dccaccd", whose length is 7.
//
// Example 2:
//
// Input: s = "a"
// Output: 1
//
// Example 3:
//
// Input: s = "bb"
// Output: 2
//
// Constraints:
//
// - 1 <= s.length <= 2000
// - s consists of lowercase and/or uppercase English letters only.
//

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = vec![0; 128];
        for c in s.bytes() {
            count[c as usize] += 1;
        }
        let mut res = 0;
        let mut odd = false;
        for &c in &count {
            if c % 2 == 0 {
                res += c;
            } else {
                res += c - 1;
                odd = true;
            }
        }
        if odd {
            res + 1
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let s = "abccccdd".to_string();
    let res = 7;
    assert_eq!(Solution::longest_palindrome(s), res);
    let s = "a".to_string();
    let res = 1;
    assert_eq!(Solution::longest_palindrome(s), res);
    let s = "bb".to_string();
    let res = 2;
    assert_eq!(Solution::longest_palindrome(s), res);
}
