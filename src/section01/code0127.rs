#![allow(dead_code)]

// 127. Word Ladder
// https://leetcode.com/problems/word-ladder/
// https://leetcode.cn/problems/word-ladder/
// https://leetcode.cn/problems/word-ladder/solution/dan-ci-jie-long-by-leetcode-solution/
//
// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a
// sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
//
// - Every adjacent pair of words differs by a single letter.
// - Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
// - sk == endWord
//
// Given two words, beginWord and endWord, and a dictionary wordList, return the number of words
// in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
//
// Example 1:
//
// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
// Output: 5
// Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
//
// Example 2:
//
// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
// Output: 0
// Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
//
// Constraints:
//
// - 1 <= beginWord.length <= 10
// - endWord.length == beginWord.length
// - 1 <= wordList.length <= 5000
// - wordList[i].length == beginWord.length
// - beginWord, endWord, and wordList[i] consist of lowercase English letters.
// - beginWord != endWord
// - All the words in wordList are unique.
//

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        fn words_exchangable(a: &str, b: &str) -> bool {
            let a = a.as_bytes();
            let b = b.as_bytes();
            let len = a.len();
            if len != b.len() {
                return false;
            }
            let mut times = 0;
            for i in 0..len {
                if a[i] != b[i] {
                    times += 1;
                }
                if times > 1 {
                    return false;
                }
            }
            times == 1
        }

        use std::collections::{HashSet, VecDeque};
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
        word_set.remove(&begin_word);
        if !word_set.contains(&end_word) {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back(begin_word);
        let mut step = 1;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let front = queue.pop_front().unwrap();
                let mut tmp = vec![];
                for w in word_set.iter() {
                    if words_exchangable(&front, w) {
                        if w == &end_word {
                            return step + 1;
                        }
                        tmp.push(w.clone());
                        queue.push_back(w.clone());
                    }
                }
                for w in tmp {
                    word_set.remove(&w);
                }
            }
            step += 1;
        }
        0
    }

    pub fn ladder_length_2(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
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
    let cases = vec![
        ("hit", "cog", vec!["hot", "dot", "dog", "lot", "log", "cog"], 5),
        ("hit", "cog", vec!["hot", "dot", "dog", "lot", "log"], 0),
        (
            "qa",
            "sq",
            vec![
                "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le", "av", "sm", "ar",
                "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn", "ya", "cr", "po", "fe", "ho", "ma",
                "re", "or", "rn", "au", "ur", "rh", "sr", "tc", "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge",
                "th", "pm", "rb", "sh", "co", "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa", "pi", "os", "uh",
                "wm", "an", "me", "mo", "na", "la", "st", "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io", "be",
                "fm", "ta", "tb", "ni", "mr", "pa", "he", "lr", "sq", "ye",
            ],
            5,
        ),
    ];
    for (begin_word, end_word, word_list, expected) in cases {
        let begin_word = begin_word.to_string();
        let end_word = end_word.to_string();
        let word_list = word_list.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), expected);
    }
}
