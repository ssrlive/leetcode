#![allow(dead_code)]

// 211. Design Add and Search Words Data Structure
// https://leetcode.com/problems/design-add-and-search-words-data-structure/
//
// Design a data structure that supports adding new words and finding if a string matches any previously added string.
//
// Implement the WordDictionary class:
//
// - WordDictionary() Initializes the object.
// - void addWord(word) Adds word to the data structure, it can be matched later.
// - bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise.
//   word may contain dots '.' where dots can be matched with any letter.
//

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

#[derive(Default)]
struct TrieNode {
    word: bool,
    children: [usize; N_LETTERS],
}

struct WordDictionary {
    nodes: Vec<TrieNode>,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }

    fn add_word(&mut self, word: String) {
        let mut i = 0;
        for b in word.bytes() {
            let c = (b - b'a') as usize;
            if self.nodes[i].children[c] == 0 {
                self.nodes[i].children[c] = self.nodes.len();
                self.nodes.push(TrieNode::default());
            }
            i = self.nodes[i].children[c];
        }
        self.nodes[i].word = true;
    }

    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        let mut stack = vec![(0, 0)];

        while let Some((i, j)) = stack.pop() {
            let node = &self.nodes[i];
            if j == word.len() {
                if node.word {
                    return true;
                }
            } else {
                let b = word[j];
                if b == b'.' {
                    // stack.extend(node.children.iter().filter_map(|c| (*c != 0).then(|| (*c, j+1))));
                    stack.extend(
                        node.children
                            .iter()
                            .filter_map(|c| (*c != 0).then_some((*c, j + 1))),
                    );
                } else {
                    let c = node.children[(b - b'a') as usize];
                    if c != 0 {
                        stack.push((c, j + 1));
                    }
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let mut wd = WordDictionary::new();
    wd.add_word("bad".to_string());
    wd.add_word("dad".to_string());
    wd.add_word("mad".to_string());
    assert_eq!(wd.search("pad".to_string()), false);
    assert_eq!(wd.search("bad".to_string()), true);
    assert_eq!(wd.search(".ad".to_string()), true);
    assert_eq!(wd.search("b..".to_string()), true);
}
