#![allow(dead_code)]

// 226. Invert Binary Tree
// https://leetcode.com/problems/invert-binary-tree/
//
// Given the root of a binary tree, invert the tree, and return its root.
//
// Example:
//
// Input:                Output:
//
//      4                     4
//    /   \                 /   \
//   2     7       =>      7     2
//  / \   / \             / \   / \
// 1   3 6   9           9   6 3   1
//
// Constraints:
//
// - The number of nodes in the tree is in the range [0, 100].
// - -100 <= Node.val <= 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        pub fn invert_tree_recurse(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(ref mut node) = root {
                let mut node = node.borrow_mut();
                let left = node.left.take();
                let right = node.right.take();
                node.left = invert_tree_recurse(right);
                node.right = invert_tree_recurse(left);
            }
            root
        }
        invert_tree_recurse(root)
    }
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)];
    let root = TreeNode::from_vec(&v1);
    let root = Solution::invert_tree(root);
    let v = root.as_ref().ok_or("")?.borrow().to_vec();
    let v2 = vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)];
    assert_eq!(v, v2);

    let v1 = vec![Some(2), Some(1), Some(3)];
    let root = TreeNode::from_vec(&v1);
    let root = Solution::invert_tree(root);
    let v = root.as_ref().ok_or("")?.borrow().to_vec();
    let v2 = vec![Some(2), Some(3), Some(1)];
    assert_eq!(v, v2);
    Ok(())
}
