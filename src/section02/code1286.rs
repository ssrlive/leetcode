#![allow(dead_code)]

// 1286. Iterator for Combination
// https://leetcode.com/problems/iterator-for-combination/
// https://leetcode.cn/problems/iterator-for-combination/
//
// Medium
//
// Design the CombinationIterator class:
//
//     CombinationIterator(string characters, int combinationLength) Initializes the object with a string characters of sorted distinct lowercase English letters and a number combinationLength as arguments.
//     next() Returns the next combination of length combinationLength in lexicographical order.
//     hasNext() Returns true if and only if there exists a next combination.
//
// Example 1:
//
// Input
// ["CombinationIterator", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
// [["abc", 2], [], [], [], [], [], []]
// Output
// [null, "ab", true, "ac", true, "bc", false]
//
// Explanation
// CombinationIterator itr = new CombinationIterator("abc", 2);
// itr.next();    // return "ab"
// itr.hasNext(); // return True
// itr.next();    // return "ac"
// itr.hasNext(); // return True
// itr.next();    // return "bc"
// itr.hasNext(); // return False
//
// Constraints:
//
// -    1 <= combinationLength <= characters.length <= 15
// -    All the characters of characters are unique.
// -    At most 104 calls will be made to next and hasNext.
// -    It is guaranteed that all calls of the function next are valid.
//

use std::collections::VecDeque;

struct CombinationIterator {
    queue: VecDeque<String>,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let mut queue = VecDeque::new();
        let mut chars = characters.chars().collect::<Vec<char>>();
        chars.sort();
        let mut stack = Vec::new();
        Self::dfs(&mut queue, &mut stack, &chars, combination_length as usize);
        Self { queue }
    }

    fn dfs(queue: &mut VecDeque<String>, stack: &mut Vec<char>, chars: &[char], k: usize) {
        if stack.len() == k {
            queue.push_back(stack.iter().collect());
            return;
        }
        for i in 0..chars.len() {
            if stack.contains(&chars[i]) {
                continue;
            }
            if !stack.is_empty() && chars[i] < *stack.last().unwrap() {
                continue;
            }
            stack.push(chars[i]);
            Self::dfs(queue, stack, chars, k);
            stack.pop();
        }
    }

    fn next(&mut self) -> String {
        self.queue.pop_front().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}

#[test]
fn test() {
    let mut obj = CombinationIterator::new("abc".to_string(), 2);
    assert_eq!(obj.next(), "ab".to_string());
    assert!(obj.has_next());
    assert_eq!(obj.next(), "ac".to_string());
    assert!(obj.has_next());
    assert_eq!(obj.next(), "bc".to_string());
    assert!(!obj.has_next());
}
