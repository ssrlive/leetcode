#![allow(dead_code)]

// 336. Palindrome Pairs
// https://leetcode.com/problems/palindrome-pairs/
// https://leetcode.cn/problems/palindrome-pairs/
//
// You are given a 0-indexed array of unique strings words.
//
// A palindrome pair is a pair of integers (i, j) such that:
//
// - 0 <= i, j < word.length,
// - i != j, and
// - words[i] + words[j] (the concatenation of the two strings) is a palindrome.
// - Return an array of all the palindrome pairs of words.
//
// Example 1:
//
// Input: words = ["abcd","dcba","lls","s","sssll"]
// Output: [[0,1],[1,0],[3,2],[2,4]]
// Explanation: The palindromes are ["abcddcba","dcbaabcd","slls","llssssll"]
//
// Example 2:
//
// Input: words = ["bat","tab","cat"]
// Output: [[0,1],[1,0]]
// Explanation: The palindromes are ["battab","tabbat"]
//
// Example 3:
//
// Input: words = ["a",""]
// Output: [[0,1],[1,0]]
// Explanation: The palindromes are ["a","a"]
//
// Constraints:
//
// - 1 <= words.length <= 5000
// - 0 <= words[i].length <= 300
// - words[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let hm = words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect::<std::collections::HashMap<_, _>>();
        let mut answer = Vec::new();
        for (i, word) in words.iter().enumerate() {
            if let Some(&j) = hm.get(&word.chars().rev().collect::<String>()) {
                if i != j {
                    answer.push([i as i32, j as i32].to_vec());
                }
            }
            {
                let w = word.chars().rev().collect::<Vec<_>>();
                for k in 0..w.len() {
                    if (0..=k / 2).all(|l| w[l] == w[k - l]) {
                        if let Some(&j) = hm.get(&w[k + 1..].iter().collect::<String>()) {
                            answer.push([i as i32, j as i32].to_vec());
                        }
                    }
                }
            }
            {
                let w = word.chars().collect::<Vec<_>>();
                for k in 0..w.len() {
                    if (0..=k / 2).all(|l| w[l] == w[k - l]) {
                        if let Some(&j) = hm.get(&w[k + 1..].iter().rev().collect::<String>()) {
                            answer.push([j as i32, i as i32].to_vec());
                        }
                    }
                }
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::palindrome_pairs(vec![
            "abcd".to_string(),
            "dcba".to_string(),
            "lls".to_string(),
            "s".to_string(),
            "sssll".to_string()
        ]),
        vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]]
    );
    assert_eq!(
        Solution::palindrome_pairs(vec!["bat".to_string(), "tab".to_string(), "cat".to_string()]),
        vec![vec![0, 1], vec![1, 0]]
    );
    assert_eq!(
        Solution::palindrome_pairs(vec!["a".to_string(), "".to_string()]),
        vec![vec![0, 1], vec![1, 0]]
    );
}
