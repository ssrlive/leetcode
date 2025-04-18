#![allow(dead_code)]

// 3517. Smallest Palindromic Rearrangement I
// https://leetcode.com/problems/smallest-palindromic-rearrangement-i/
// https://leetcode.cn/problems/smallest-palindromic-rearrangement-i/
//
// Medium
//
// You are given a string s.
// Return the palindromic of s.
//
// Example 1:
//
// Input: s = "z"
//
// Output: "z"
//
// Explanation:
//
// A string of only one character is already the lexicographically smallest palindrome.
//
// Example 2:
//
// Input: s = "babab"
//
// Output: "abbba"
//
// Explanation:
//
// Rearranging "babab" → "abbba" gives the smallest lexicographic palindrome.
//
// Example 3:
//
// Input: s = "daccad"
//
// Output: "acddca"
//
// Explanation:
//
// Rearranging "daccad" → "acddca" gives the smallest lexicographic palindrome.
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     s consists of lowercase English letters.
//     s is guaranteed to be palindromic.
//

struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let n = s.len();
        let m = n / 2;
        if n == 1 || n == 2 {
            return s;
        }

        let mut f: Vec<char> = s.chars().take(n / 2).collect(); // half string generation
        f.sort();

        let mut rev = f.clone();
        rev.reverse(); // reversal

        if n % 2 == 1 {
            f.push(s.chars().nth(m).unwrap()); // append middle char as is if n is odd
        }
        f.extend(rev); // concatenate
        f.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_palindrome("z".to_string()), "z".to_string());
    assert_eq!(Solution::smallest_palindrome("babab".to_string()), "abbba".to_string());
    assert_eq!(Solution::smallest_palindrome("daccad".to_string()), "acddca".to_string());
}
