#![allow(dead_code)]

// 965. Univalued Binary Tree
// https://leetcode.com/problems/univalued-binary-tree/
// https://leetcode.cn/problems/univalued-binary-tree/
//
// A binary tree is uni-valued if every node in the tree has the same value.
//
// Given the root of a binary tree, return true if the given tree is uni-valued, or false otherwise.
//
// Example 1:
//
// Input: root = [1,1,1,1,1,null,1]
// Output: true
//
// Example 2:
//
// Input: root = [2,2,2,5,2]
// Output: false
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 100].
// - 0 <= Node.val < 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn _is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(node) = root {
                let val = node.borrow().val;
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if let Some(left) = left {
                    if left.borrow().val != val {
                        return false;
                    }
                }
                if let Some(right) = right {
                    if right.borrow().val != val {
                        return false;
                    }
                }
                return _is_unival_tree(node.borrow().left.clone()) && _is_unival_tree(node.borrow().right.clone());
            }
            true
        }
        _is_unival_tree(root)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(1), Some(1), Some(1), Some(1), None, Some(1)]);
    assert!(Solution::is_unival_tree(root));
    let root = TreeNode::from_vec(&[Some(2), Some(2), Some(2), Some(5), Some(2)]);
    assert!(!Solution::is_unival_tree(root));
}
