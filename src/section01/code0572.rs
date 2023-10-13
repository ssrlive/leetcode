#![allow(dead_code)]

// 572. Subtree of Another Tree
// https://leetcode.com/problems/subtree-of-another-tree/
// https://leetcode.cn/problems/subtree-of-another-tree/
//
// Given the roots of two binary trees root and subRoot, return true if there is a subtree of root
// with the same structure and node values of subRoot and false otherwise.
//
// A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants.
// The tree tree could also be considered as a subtree of itself.
//
// Example 1:
//
// Input: root = [3,4,5,1,2], subRoot = [4,1,2]
// Output: true
//
// Example 2:
//
// Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
// Output: false
//
// Constraints:
//
// - The number of nodes in the root tree is in the range [1, 2000].
// - The number of nodes in the subRoot tree is in the range [1, 1000].
// - -10^4 <= root.val <= 10^4
// - -10^4 <= subRoot.val <= 10^4
//
// Follow up: Could you check for a subtree with O(n) time complexity?

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::_is_subtree(root, sub_root).unwrap_or(false)
    }

    fn _is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> Option<bool> {
        if let Some(root) = root {
            if Self::is_same_tree(Some(root.clone()), sub_root.clone()) {
                return Some(true);
            }

            if Self::_is_subtree(root.borrow().left.clone(), sub_root.clone()).unwrap_or(false) {
                return Some(true);
            }

            if Self::_is_subtree(root.borrow().right.clone(), sub_root).unwrap_or(false) {
                return Some(true);
            }
        }

        Some(false)
    }

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::_is_same_tree(p, q).unwrap_or(false)
    }

    fn _is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<bool> {
        if let Some(p) = p {
            if let Some(q) = q {
                if p.borrow().val != q.borrow().val {
                    return Some(false);
                }

                if !Self::_is_same_tree(p.borrow().left.clone(), q.borrow().left.clone()).unwrap_or(false) {
                    return Some(false);
                }

                if !Self::_is_same_tree(p.borrow().right.clone(), q.borrow().right.clone()).unwrap_or(false) {
                    return Some(false);
                }
            } else {
                return Some(false);
            }
        } else if q.is_some() {
            return Some(false);
        }

        Some(true)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(3), Some(4), Some(5), Some(1), Some(2)]);
    let sub_root = TreeNode::from_vec(&[Some(4), Some(1), Some(2)]);
    assert!(Solution::is_subtree(root, sub_root));

    let root = TreeNode::from_vec(&[Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, None, Some(0)]);
    let sub_root = TreeNode::from_vec(&[Some(4), Some(1), Some(2)]);
    assert!(!Solution::is_subtree(root, sub_root));
}
