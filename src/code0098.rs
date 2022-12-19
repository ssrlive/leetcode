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
        fn validate(
            n: Option<Rc<RefCell<TreeNode>>>,
            min: Option<Rc<RefCell<TreeNode>>>,
            max: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match n {
                None => true,
                Some(n) => {
                    if let Some(max) = max.clone() {
                        if max.borrow().val <= n.borrow().val {
                            return false;
                        }
                    }
                    if let Some(min) = min.clone() {
                        if min.borrow().val >= n.borrow().val {
                            return false;
                        }
                    }
                    validate(n.borrow().left.clone(), min, Some(n.clone()))
                        && validate(n.borrow().right.clone(), Some(n.clone()), max)
                }
            }
        }
        validate(root, None, None)
    }
}

#[test]
fn test() {
    let tree = TreeNode::from_vec(&[Some(2), Some(1), Some(3)]);
    assert_eq!(Solution::is_valid_bst(tree), true);
}
