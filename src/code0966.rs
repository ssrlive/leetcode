#![allow(dead_code)]

// 966. Vowel Spellchecker
// https://leetcode.com/problems/vowel-spellchecker/
// https://leetcode.cn/problems/vowel-spellchecker/
//
// Given a wordlist, we want to implement a spellchecker that converts a query word into a correct word.
//
// For a given query word, the spell checker handles two categories of spelling mistakes:
//
// Capitalization: If the query matches a word in the wordlist (case-insensitive), then the query word is returned
//   with the same case as the case in the wordlist.
// Example: wordlist = ["yellow"], query = "YellOw": correct = "yellow"
// Example: wordlist = ["Yellow"], query = "yellow": correct = "Yellow"
// Example: wordlist = ["yellow"], query = "yellow": correct = "yellow"
// Vowel Errors: If after replacing the vowels ('a', 'e', 'i', 'o', 'u') of the query word with any vowel individually, it matches a word
//   in the wordlist (case-insensitive), then the query word is returned with the same case as the match in the wordlist.
// Example: wordlist = ["YellOw"], query = "yollow": correct = "YellOw"
// Example: wordlist = ["YellOw"], query = "yeellow": correct = "" (no match)
// Example: wordlist = ["YellOw"], query = "yllw": correct = "" (no match)
// In addition, the spell checker operates under the following precedence rules:
//
// When the query exactly matches a word in the wordlist (case-sensitive), you should return the same word back.
// When the query matches a word up to capitlization, you should return the first such match in the wordlist.
// When the query matches a word up to vowel errors, you should return the first such match in the wordlist.
// If the query has no matches in the wordlist, you should return the empty string.
// Given some queries, return a list of words answer, where answer[i] is the correct word for query = queries[i].
//
// Example 1:
//
// Input: wordlist = ["KiTe","kite","hare","Hare"], queries = ["kite","Kite","KiTe","Hare","HARE","Hear","hear","keti","keet","keto"]
// Output: ["kite","KiTe","KiTe","Hare","hare","","","KiTe","","KiTe"]
//
// Example 2:
//
// Input: wordlist = ["yellow"], queries = ["YellOw"]
// Output: ["yellow"]
//
// Constraints:
//
// - 1 <= wordlist.length, queries.length <= 5000
// - 1 <= wordlist[i].length, queries[i].length <= 7
// - wordlist[i] and queries[i] consist only of only English letters.
//

struct Solution;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        use std::collections::{HashMap, HashSet};

        let mut exactly_matches = HashSet::new();
        let mut capitalization = HashMap::new();
        let mut vowel = HashMap::new();
        let to_any_vowels = |s: &String| -> String {
            s.to_ascii_lowercase()
                .replace(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'), "*")
        };
        for word in wordlist.iter() {
            exactly_matches.insert(word);
            capitalization.entry(word.to_ascii_lowercase()).or_insert(word);
            vowel.entry(to_any_vowels(word)).or_insert(word);
        }
        let convert = |s: &String| -> String {
            if exactly_matches.contains(s) {
                return s.clone();
            }
            if let Some(&collect) = capitalization.get(&s.to_ascii_lowercase()) {
                return collect.clone();
            }
            if let Some(&collect) = vowel.get(&to_any_vowels(s)) {
                return collect.clone();
            }
            String::new()
        };
        queries.iter().map(convert).collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["KiTe", "kite", "hare", "Hare"],
            vec![
                "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
            ],
            vec!["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"],
        ),
        (vec!["yellow"], vec!["YellOw"], vec!["yellow"]),
    ];
    let cases = cases
        .into_iter()
        .map(|(wordlist, queries, expected)| {
            (
                wordlist.into_iter().map(String::from).collect::<Vec<String>>(),
                queries.into_iter().map(String::from).collect::<Vec<String>>(),
                expected.into_iter().map(String::from).collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<_>>();
    for (wordlist, queries, expected) in cases {
        assert_eq!(Solution::spellchecker(wordlist, queries), expected);
    }
}
