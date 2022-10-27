#![allow(dead_code)]

// 144. Binary Tree Preorder Traversal
// https://leetcode.com/problems/binary-tree-preorder-traversal/
//
// Given the root of a binary tree, return the preorder traversal of its nodes' values.
//

use super::treenode::TreeNode;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut node = root;

        while let Some(n) = node {
            result.push(n.borrow().val);
            if let Some(right) = n.borrow().right.clone() {
                stack.push(right);
            }
            node = n.borrow().left.clone();
            if node.is_none() {
                node = stack.pop();
            }
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::preorder_traversal(TreeNode::from_vec(&[Some(1), None, Some(2), Some(3)])),
        vec![1, 2, 3]
    );
}
