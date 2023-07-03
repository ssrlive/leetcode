#![allow(dead_code)]

// 103. Binary Tree Zigzag Level Order Traversal
// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/
//
// Given a binary tree, return the zigzag level order traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
//
// For example:
// Given binary tree [3,9,20,null,null,15,7],
//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its zigzag level order traversal as:
// [
//   [3],
//   [20,9],
//   [15,7]
// ]
//

use crate::treenode::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn _zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>) -> Option<()> {
            if root.is_none() {
                return Some(());
            }
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            let mut is_left_to_right = true;
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
                if !is_left_to_right {
                    level.reverse();
                }
                is_left_to_right = !is_left_to_right;
                result.push(level);
            }
            Some(())
        }
        let mut result = Vec::new();
        _zigzag_level_order(root, &mut result);
        result
    }
}

#[test]
fn test_zigzag_level_order() {
    assert_eq!(
        Solution::zigzag_level_order(TreeNode::from_vec(&vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
}
