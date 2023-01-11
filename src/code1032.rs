#![allow(dead_code)]

// 1032. Stream of Characters
// https://leetcode.com/problems/stream-of-characters/
// https://leetcode.cn/problems/stream-of-characters/
//
// Design an algorithm that accepts a stream of characters and checks if a suffix of these characters is a string of a given array of strings words.
//
// For example, if words = ["abc", "xyz"] and the stream added the four characters (one by one) 'a', 'x', 'y', and 'z', your algorithm should detect that the suffix "xyz" of the characters "axyz" matches "xyz" from words.
//
// Implement the StreamChecker class:
//
// StreamChecker(String[] words) Initializes the object with the strings array words.
// boolean query(char letter) Accepts a new character from the stream and returns true if any non-empty suffix from the stream forms a word that is in words.
//
// Example 1:
//
// Input
// ["StreamChecker", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query"]
// [[["cd", "f", "kl"]], ["a"], ["b"], ["c"], ["d"], ["e"], ["f"], ["g"], ["h"], ["i"], ["j"], ["k"], ["l"]]
// Output
// [null, false, false, false, true, false, true, false, false, false, false, false, true]
//
// Explanation
// StreamChecker streamChecker = new StreamChecker(["cd", "f", "kl"]);
// streamChecker.query("a"); // return False
// streamChecker.query("b"); // return False
// streamChecker.query("c"); // return False
// streamChecker.query("d"); // return True, because 'cd' is in the wordlist
// streamChecker.query("e"); // return False
// streamChecker.query("f"); // return True, because 'f' is in the wordlist
// streamChecker.query("g"); // return False
// streamChecker.query("h"); // return False
// streamChecker.query("i"); // return False
// streamChecker.query("j"); // return False
// streamChecker.query("k"); // return False
// streamChecker.query("l"); // return True, because 'kl' is in the wordlist
//
// Constraints:
//
// - 1 <= words.length <= 2000
// - 1 <= words[i].length <= 200
// - words[i] consists of lowercase English letters.
// - letter is a lowercase English letter.
// - At most 4 * 104 calls will be made to query.
//

struct TrieNode {
    layer: Vec<Option<TrieNode>>,
    words_exist: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            layer: (0..26).map(|_| None).collect(),
            words_exist: false,
        }
    }

    fn helper(&mut self, word: &String, idx: usize) {
        let key = (word.as_bytes()[idx] - b'a') as usize;
        let mut next_node = match self.layer[key].take() {
            None => TrieNode::new(),
            Some(node) => node,
        };

        if idx == 0 {
            next_node.words_exist = true;
        } else {
            next_node.helper(word, idx - 1);
        }

        self.layer[key] = Some(next_node);
    }

    fn add_word_in_reverse(&mut self, word: &String) {
        self.helper(word, word.len() - 1);
    }

    fn helper2(&self, suffix: &Vec<char>, idx: usize) -> bool {
        let key = ((suffix[idx] as u8) - b'a') as usize;
        if self.words_exist {
            return true;
        }
        match &self.layer[key] {
            Some(node) => {
                if idx == 0 {
                    node.words_exist
                } else {
                    node.helper2(suffix, idx - 1)
                }
            }
            None => false,
        }
    }

    fn query_suffix(&self, suffix: &Vec<char>) -> bool {
        self.helper2(suffix, suffix.len() - 1)
    }
}

struct StreamChecker {
    suffix: Vec<char>,
    trie: TrieNode,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = TrieNode::new();
        words.iter().for_each(|word| trie.add_word_in_reverse(word));
        Self { trie, suffix: vec![] }
    }

    fn query(&mut self, letter: char) -> bool {
        self.suffix.push(letter);
        self.trie.query_suffix(&self.suffix)
    }
}

#[test]
fn test() {
    let words = vec!["cd", "f", "kl"].into_iter().map(|s| s.to_string()).collect();
    let mut stream_checker = StreamChecker::new(words);
    assert_eq!(stream_checker.query('a'), false);
    assert_eq!(stream_checker.query('b'), false);
    assert_eq!(stream_checker.query('c'), false);
    assert_eq!(stream_checker.query('d'), true);
    assert_eq!(stream_checker.query('e'), false);
    assert_eq!(stream_checker.query('f'), true);
    assert_eq!(stream_checker.query('g'), false);
    assert_eq!(stream_checker.query('h'), false);
    assert_eq!(stream_checker.query('i'), false);
    assert_eq!(stream_checker.query('j'), false);
    assert_eq!(stream_checker.query('k'), false);
    assert_eq!(stream_checker.query('l'), true);
}
