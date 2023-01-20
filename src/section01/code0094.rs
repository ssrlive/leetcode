#![allow(dead_code)]

// 94. Binary Tree Inorder Traversal
// https://leetcode.com/problems/binary-tree-inorder-traversal/
// https://leetcode.cn/problems/binary-tree-inorder-traversal/
//
// Given a binary tree, return the inorder traversal of its nodes' values.
//
// For example:
// Given binary tree [1,null,2,3],
//
//    1
//     \
//      2
//     /
//    3
//
// return [1,3,2].
//
// Note: Recursive solution is trivial, could you do it iteratively?

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn _inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                _inorder_traversal(&node.borrow().left, result);
                result.push(node.borrow().val);
                _inorder_traversal(&node.borrow().right, result);
            }
        }

        let mut result = Vec::new();
        _inorder_traversal(&root, &mut result);
        result
    }
}

#[test]
fn test_inorder_traversal() {
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_vec(&[Some(1), Some(2), Some(3)])),
        vec![2, 1, 3]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7)
        ])),
        vec![4, 2, 5, 1, 6, 3, 7]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9)
        ])),
        vec![8, 4, 9, 2, 5, 1, 6, 3, 7]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11)
        ])),
        vec![8, 4, 9, 2, 10, 5, 11, 1, 6, 3, 7]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13)
        ])),
        vec![8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 7]
    );
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15)
        ])),
        vec![8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
    );
}
