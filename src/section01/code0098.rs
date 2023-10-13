#![allow(dead_code)]

// 98. Validate Binary Search Tree
// https://leetcode.com/problems/validate-binary-search-tree/
// https://leetcode.cn/problems/validate-binary-search-tree/
//
// Given a binary tree, determine if it is a valid binary search tree (BST).
//
// A valid BST is defined as follows:
//
// - The left subtree of a node contains only nodes with keys less than the node's key.
// - The right subtree of a node contains only nodes with keys greater than the node's key.
// - Both the left and right subtrees must also be binary search trees.

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type OptNode = Option<Rc<RefCell<TreeNode>>>;
        fn validate(n: &OptNode, min: &OptNode, max: &OptNode) -> Option<bool> {
            if n.is_none() {
                return Some(true);
            }
            let val = n.as_ref()?.borrow().val;
            if max.is_some() && max.as_ref()?.borrow().val <= val {
                return Some(false);
            }
            if min.is_some() && min.as_ref()?.borrow().val >= val {
                return Some(false);
            }
            let v = validate(&n.as_ref()?.borrow().left, min, n)? && validate(&n.as_ref()?.borrow().right, n, max)?;
            Some(v)
        }
        validate(&root, &None, &None).unwrap_or_default()
    }
}

#[test]
fn test() {
    let tree = TreeNode::from_vec(&[Some(2), Some(1), Some(3)]);
    assert!(Solution::is_valid_bst(tree));
}
