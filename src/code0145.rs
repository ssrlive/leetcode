#![allow(dead_code)]

// 145. Binary Tree Postorder Traversal
// https://leetcode.com/problems/binary-tree-postorder-traversal/
// https://leetcode.cn/problems/binary-tree-postorder-traversal/
//
// Given the root of a binary tree, return the postorder traversal of its nodes' values.
//

use super::treenode::TreeNode;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut node = root;

        while let Some(n) = node {
            result.push(n.borrow().val);
            if let Some(left) = n.borrow().left.clone() {
                stack.push(left);
            }
            node = n.borrow().right.clone();
            if node.is_none() {
                node = stack.pop();
            }
        }

        result.reverse();
        result
    }
}

#[test]
fn test_postorder_traversal() {
    assert_eq!(
        Solution::postorder_traversal(TreeNode::from_vec(&[Some(1), None, Some(2), Some(3)])),
        vec![3, 2, 1]
    );
}
