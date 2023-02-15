#![allow(dead_code)]

/*

// 1662. Check If Two String Arrays are Equivalent
// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
// https://leetcode.cn/problems/check-if-two-string-arrays-are-equivalent/
//
// Easy
//
// Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.

A string is represented by an array if the array elements concatenated in order forms the string.

Example 1:

Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
Output: true
Explanation:
word1 represents string "ab" + "c" -> "abc"
word2 represents string "a" + "bc" -> "abc"
The strings are the same, so return true.

Example 2:

Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
Output: false

Example 3:

Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
Output: true

Constraints:

    1 <= word1.length, word2.length <= 10^3
    1 <= word1[i].length, word2[i].length <= 10^3
    1 <= sum(word1[i].length), sum(word2[i].length) <= 10^3
    word1[i] and word2[i] consist of lowercase letters.
*/

struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut s1 = String::new();
        let mut s2 = String::new();
        for w in word1 {
            s1.push_str(&w);
        }
        for w in word2 {
            s2.push_str(&w);
        }
        s1 == s2
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["ab", "c"], vec!["a", "bc"], true),
        (vec!["a", "cb"], vec!["ab", "c"], false),
        (vec!["abc", "d", "defg"], vec!["abcddefg"], true),
    ];
    for (word1, word2, expected) in cases {
        let word1 = word1.iter().map(|s| s.to_string()).collect();
        let word2 = word2.iter().map(|s| s.to_string()).collect();
        let result = Solution::array_strings_are_equal(word1, word2);
        assert_eq!(result, expected);
    }
}
