#![allow(dead_code)]

/*

// 1657. Determine if Two Strings Are Close
// https://leetcode.com/problems/determine-if-two-strings-are-close/
// https://leetcode.com/problems/determine-if-two-strings-are-close/
//
// Medium
//
// Two strings are considered close if you can attain one from the other using the following operations:

    Operation 1: Swap any two existing characters.
        For example, abcde -> aecdb
    Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
        For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)

You can use the operations on either string as many times as necessary.

Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.

Example 1:

Input: word1 = "abc", word2 = "bca"
Output: true
Explanation: You can attain word2 from word1 in 2 operations.
Apply Operation 1: "abc" -> "acb"
Apply Operation 1: "acb" -> "bca"

Example 2:

Input: word1 = "a", word2 = "aa"
Output: false
Explanation: It is impossible to attain word2 from word1, or vice versa, in any number of operations.

Example 3:

Input: word1 = "cabbba", word2 = "abbccc"
Output: true
Explanation: You can attain word2 from word1 in 3 operations.
Apply Operation 1: "cabbba" -> "caabbb"
Apply Operation 2: "caabbb" -> "baaccc"
Apply Operation 2: "baaccc" -> "abbccc"

Constraints:

    1 <= word1.length, word2.length <= 10^5
    word1 and word2 contain only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut counts1 = [0; 26];
        let mut counts2 = [0; 26];
        for &b in word1.as_bytes() {
            counts1[(b - b'a') as usize] += 1;
        }
        for &b in word2.as_bytes() {
            counts2[(b - b'a') as usize] += 1;
        }
        if (0..26).any(|i| (counts1[i] > 0) != (counts2[i] > 0)) {
            return false;
        }
        counts1.sort_unstable();
        counts2.sort_unstable();
        counts1 == counts2
    }
}

#[test]
fn test() {
    let cases = vec![
        ("abc", "bca", true),
        ("a", "aa", false),
        ("cabbba", "abbccc", true),
        ("uau", "ssx", false),
    ];
    for (word1, word2, expected) in cases {
        assert_eq!(Solution::close_strings(word1.to_string(), word2.to_string()), expected);
    }
}
