#![allow(dead_code)]

// 214. Shortest Palindrome
// https://leetcode.com/problems/shortest-palindrome/
//
// You are given a string s. You can convert s to a palindrome by adding characters in front of it.
//
// Return the shortest palindrome you can find by performing this transformation.
//
// Example 1:
//
// Input: s = "aacecaaa"
// Output: "aaacecaaa"
//
// Example 2:
//
// Input: s = "abcd"
// Output: "dcbabcd"
//
// Constraints:
//
// 0 <= s.length <= 5 * 104
// s consists of lowercase English letters only.

struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let prime = 31;
        let m = 1e9 as i64 + 7;
        let mut pnos = 31;
        let mut ind = 0;
        let s_bytes = s.as_bytes();
        let mut shash = s_bytes[0] as i64 - 'a' as i64 + 1;
        let mut rhash = s_bytes[0] as i64 - 'a' as i64 + 1;
        for (i, item) in s_bytes.iter().enumerate().take(s.len()).skip(1) {
            shash = (shash + (*item as i64 - 'a' as i64 + 1) * pnos) % m;
            rhash = (rhash * prime + (*item as i64 - 'a' as i64 + 1)) % m;
            pnos = (pnos * prime) % m;
            if shash == rhash {
                ind = i;
            }
        }
        let rev = s[ind + 1..].chars().rev().collect::<String>();
        rev + &s
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::shortest_palindrome("aacecaaa".to_string()),
        "aaacecaaa".to_string()
    );
    assert_eq!(Solution::shortest_palindrome("abcd".to_string()), "dcbabcd".to_string());
}
