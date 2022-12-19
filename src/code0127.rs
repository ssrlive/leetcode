#![allow(dead_code)]

// 127. Word Ladder
// https://leetcode.com/problems/word-ladder/
// https://leetcode.cn/problems/word-ladder/
//
// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
// - Every adjacent pair of words differs by a single letter.
// - Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
// - sk == endWord
//
// Given two words, beginWord and endWord, and a dictionary wordList, return the number of words
// in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
//

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        fn is_adjacent(a: &str, b: &str) -> bool {
            let a = a.as_bytes();
            let b = b.as_bytes();
            assert_eq!(a.len(), b.len());

            let mut diffs = 0;
            for idx in 0..a.len() {
                if a[idx] != b[idx] {
                    diffs += 1;
                }

                if diffs > 1 {
                    break;
                }
            }

            diffs == 1
        }

        if !word_list.iter().any(|w| end_word.eq(w)) {
            return 0;
        }

        let mut visited = std::collections::HashSet::new();
        let mut left = std::collections::HashSet::new();
        let mut right = std::collections::HashSet::new();
        let mut next = std::collections::HashSet::new();

        left.insert(begin_word.as_str());
        right.insert(end_word.as_str());
        visited.insert(begin_word.as_str());
        visited.insert(end_word.as_str());

        let mut len = 1;
        while !left.is_empty() && !right.is_empty() {
            if left.len() > right.len() {
                std::mem::swap(&mut left, &mut right);
            }

            for w in left.drain() {
                for n in word_list.iter().map(|s| s.as_str()) {
                    if is_adjacent(w, n) {
                        if right.contains(n) {
                            return len + 1;
                        }

                        if visited.insert(n) {
                            next.insert(n);
                        }
                    }
                }
            }

            std::mem::swap(&mut left, &mut next);
            len += 1;
        }

        0
    }
}

#[test]
fn test_ladder_length() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);

    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec!["hot", "dot", "dog", "lot", "log"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
}
