#![allow(dead_code)]

// 102. Binary Tree Level Order Traversal
// https://leetcode.com/problems/binary-tree-level-order-traversal/
//
// Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).
//
// For example:
// Given binary tree [3,9,20,null,null,15,7],
//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its level order traversal as:
// [
//   [3],
//   [9,20],
//   [15,7]
// ]
//

use crate::treenode::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn _level_order(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>) -> Option<()> {
            if root.is_none() {
                return Some(());
            }
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                let mut level = Vec::new();
                for _ in 0..queue.len() {
                    let node = queue.pop_front()?;
                    level.push(node.as_ref()?.borrow().val);
                    if node.as_ref()?.borrow().left.is_some() {
                        queue.push_back(node.as_ref()?.borrow().left.clone());
                    }
                    if node.as_ref()?.borrow().right.is_some() {
                        queue.push_back(node.as_ref()?.borrow().right.clone());
                    }
                }
                result.push(level);
            }
            Some(())
        }
        let mut result = Vec::new();
        _level_order(root, &mut result);
        result
    }
}

#[test]
fn test_level_order() {
    assert_eq!(
        Solution::level_order(TreeNode::from_vec(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7)
        ])),
        vec![vec![3], vec![9, 20], vec![15, 7]]
    );
}
