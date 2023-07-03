#![allow(dead_code)]

/*

// 2213. Longest Substring of One Repeating Character
// https://leetcode.com/problems/longest-substring-of-one-repeating-character/
// https://leetcode.cn/problems/longest-substring-of-one-repeating-character/
//
// Hard
//
// You are given a 0-indexed string s. You are also given a 0-indexed string queryCharacters of length k and a 0-indexed array of integer indices queryIndices of length k, both of which are used to describe k queries.

The ith query updates the character in s at index queryIndices[i] to the character queryCharacters[i].

Return an array lengths of length k where lengths[i] is the length of the longest substring of s consisting of only one repeating character after the ith query is performed.

Example 1:

Input: s = "babacc", queryCharacters = "bcb", queryIndices = [1,3,3]
Output: [3,3,4]
Explanation:
- 1st query updates s = "bbbacc". The longest substring consisting of one repeating character is "bbb" with length 3.
- 2nd query updates s = "bbbccc".
  The longest substring consisting of one repeating character can be "bbb" or "ccc" with length 3.
- 3rd query updates s = "bbbbcc". The longest substring consisting of one repeating character is "bbbb" with length 4.
Thus, we return [3,3,4].

Example 2:

Input: s = "abyzz", queryCharacters = "aa", queryIndices = [2,1]
Output: [2,3]
Explanation:
- 1st query updates s = "abazz". The longest substring consisting of one repeating character is "zz" with length 2.
- 2nd query updates s = "aaazz". The longest substring consisting of one repeating character is "aaa" with length 3.
Thus, we return [2,3].

Constraints:

    1 <= s.length <= 10^5
    s consists of lowercase English letters.
    k == queryCharacters.length == queryIndices.length
    1 <= k <= 10^5
    queryCharacters consists of lowercase English letters.
    0 <= queryIndices[i] < s.length
*/

struct Solution;

impl Solution {
    pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
        let characters: Vec<char> = s.chars().collect();
        let query_characters: Vec<char> = query_characters.chars().collect();
        let mut res = vec![0; query_characters.len()];
        let mut segment_tree = SegmentTree::construct(&characters);
        for query_index in 0..query_characters.len() {
            segment_tree.update(query_indices[query_index] as usize, query_characters[query_index]);
            res[query_index] = segment_tree.longest_repeating() as i32;
        }
        res
    }
}

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    left_index: usize,
    right_index: usize,
    longest_substring_len: usize,
    left_char: char,
    right_char: char,
    left_char_rep_len: usize,
    right_char_rep_len: usize,
    left_node: Option<Rc<RefCell<Node>>>,
    right_node: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn construct(characters: &Vec<char>, left_index: usize, right_index: usize) -> Node {
        if left_index == right_index {
            Node {
                left_index,
                right_index,
                longest_substring_len: 1,
                left_char: characters[left_index],
                right_char: characters[right_index],
                left_char_rep_len: 1,
                right_char_rep_len: 1,
                left_node: None,
                right_node: None,
            }
        } else {
            let mid_index = (left_index + right_index) / 2;
            let left_node = Rc::new(RefCell::new(Node::construct(characters, left_index, mid_index)));
            let right_node = Rc::new(RefCell::new(Node::construct(characters, mid_index + 1, right_index)));
            Self::merge(left_node, right_node)
        }
    }

    fn merge(left_node: Rc<RefCell<Node>>, right_node: Rc<RefCell<Node>>) -> Node {
        let mut longest_substring_len = std::cmp::max(left_node.borrow().longest_substring_len, right_node.borrow().longest_substring_len);
        let merge_len = if left_node.borrow().right_char == right_node.borrow().left_char {
            left_node.borrow().right_char_rep_len + right_node.borrow().left_char_rep_len
        } else {
            0
        };
        longest_substring_len = std::cmp::max(longest_substring_len, merge_len);

        let left_char_rep_len = if left_node.borrow().right_index - left_node.borrow().left_index + 1
            == left_node.borrow().left_char_rep_len
            && left_node.borrow().right_char == right_node.borrow().left_char
        {
            left_node.borrow().left_char_rep_len + right_node.borrow().left_char_rep_len
        } else {
            left_node.borrow().left_char_rep_len
        };

        let right_char_rep_len = if right_node.borrow().right_index - right_node.borrow().left_index + 1
            == right_node.borrow().right_char_rep_len
            && left_node.borrow().right_char == right_node.borrow().left_char
        {
            right_node.borrow().right_char_rep_len + left_node.borrow().right_char_rep_len
        } else {
            right_node.borrow().right_char_rep_len
        };

        Node {
            left_index: left_node.borrow().left_index,
            right_index: right_node.borrow().right_index,
            longest_substring_len,
            left_char: left_node.borrow().left_char,
            right_char: right_node.borrow().right_char,
            left_char_rep_len,
            right_char_rep_len,
            left_node: Some(left_node.clone()),
            right_node: Some(right_node.clone()),
        }
    }

    fn update(&mut self, index: usize, character: char) {
        if self.left_index == self.right_index {
            self.left_char = character;
            self.right_char = character;
        } else {
            let l_node = self.left_node.clone().unwrap();
            let r_node = self.right_node.clone().unwrap();
            if index <= l_node.borrow().right_index {
                l_node.borrow_mut().update(index, character)
            } else {
                r_node.borrow_mut().update(index, character)
            }

            *self = Node::merge(l_node, r_node)
        }
    }
}

#[derive(Debug)]
struct SegmentTree {
    node: Node,
}

impl SegmentTree {
    fn construct(characters: &Vec<char>) -> SegmentTree {
        SegmentTree {
            node: Node::construct(characters, 0, characters.len() - 1),
        }
    }

    fn update(&mut self, index: usize, character: char) {
        self.node.update(index, character)
    }

    fn longest_repeating(&self) -> usize {
        self.node.longest_substring_len
    }
}

#[test]
fn test() {
    let cases = vec![
        ("babacc", "bcb", vec![1, 3, 3], vec![3, 3, 4]),
        ("abyzz", "aa", vec![2, 1], vec![2, 3]),
    ];
    for (s, q, idx, expect) in cases {
        assert_eq!(Solution::longest_repeating(s.to_string(), q.to_string(), idx), expect);
    }
}
