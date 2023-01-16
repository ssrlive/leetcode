#![allow(dead_code)]

// 2423. Remove Letter To Equalize Frequency
// https://leetcode.com/problems/remove-letter-to-equalize-frequency/
// https://leetcode.cn/problems/remove-letter-to-equalize-frequency/
//
// You are given a 0-indexed string word, consisting of lowercase English letters.
// You need to select one index and remove the letter at that index from word so that the frequency of every letter present in word is equal.
//
// Return true if it is possible to remove one letter so that the frequency of all letters in word are equal, and false otherwise.
//
// Note:
//
// The frequency of a letter x is the number of times it occurs in the string.
// You must remove exactly one letter and cannot chose to do nothing.
//
// Example 1:
//
// Input: word = "abcc"
// Output: true
// Explanation: Select index 3 and delete it: word becomes "abc" and each character has a frequency of 1.
//
// Example 2:
//
// Input: word = "aazz"
// Output: false
// Explanation: We must delete a character, so either the frequency of "a" is 1 and the frequency of "z" is 2, or vice versa.
//              It is impossible to make all present letters have equal frequency.
//
// Constraints:
//
// - 2 <= word.length <= 100
// - word consists of lowercase English letters only.
//

struct Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        use std::collections::HashMap;

        let mut cf = HashMap::new();
        for c in word.chars() {
            *cf.entry(c).or_insert(0) += 1;
        }
        if cf.len() == 1 {
            return true;
        }
        let mut fc = HashMap::new();
        for (_, f) in cf {
            *fc.entry(f).or_insert(0) += 1;
        }
        if fc.len() != 2 {
            if fc.len() == 1 && fc.contains_key(&1) {
                return true;
            }
            return false;
        }
        let mut arr: Vec<[i32; 2]> = fc.into_iter().map(|x| [x.0, x.1]).collect();
        arr.sort();
        if arr[0][0] == 1 && arr[0][1] == 1 {
            return true;
        }
        if arr[1][1] == 1 && arr[1][0] - arr[0][0] == 1 {
            return true;
        }
        false
    }
}

#[test]
fn test() {
    let cases = vec![
        ("abcc", true),
        ("aazz", false),
        ("aaabbb", false),
        ("aaabbbccc", false),
        ("aaabbbcccddd", false),
        ("aaabbbcccddde", true),
        ("aaabbbcccdddeee", false),
    ];
    for (word, expected) in cases {
        assert_eq!(Solution::equal_frequency(word.to_string()), expected);
    }
}
