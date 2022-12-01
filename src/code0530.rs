#![allow(dead_code)]

// 530. Minimum Absolute Difference in BST
// https://leetcode.com/problems/minimum-absolute-difference-in-bst/
//
// Given the root of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.
//
// Example 1:
//
// Input: root = [4,2,6,1,3]
// Output: 1
//
// Example 2:
//
// Input: root = [1,0,48,null,null,12,49]
// Output: 1
//
// Constraints:
//
// - The number of nodes in the tree is in the range [2, 104].
// - 0 <= Node.val <= 105
//
// Note: This question is the same as 783: https://leetcode.com/problems/minimum-distance-between-bst-nodes/
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        let mut prev = None;
        Self::inorder(root, &mut prev, &mut min);
        min
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder(node.left.clone(), prev, min);
            if let Some(p) = prev {
                *min = (*min).min(node.val - *p);
            }
            *prev = Some(node.val);
            Self::inorder(node.right.clone(), prev, min);
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(4), Some(2), Some(6), Some(1), Some(3)]);
    assert_eq!(Solution::get_minimum_difference(root), 1);

    let root = TreeNode::from_vec(&[Some(1), Some(0), Some(48), None, None, Some(12), Some(49)]);
    assert_eq!(Solution::get_minimum_difference(root), 1);
}
