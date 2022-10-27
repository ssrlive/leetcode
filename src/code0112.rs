#![allow(dead_code)]

// 112. Path Sum
// https://leetcode.com/problems/path-sum/
//
// Given a binary tree and a sum, determine if the tree has a root-to-leaf path
// such that adding up all the values along the path equals the given sum.
//
// Note: A leaf is a node with no children.
//
// Example:
//
// Given the below binary tree and sum = 22,
//
//       5
//      / \
//     4   8
//    /   / \
//   11  13  4
//  /  \      \
// 7    2      1
//
// return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
//
// Example 2:
//
// Given the below binary tree and sum = 1,
//
//       1
//      / \
//     2   3
//
// return false.

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum<T: std::ops::Sub<Output = T> + PartialEq + Copy>(
        root: Option<Rc<RefCell<TreeNode<T>>>>,
        sum: T,
    ) -> bool {
        fn _has_path_sum<T: std::ops::Sub<Output = T> + PartialEq + Copy>(
            node: &Option<Rc<RefCell<TreeNode<T>>>>,
            sum: T,
        ) -> bool {
            if let Some(node) = node {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    return node.val == sum;
                }
                _has_path_sum(&node.left, sum - node.val)
                    || _has_path_sum(&node.right, sum - node.val)
            } else {
                false
            }
        }
        _has_path_sum(&root, sum)
    }
}

#[test]
fn test_has_path_sum() {
    let root = TreeNode::from_vec(&[
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(13),
        Some(4),
        Some(7),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        Some(1),
    ]);
    assert_eq!(Solution::has_path_sum(root, 22), true);
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::has_path_sum(root, 5), false);
}
