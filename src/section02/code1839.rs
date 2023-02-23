#![allow(dead_code)]

/*

// 1839. Longest Substring Of All Vowels in Order
// https://leetcode.com/problems/longest-substring-of-all-vowels-in-order/
// https://leetcode.cn/problems/longest-substring-of-all-vowels-in-order/
//
// Medium
//
// A string is considered beautiful if it satisfies the following conditions:

    Each of the 5 English vowels ('a', 'e', 'i', 'o', 'u') must appear at least once in it.
    The letters must be sorted in alphabetical order (i.e. all 'a's before 'e's, all 'e's before 'i's, etc.).

For example, strings "aeiou" and "aaaaaaeiiiioou" are considered beautiful, but "uaeio", "aeoiu", and "aaaeeeooo" are not beautiful.

Given a string word consisting of English vowels, return the length of the longest beautiful substring of word. If no such substring exists, return 0.

A substring is a contiguous sequence of characters in a string.

Example 1:

Input: word = "aeiaaioaaaaeiiiiouuuooaauuaeiu"
Output: 13
Explanation: The longest beautiful substring in word is "aaaaeiiiiouuu" of length 13.

Example 2:

Input: word = "aeeeiiiioooauuuaeiou"
Output: 5
Explanation: The longest beautiful substring in word is "aeiou" of length 5.

Example 3:

Input: word = "a"
Output: 0
Explanation: There is no beautiful substring, so return 0.

Constraints:

    1 <= word.length <= 5 * 10^5
    word consists of characters 'a', 'e', 'i', 'o', and 'u'.
*/

struct Solution;

impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        use std::collections::HashSet;
        let bytes = word.into_bytes();
        let a_indice = {
            let mut res = vec![];
            for i in 0..bytes.len() {
                if bytes[i] == b'a' && (i == 0 || bytes[i - 1] != b'a') {
                    res.push(i);
                }
            }
            res
        };
        let mut res = 0;
        let mut appeared = HashSet::new();
        for l in a_indice {
            let mut r = l;
            appeared.clear();
            appeared.insert(bytes[r]);
            while r + 1 < bytes.len() {
                if bytes[r + 1] < bytes[r] {
                    if appeared.len() == 5 {
                        res = res.max(r - l + 1);
                    }
                    break;
                } else {
                    r += 1;
                    appeared.insert(bytes[r]);
                }
            }
            if appeared.len() == 5 {
                res = res.max(r - l + 1);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        ("aeiaaioaaaaeiiiiouuuooaauuaeiu", 13),
        ("aeeeiiiioooauuuaeiou", 5),
        ("a", 0),
        ("aeiou", 5),
        ("aeiaaioaaaaeiiiiouuuooaauuaeiu", 13),
    ];
    for (word, expected) in cases {
        assert_eq!(Solution::longest_beautiful_substring(word.to_string()), expected);
    }
}
