#![allow(dead_code)]

// 524. Longest Word in Dictionary through Deleting
// https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/
// https://leetcode.cn/problems/longest-word-in-dictionary-through-deleting/
//
// Given a string s and a string array dictionary, return the longest string in the dictionary that can be formed by deleting some of the given string characters. If there is more than one possible result, return the longest word with the smallest lexicographical order. If there is no possible result, return the empty string.
//
// Example 1:
//
// Input: s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
// Output: "apple"
//
// Example 2:
//
// Input: s = "abpcplea", dictionary = ["a","b","c"]
// Output: "a"
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - 1 <= dictionary.length <= 1000
// - 1 <= dictionary[i].length <= 1000
// - s and dictionary[i] consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut dictionary = dictionary;
        dictionary.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
        let s = s.as_bytes();
        let can_be_formed = |word: &String| -> bool {
            let mut i = 0;
            for &b in word.as_bytes() {
                while i < s.len() && s[i] != b {
                    i += 1;
                }
                if i == s.len() {
                    return false;
                }
                i += 1;
            }
            true
        };
        dictionary.into_iter().find(can_be_formed).unwrap_or_default()
    }
}

#[test]
fn test() {
    let s = "abpcplea".to_string();
    let dictionary = ["ale", "apple", "monkey", "plea"];
    let dictionary = dictionary.iter().map(|s| s.to_string()).collect();
    let res = "apple".to_string();
    assert_eq!(Solution::find_longest_word(s, dictionary), res);

    let s = "abpcplea".to_string();
    let dictionary = ["a", "b", "c"];
    let dictionary = dictionary.iter().map(|s| s.to_string()).collect();
    let res = "a".to_string();
    assert_eq!(Solution::find_longest_word(s, dictionary), res);
}
