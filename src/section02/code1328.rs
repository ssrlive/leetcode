#![allow(dead_code)]

// 1328. Break a Palindrome
// https://leetcode.com/problems/break-a-palindrome/
// https://leetcode.cn/problems/break-a-palindrome/
//
// Medium
//
// Given a palindromic string of lowercase English letters palindrome, replace exactly one character with any lowercase
// English letter so that the resulting string is not a palindrome and that it is the lexicographically smallest one possible.
//
// Return the resulting string. If there is no way to replace a character to make it not a palindrome, return an empty string.
//
// A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ,
// a has a character strictly smaller than the corresponding character in b.
// For example, "abcc" is lexicographically smaller than "abcd" because the first position they differ is at the fourth character,
// and 'c' is smaller than 'd'.
//
// Example 1:
//
// Input: palindrome = "abccba"
// Output: "aaccba"
// Explanation: There are many ways to make "abccba" not a palindrome, such as "zbccba", "aaccba", and "abacba".
// Of all the ways, "aaccba" is the lexicographically smallest.
//
// Example 2:
//
// Input: palindrome = "a"
// Output: ""
// Explanation: There is no way to replace a single character to make "a" not a palindrome, so return an empty string.
//
// Constraints:
//
// -    1 <= palindrome.length <= 1000
// -    palindrome consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut chars: Vec<char> = palindrome.chars().collect();
        let n = chars.len();
        if n == 1 {
            return "".to_string();
        }
        for i in 0..n / 2 {
            if chars[i] != 'a' {
                chars[i] = 'a';
                return chars.iter().collect();
            }
        }
        chars[n - 1] = 'b';
        chars.iter().collect()
    }
}

#[test]
fn test() {
    let cases = vec![("abccba", "aaccba"), ("a", ""), ("aa", "ab"), ("aba", "abb")];
    for (palindrome, expected) in cases {
        assert_eq!(Solution::break_palindrome(palindrome.to_string()), expected);
    }
}
