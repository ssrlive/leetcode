#![allow(dead_code)]

/*

// 2108. Find First Palindromic String in the Array
// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
// https://leetcode.cn/problems/find-first-palindromic-string-in-the-array/
//
// Easy
//
// Given an array of strings words, return the first palindromic string in the array. If there is no such string, return an empty string "".

A string is palindromic if it reads the same forward and backward.

Example 1:

Input: words = ["abc","car","ada","racecar","cool"]
Output: "ada"
Explanation: The first string that is palindromic is "ada".
Note that "racecar" is also palindromic, but it is not the first.

Example 2:

Input: words = ["notapalindrome","racecar"]
Output: "racecar"
Explanation: The first and only string that is palindromic is "racecar".

Example 3:

Input: words = ["def","ghi"]
Output: ""
Explanation: There are no palindromic strings, so the empty string is returned.

Constraints:

    1 <= words.length <= 100
    1 <= words[i].length <= 100
    words[i] consists only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in &words {
            let half = word.len() / 2;
            if word.chars().take(half).eq(word.chars().rev().take(half)) {
                return word.to_string();
            }
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["abc", "car", "ada", "racecar", "cool"], "ada"),
        (vec!["notapalindrome", "racecar"], "racecar"),
        (vec!["def", "ghi"], ""),
    ];
    for (words, expected) in cases {
        let v = words.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::first_palindrome(v), expected);
    }
}
