#![allow(dead_code)]

// 916. Word Subsets
// https://leetcode.com/problems/word-subsets/
// https://leetcode.cn/problems/word-subsets/
//
// You are given two string arrays words1 and words2.
//
// A string b is a subset of string a if every letter in b occurs in a including multiplicity.
//
// For example, "wrr" is a subset of "warrior" but is not a subset of "world".
// A string a from words1 is universal if for every string b in words2, b is a subset of a.
//
// Return an array of all the universal strings in words1. You may return the answer in any order.
//
// Example 1:
//
// Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["e","o"]
// Output: ["facebook","google","leetcode"]
//
// Example 2:
//
// Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["l","e"]
// Output: ["apple","google","leetcode"]
//
// Constraints:
//
// - 1 <= words1.length, words2.length <= 10^4
// - 1 <= words1[i].length, words2[i].length <= 10
// - words1[i] and words2[i] consist only of lowercase English letters.
// - All the strings of words1 are unique.
//

struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let char_counts = |s: &String| -> [usize; 26] {
            s.as_bytes().iter().fold([0; 26], |mut acc, &u| {
                acc[(u - b'a') as usize] += 1;
                acc
            })
        };
        let target = words2.iter().map(char_counts).fold(vec![0; 26], |acc, x| {
            acc.iter().enumerate().map(|(i, &count)| x[i].max(count)).collect()
        });
        words1
            .into_iter()
            .filter(|s| char_counts(s).iter().enumerate().all(|(i, &count)| count >= target[i]))
            .collect()
    }
}

#[test]
fn test() {
    let words1 = vec![
        "amazon".to_string(),
        "apple".to_string(),
        "facebook".to_string(),
        "google".to_string(),
        "leetcode".to_string(),
    ];
    let words2 = vec!["e".to_string(), "o".to_string()];
    let res = vec!["facebook".to_string(), "google".to_string(), "leetcode".to_string()];
    assert_eq!(Solution::word_subsets(words1, words2), res);

    let words1 = vec![
        "amazon".to_string(),
        "apple".to_string(),
        "facebook".to_string(),
        "google".to_string(),
        "leetcode".to_string(),
    ];
    let words2 = vec!["l".to_string(), "e".to_string()];
    let res = vec!["apple".to_string(), "google".to_string(), "leetcode".to_string()];
    assert_eq!(Solution::word_subsets(words1, words2), res);
}
