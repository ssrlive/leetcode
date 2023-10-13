#![allow(dead_code)]

// 100. Same Tree
// https://leetcode.com/problems/same-tree/
// https://leetcode.cn/problems/same-tree/
//
// Given two binary trees, write a function to check if they are equal or not.
//
// Two binary trees are considered equal if they are structurally identical and
// the nodes have the same value.
//

use crate::treenode::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        pub fn _is_same_tree(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    p.val == q.val && _is_same_tree(&p.left, &q.left) && _is_same_tree(&p.right, &q.right)
                }
                _ => false,
            }
        }
        _is_same_tree(&p, &q)
    }
}

#[test]
fn test() {
    assert!(Solution::is_same_tree(
        TreeNode::from_vec(&[Some(1), Some(2), Some(3)]),
        TreeNode::from_vec(&[Some(1), Some(2), Some(3)])
    ));
    assert!(!Solution::is_same_tree(
        TreeNode::from_vec(&[Some(1), Some(2)]),
        TreeNode::from_vec(&[Some(1), None, Some(2)])
    ));
    assert!(!Solution::is_same_tree(
        TreeNode::from_vec(&[Some(1), Some(2), Some(1)]),
        TreeNode::from_vec(&[Some(1), Some(1), Some(2)])
    ));
}
