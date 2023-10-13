#![allow(dead_code)]

// 208. Implement Trie (Prefix Tree)
// https://leetcode.com/problems/implement-trie-prefix-tree/
// https://leetcode.cn/problems/implement-trie-prefix-tree/
//
// Implement a trie with insert, search, and startsWith methods.
//
// Example:
//
// Trie trie = new Trie();
//
// trie.insert("apple");
// trie.search("apple");   // returns true
// trie.search("app");     // returns false
// trie.startsWith("app"); // returns true
// trie.insert("app");
// trie.search("app");     // returns true
//
// Note:
//
// You may assume that all inputs are consist of lowercase letters a-z.
// All inputs are guaranteed to be non-empty strings.
//

use std::collections::HashMap;

struct Trie {
    root: TrieNode,
}

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

impl Trie {
    fn new() -> Self {
        Self { root: TrieNode::new() }
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            let next_node = current_node.children.entry(c).or_insert_with(TrieNode::new);
            current_node = next_node;
        }
        current_node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }
        current_node.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = &self.root;
        for c in prefix.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }
        true
    }
}

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string()));
    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    assert!(trie.search("app".to_string()));
}
