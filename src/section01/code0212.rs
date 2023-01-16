#![allow(dead_code)]

// 212. Word Search II
// https://leetcode.com/problems/word-search-ii/
// https://leetcode.cn/problems/word-search-ii/
//
// Given an m x n board of characters and a list of strings words, return all words on the board.
//
// Each word must be constructed from letters of sequentially adjacent cells,
// where adjacent cells are horizontally or vertically neighboring.
// The same letter cell may not be used more than once in a word.
//
// Example 1:
//
// Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]],
//        words = ["oath","pea","eat","rain"]
// Output: ["eat","oath"]
//
// Example 2:
//
// Input: board = [["a","b"],["c","d"]], words = ["abcb"]
// Output: []
//

use std::collections::HashSet;
use std::ops::{Index, IndexMut};

const INNER_SIDE: usize = 8;
const RANGE: usize = (b'z' - b'a') as usize + 1;

#[inline(always)]
fn alph_to_idx(c: u8) -> usize {
    (c - b'a').into()
}

struct Board<T: Copy + Default> {
    b_width: usize,
    width: usize,
    height: usize,
    buff: Vec<[[T; INNER_SIDE]; INNER_SIDE]>,
}

impl<T: Copy + Default> Board<T> {
    fn zeroed(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);
        let b_width = width / INNER_SIDE + usize::from(height % INNER_SIDE > 0);
        let b_height = height / INNER_SIDE + usize::from(height % INNER_SIDE > 0);
        Self {
            b_width,
            width,
            height,
            buff: vec![[[T::default(); INNER_SIDE]; INNER_SIDE]; b_height * b_width],
        }
    }
}

fn from_chars(board: Vec<Vec<char>>) -> Board<u8> {
    assert!(!board.is_empty(), "Expected square");
    assert!(!board.first().unwrap().is_empty(), "Expected square");
    assert!(board.windows(2).all(|w| w[0].len() == w[1].len()), "Expected square");
    assert!(
        board.iter().all(|sub| sub.iter().all(char::is_ascii_lowercase)),
        "Expected ascii lowercase"
    );
    let width = board[0].len();
    let height = board.len();
    let mut res = Board::zeroed(width, height);
    for (y, row) in board.into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            res[(x, y)] = elem as u8;
        }
    }
    res
}

fn calc_existing_chars(board: &Board<u8>) -> [usize; RANGE] {
    let mut result = [0; RANGE];
    for block in board.buff.iter() {
        if block.iter().flatten().any(|&x| x == 0) {
            for b in block.iter().flatten().copied().filter(|&x| x != 0).map(alph_to_idx) {
                result[b] += 1;
            }
        } else {
            for b in block.iter().flatten().copied().map(alph_to_idx) {
                result[b] += 1;
            }
        }
    }
    result
}

impl<T: Copy + Default> Index<(usize, usize)> for Board<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width && y < self.height, "Should be in bounds");
        let block_x = x / INNER_SIDE;
        let inner_x = x % INNER_SIDE;
        let block_y = y / INNER_SIDE;
        let inner_y = y % INNER_SIDE;
        let block = &self.buff[self.b_width * block_y + block_x];
        &block[inner_y][inner_x]
    }
}

impl<T: Copy + Default> IndexMut<(usize, usize)> for Board<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.width && y < self.height, "Should be in bounds");
        let block_x = x / INNER_SIDE;
        let inner_x = x % INNER_SIDE;
        let block_y = y / INNER_SIDE;
        let inner_y = y % INNER_SIDE;
        let block = &mut self.buff[self.b_width * block_y + block_x];
        &mut block[inner_y][inner_x]
    }
}

fn remove_all_invalid_words(words: &mut Vec<String>, board: &Board<u8>) {
    let allowed_chars = calc_existing_chars(board);
    words.retain(|w| {
        if w.as_bytes().iter().any(|&c| allowed_chars[alph_to_idx(c)] == 0) {
            return false;
        }
        let mut chars = [0usize; RANGE];
        for b in w.as_bytes().iter().copied().map(alph_to_idx) {
            chars[b] += 1;
        }
        chars.iter().zip(&allowed_chars).all(|(&a, &b)| a <= b)
    });
}

#[derive(Default)]
struct TrieNode<'a> {
    children: [Option<Box<TrieNode<'a>>>; RANGE],
    word: Option<&'a str>,
}

impl<'a> TrieNode<'a> {
    fn produce(words: &'a [String]) -> Box<Self> {
        let mut root = Self::default();
        for w in words {
            let word = w.as_str();
            let bytes = word.as_bytes();
            let mut current = &mut root;
            for &b in bytes {
                let idx = alph_to_idx(b);
                if current.children[idx].is_none() {
                    current.children[idx] = Some(Box::default());
                }
                let next = current.children[idx].as_mut().unwrap().as_mut();
                current = next;
            }
            current.word = Some(word);
        }
        Box::new(root)
    }
}

fn traverse<'a>(
    pos: (usize, usize),
    board: &Board<u8>,
    visited: &mut Board<bool>,
    found: &mut HashSet<&'a str>,
    trie: &TrieNode<'a>,
) {
    let c = board[pos];
    if let Some(ref next) = trie.children[alph_to_idx(c)] {
        let next = next.as_ref();
        if let Some(w) = next.word {
            found.insert(w);
        }
        visited[pos] = true;

        if pos.0 > 0 {
            let candidate = (pos.0 - 1, pos.1);
            if !visited[candidate] {
                traverse(candidate, board, visited, found, next);
            }
        }
        if pos.1 > 0 {
            let candidate = (pos.0, pos.1 - 1);
            if !visited[candidate] {
                traverse(candidate, board, visited, found, next);
            }
        }
        if pos.0 + 1 < board.width {
            let candidate = (pos.0 + 1, pos.1);
            if !visited[candidate] {
                traverse(candidate, board, visited, found, next);
            }
        }
        if pos.1 + 1 < board.height {
            let candidate = (pos.0, pos.1 + 1);
            if !visited[candidate] {
                traverse(candidate, board, visited, found, next);
            }
        }

        visited[pos] = false;
    }
}

pub struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut words = words;
        words.sort_unstable();
        words.dedup();

        assert!(words.iter().flat_map(|w| w.as_bytes()).all(|&b| b.is_ascii_lowercase()));

        let board = from_chars(board);
        words.retain(|x| x.len() <= board.width * board.height);

        remove_all_invalid_words(&mut words, &board);

        if words.is_empty() {
            return words;
        }

        let mut result = HashSet::new();
        let trie = TrieNode::produce(&words);
        let mut visited: Board<bool> = Board::zeroed(board.width, board.height);
        for y in 0..board.height {
            for x in 0..board.width {
                if result.len() == words.len() {
                    return words;
                }
                traverse((x, y), &board, &mut visited, &mut result, trie.as_ref());
            }
        }
        result.into_iter().map(|x| x.to_string()).collect()
    }
}

#[test]
fn test_find_words() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec!["oath", "pea", "eat", "rain"];
    let words = words.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let mut res = Solution::find_words(board, words);
    assert_eq!(res.sort(), vec!["oath", "eat",].sort());
}
