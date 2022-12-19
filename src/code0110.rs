#![allow(dead_code)]

// 110. Balanced Binary Tree
// https://leetcode.com/problems/balanced-binary-tree/
// https://leetcode.cn/problems/balanced-binary-tree/
//
// Given a binary tree, determine if it is height-balanced.
//
// For this problem, a height-balanced binary tree is defined as:
//
// a binary tree in which the left and right subtrees of every node differ in
// height by no more than 1.
//
// Example 1:
//
// Given the following tree [3,9,20,null,null,15,7]:
//
//     3
//    / \
//   9  20
//     /  \
//    15   7
//
// Return true.
//
// Example 2:
//
// Given the following tree [1,2,2,3,3,null,null,4,4]:
//
//        1
//       / \
//      2   2
//     / \
//    3   3
//   / \
//  4   4
//
// Return false.
//

use crate::treenode::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn _is_balanced(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            if let Some(node) = node {
                let node = node.borrow();
                match (_is_balanced(&node.left), _is_balanced(&node.right)) {
                    (Some(h1), Some(h2)) if (h1 - h2).abs() <= 1 => Some(h1.max(h2) + 1),
                    _ => None,
                }
            } else {
                Some(0)
            }
        }

        _is_balanced(&root).is_some()
    }
}

#[test]
fn test_is_balanced() {
    assert_eq!(
        Solution::is_balanced(TreeNode::from_vec(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7)
        ])),
        true
    );
    assert_eq!(
        Solution::is_balanced(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4)
        ])),
        false
    );
}
