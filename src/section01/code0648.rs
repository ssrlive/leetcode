#![allow(dead_code)]

// 648. Replace Words
// https://leetcode.com/problems/replace-words/
// https://leetcode.cn/problems/replace-words/
//
// In English, we have a concept called root, which can be followed by some other word
// to form another longer word - let's call this word successor. For example,
// when the root "an" is followed by the successor word "other", we can form a new word "another".
//
// Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces,
// replace all the successors in the sentence with the root forming it.
// If a successor can be replaced by more than one root, replace it with the root that has the shortest length.
//
// Return the sentence after the replacement.
//
// Example 1:
//
// Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
// Output: "the cat was rat by the bat"
//
// Example 2:
//
// Input: dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
// Output: "a a b c"
//
// Constraints:
//
// - 1 <= dictionary.length <= 1000
// - 1 <= dictionary[i].length <= 100
// - dictionary[i] consists of only lower-case letters.
// - 1 <= sentence.length <= 106
// - sentence consists of only lower-case letters and spaces.
// - The number of words in sentence is in the range [1, 1000]
// - The length of each word in sentence is in the range [1, 1000]
// - Every two consecutive words in sentence will be separated by exactly one space.
// - sentence does not have leading or trailing spaces.
//

struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let trie = dictionary.iter().enumerate().fold(Trie::new(), |mut trie, (i, word)| {
            trie.insert(word.as_bytes(), i);
            trie
        });
        sentence
            .split_whitespace()
            .map(|word| match trie.find(word.as_bytes()) {
                Some(i) => dictionary[i].clone(),
                None => word.to_owned(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

#[derive(Default)]
struct TrieNode {
    entry: Option<usize>,
    children: [usize; N_LETTERS],
}

struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }

    fn insert(&mut self, s: &[u8], entry: usize) {
        let mut i = 0;
        for child in s.iter().map(|b| (*b - b'a') as usize) {
            if self.nodes[i].children[child] == 0 {
                self.nodes[i].children[child] = self.nodes.len();
                self.nodes.push(TrieNode::default());
            }
            i = self.nodes[i].children[child];
        }
        self.nodes[i].entry = Some(entry);
    }

    fn find(&self, s: &[u8]) -> Option<usize> {
        let mut i = 0;
        for child in s.iter().map(|b| (*b - b'a') as usize) {
            i = self.nodes[i].children[child];
            if i == 0 {
                return None;
            }
            if self.nodes[i].entry.is_some() {
                return self.nodes[i].entry;
            }
        }
        None
    }
}

#[test]
fn test() {
    let dictionary = vec!["cat", "bat", "rat"].into_iter().map(|s| s.to_string()).collect();
    let sentence = "the cattle was rattled by the battery".to_string();
    let expected = "the cat was rat by the bat".to_string();
    assert_eq!(Solution::replace_words(dictionary, sentence), expected);

    let dictionary = vec!["a", "b", "c"].into_iter().map(|s| s.to_string()).collect();
    let sentence = "aadsfasf absbs bbab cadsfafs".to_string();
    let expected = "a a b c".to_string();
    assert_eq!(Solution::replace_words(dictionary, sentence), expected);

    let dictionary = vec!["a", "aa", "aaa", "aaaa"].into_iter().map(|s| s.to_string()).collect();
    let sentence = "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa".to_string();
    let expected = "a a a a a a a a bbb baba a".to_string();
    assert_eq!(Solution::replace_words(dictionary, sentence), expected);

    let dictionary = vec!["catt", "cat", "bat", "rat"].into_iter().map(|s| s.to_string()).collect();
    let sentence = "the cattle was rattled by the battery".to_string();
    let expected = "the cat was rat by the bat".to_string();
    assert_eq!(Solution::replace_words(dictionary, sentence), expected);
}
