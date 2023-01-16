#![allow(dead_code)]

// 988. Smallest String Starting From Leaf
// https://leetcode.com/problems/smallest-string-starting-from-leaf/
// https://leetcode.cn/problems/smallest-string-starting-from-leaf/
//
// You are given the root of a binary tree where each node has a value in the range [0, 25] representing the letters 'a' to 'z'.
//
// Return the lexicographically smallest string that starts at a leaf of this tree and ends at the root.
//
// As a reminder, any shorter prefix of a string is lexicographically smaller.
//
// For example, "ab" is lexicographically smaller than "aba".
// A leaf of a node is a node that has no children.
//
// Example 1:
//
// Input: root = [0,1,2,3,4,3,4]
// Output: "dba"
//
// Example 2:
//
// Input: root = [25,1,3,1,3,0,2]
// Output: "adz"
//
// Example 3:
//
// Input: root = [2,2,1,null,1,0,null,0]
// Output: "abc"
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 8500].
// - 0 <= Node.val <= 25
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        use std::collections::{HashMap, VecDeque};
        let mut map = HashMap::new();
        for i in 0..26 {
            map.insert(i, (b'a' + i as u8) as char);
        }
        let mut queue = VecDeque::new();
        let mut result = String::new();
        if let Some(node) = root {
            queue.push_back((node, String::new()));
        }
        while !queue.is_empty() {
            let (node, mut path) = queue.pop_front().unwrap();
            let node = node.borrow();
            path.push(map[&node.val]);
            if node.left.is_none() && node.right.is_none() {
                let path = path.chars().rev().collect::<String>();
                if result.is_empty() {
                    result = path;
                } else {
                    result = result.min(path);
                }
            }
            if let Some(left) = &node.left {
                queue.push_back((left.clone(), path.clone()));
            }
            if let Some(right) = &node.right {
                queue.push_back((right.clone(), path.clone()));
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::smallest_from_leaf(TreeNode::from_vec(&[
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(3),
            Some(4)
        ])),
        "dba".to_string()
    );
    assert_eq!(
        Solution::smallest_from_leaf(TreeNode::from_vec(&[
            Some(25),
            Some(1),
            Some(3),
            Some(1),
            Some(3),
            Some(0),
            Some(2)
        ])),
        "adz".to_string()
    );
    assert_eq!(
        Solution::smallest_from_leaf(TreeNode::from_vec(&[
            Some(2),
            Some(2),
            Some(1),
            None,
            Some(1),
            Some(0),
            None,
            Some(0)
        ])),
        "abc".to_string()
    );
}
