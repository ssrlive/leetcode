#![allow(dead_code)]

// 107. Binary Tree Level Order Traversal II
// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
//
// Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
//
// For example:
// Given binary tree [3,9,20,null,null,15,7],
//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its bottom-up level order traversal as:
// [
//   [15,7],
//   [9,20],
//   [3]
// ]

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order_bottom<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<Vec<T>> {
        fn helper<T: Copy>(
            root: Option<Rc<RefCell<TreeNode<T>>>>,
            level: usize,
            res: &mut Vec<Vec<T>>,
        ) {
            if let Some(node) = root {
                if res.len() <= level {
                    res.push(vec![]);
                }
                res[level].push(node.borrow().val);
                helper(node.borrow().left.clone(), level + 1, res);
                helper(node.borrow().right.clone(), level + 1, res);
            }
        }

        let mut res = vec![];
        helper(root, 0, &mut res);
        res.reverse();
        res
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let res = Solution::level_order_bottom(root);
    assert_eq!(res, vec![vec![15, 7], vec![9, 20], vec![3]]);
}
