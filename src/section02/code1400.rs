#![allow(dead_code)]

// 1400. Construct K Palindrome Strings
// https://leetcode.com/problems/construct-k-palindrome-strings/
// https://leetcode.cn/problems/construct-k-palindrome-strings/
//
// Medium
//
// Given a string s and an integer k, return true if you can use all the characters in s to construct k palindrome strings or false otherwise.
//
// Example 1:
//
// Input: s = "annabelle", k = 2
// Output: true
// Explanation: You can construct two palindromes using all characters in s.
// Some possible constructions "anna" + "elble", "anbna" + "elle", "anellena" + "b"
//
// Example 2:
//
// Input: s = "leetcode", k = 3
// Output: false
// Explanation: It is impossible to construct 3 palindromes using all the characters of s.
//
// Example 3:
//
// Input: s = "true", k = 4
// Output: true
// Explanation: The only possible solution is to put each character in a separate string.
//
// Constraints:
//
// -    1 <= s.length <= 10^5
// -    s consists of lowercase English letters.
// -    1 <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if (s.len() as i32) < k {
            return false;
        }
        let mut memo = vec![0; 26];
        for c in s.chars() {
            memo[(c as u8 - b'a') as usize] += 1;
        }
        let mut odds = 0;
        for v in memo {
            if v % 2 == 1 {
                odds += 1;
            }
        }
        k >= odds
    }
}

#[test]
fn test() {
    assert!(Solution::can_construct("annabelle".to_string(), 2));
    assert!(!Solution::can_construct("leetcode".to_string(), 3));
    assert!(Solution::can_construct("true".to_string(), 4));
}
