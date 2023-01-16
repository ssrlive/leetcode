#![allow(dead_code)]

// 720. Longest Word in Dictionary
// https://leetcode.com/problems/longest-word-in-dictionary/
// https://leetcode.cn/problems/longest-word-in-dictionary/
//
// Given an array of strings words representing an English Dictionary, return the longest word in words that can be built one character at a time by other words in words.
//
// If there is more than one possible answer, return the longest word with the smallest lexicographical order. If there is no answer, return the empty string.
//
// Note that the word should be built from left to right with each additional character being added to the end of a previous word.
//
// Example 1:
//
// Input: words = ["w","wo","wor","worl","world"]
// Output: "world"
// Explanation: The word "world" can be built one character at a time by "w", "wo", "wor", and "worl".
//
// Example 2:
//
// Input: words = ["a","banana","app","appl","ap","apply","apple"]
// Output: "apple"
// Explanation: Both "apply" and "apple" can be built from other words in the dictionary. However, "apple" is lexicographically smaller than "apply".
//
// Constraints:
//
// - 1 <= words.length <= 1000
// - 1 <= words[i].length <= 30
// - words[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut words = words;
        words.sort_unstable();
        let mut set = std::collections::HashSet::new();
        let mut ans = String::new();

        for word in words {
            if word.len() == 1 || set.contains(&word[..word.len() - 1]) {
                if word.len() > ans.len() {
                    ans = word.clone();
                }
                set.insert(word.clone());
            }
        }

        ans
    }
}

#[test]
fn test() {
    let words = vec!["w", "wo", "wor", "worl", "world"];
    let words = words.iter().map(|s| s.to_string()).collect();
    let ans = "world".to_string();
    assert_eq!(Solution::longest_word(words), ans);

    let words = vec!["a", "banana", "app", "appl", "ap", "apply", "apple"];
    let words = words.iter().map(|s| s.to_string()).collect();
    let ans = "apple".to_string();
    assert_eq!(Solution::longest_word(words), ans);
}
