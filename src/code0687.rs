#![allow(dead_code)]

// 687. Longest Univalue Path
// https://leetcode.com/problems/longest-univalue-path/
//
// Given the root of a binary tree, return the length of the longest path, where each node in the path has the same value.
// This path may or may not pass through the root.
//
// The length of the path between two nodes is represented by the number of edges between them.
//
// Example 1:
//
// Input: root = [5,4,5,1,1,null,5]
// Output: 2
// Explanation: The shown image shows that the longest path of the same value (i.e. 5).
//
// Example 2:
//
// Input: root = [1,4,5,4,4,null,5]
// Output: 2
// Explanation: The shown image shows that the longest path of the same value (i.e. 4).
//
// Constraints:
//
// - The number of nodes in the tree is in the range [0, 10^4].
// - -1000 <= Node.val <= 1000
// - The depth of the tree will not exceed 1000.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::dfs(root.as_ref(), &mut max);
        max
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::dfs(node.left.as_ref(), max);
            let right = Self::dfs(node.right.as_ref(), max);
            let left = if node.left.as_ref().map_or(false, |n| n.borrow().val == node.val) {
                left + 1
            } else {
                0
            };
            let right = if node.right.as_ref().map_or(false, |n| n.borrow().val == node.val) {
                right + 1
            } else {
                0
            };
            *max = (*max).max(left + right);
            left.max(right)
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(5), Some(4), Some(5), Some(1), Some(1), None, Some(5)]);
    assert_eq!(Solution::longest_univalue_path(root), 2);

    let root = TreeNode::from_vec(&[Some(1), Some(4), Some(5), Some(4), Some(4), None, Some(5)]);
    assert_eq!(Solution::longest_univalue_path(root), 2);
}
