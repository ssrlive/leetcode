#![allow(dead_code)]

// 745. Prefix and Suffix Search
// https://leetcode.com/problems/prefix-and-suffix-search/
// https://leetcode.cn/problems/prefix-and-suffix-search/
//
// Design a special dictionary that searches the words in it by a prefix and a suffix.
//
// Implement the WordFilter class:
//
// - WordFilter(string[] words) Initializes the object with the words in the dictionary.
// - f(string pref, string suff) Returns the index of the word in the dictionary, which has the prefix pref and the suffix suff.
//   If there is more than one valid index, return the largest of them. If there is no such word in the dictionary, return -1.
//
// Example 1:
//
// Input
// ["WordFilter", "f"]
// [[["apple"]], ["a", "e"]]
// Output
// [null, 0]
// Explanation
// WordFilter wordFilter = new WordFilter(["apple"]);
// wordFilter.f("a", "e"); // return 0, because the word at index 0 has prefix = "a" and suffix = "e".
//
// Constraints:
//
// - 1 <= words.length <= 10^4
// - 1 <= words[i].length <= 7
// - 1 <= pref.length, suff.length <= 7
// - words[i], pref and suff consist of lowercase English letters only.
// - At most 10^4 calls will be made to the function f.
//

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    index: HashSet<usize>,
}

struct WordFilter {
    prefix_root: TrieNode,
    suffix_root: TrieNode,
}

impl TrieNode {
    pub fn insert(&mut self, word: &str, index: usize) {
        let mut node = self;

        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
            node.index.insert(index);
        }
    }

    pub fn find(&mut self, word: &str) -> Option<HashSet<usize>> {
        let mut node = self;
        for ch in word.chars() {
            if let Some(n) = node.children.get_mut(&ch) {
                node = n;
            } else {
                return None;
            }
        }
        Some(node.index.clone())
    }
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let unique_val: HashMap<String, usize> = words.iter().enumerate().map(|(i, w)| (w.to_owned(), i)).collect();

        let mut prefix_root = TrieNode::default();
        let mut suffix_root = TrieNode::default();

        for (w, i) in unique_val.iter() {
            prefix_root.insert(w.as_str(), i.to_owned());
            suffix_root.insert(&w.chars().rev().collect::<String>(), i.to_owned());
        }

        Self { prefix_root, suffix_root }
    }

    fn f(&mut self, prefix: String, suffix: String) -> i32 {
        let prefix_res = self.prefix_root.find(&prefix);
        if prefix_res.is_none() {
            return -1;
        }
        let p_res = match prefix_res {
            Some(res) => res,
            None => return -1,
        };
        let suffix_res = self.suffix_root.find(&suffix.chars().rev().collect::<String>());
        let s_res = match suffix_res {
            Some(res) => res,
            None => return -1,
        };

        let res = p_res.intersection(&s_res);
        if let Some(i) = res.max() { *i as i32 } else { -1 }
    }
}

#[test]
fn test() {
    let mut word_filter = WordFilter::new(vec![
        "apple".to_string(),
        "banana".to_string(),
        "orange".to_string(),
        "watermelon".to_string(),
    ]);
    assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 0);
    assert_eq!(word_filter.f("b".to_string(), "a".to_string()), 1);
    assert_eq!(word_filter.f("o".to_string(), "e".to_string()), 2);
    assert_eq!(word_filter.f("w".to_string(), "melon".to_string()), 3);
    assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 0);
    assert_eq!(word_filter.f("b".to_string(), "a".to_string()), 1);
    assert_eq!(word_filter.f("o".to_string(), "e".to_string()), 2);
    assert_eq!(word_filter.f("w".to_string(), "melon".to_string()), 3);
    assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 0);
    assert_eq!(word_filter.f("b".to_string(), "a".to_string()), 1);
    assert_eq!(word_filter.f("o".to_string(), "e".to_string()), 2);
    assert_eq!(word_filter.f("w".to_string(), "melon".to_string()), 3);
    assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 0);
    assert_eq!(word_filter.f("b".to_string(), "a".to_string()), 1);
    assert_eq!(word_filter.f("o".to_string(), "e".to_string()), 2);
    assert_eq!(word_filter.f("w".to_string(), "melon".to_string()), 3);
    assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 0);
    assert_eq!(word_filter.f("b".to_string(), "a".to_string()), 1);
    assert_eq!(word_filter.f("o".to_string(), "e".to_string()), 2);
    assert_eq!(word_filter.f("w".to_string(), "melon".to_string()), 3);
    assert_eq!(word_filter.f("a".to_string(), "lovely".to_string()), -1);
    assert_eq!(word_filter.f("b".to_string(), "lovely".to_string()), -1);
    assert_eq!(word_filter.f("o".to_string(), "lovely".to_string()), -1);
}
