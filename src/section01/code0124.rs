#![allow(dead_code)]

// 124. Binary Tree Maximum Path Sum
// https://leetcode.com/problems/binary-tree-maximum-path-sum/
// https://leetcode.cn/problems/binary-tree-maximum-path-sum/
//
// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.
//
// The path sum of a path is the sum of the node's values in the path.
//
// Given the root of a binary tree, return the maximum path sum of any non-empty path.

use crate::treenode::TreeNode;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut i32) -> i32 {
            if let Some(n) = node {
                let val = n.borrow().val;
                let l = std::cmp::max(0, dfs(&n.borrow().left, answer));
                let r = std::cmp::max(0, dfs(&n.borrow().right, answer));
                *answer = std::cmp::max(*answer, val + l + r);
                val + std::cmp::max(l, r)
            } else {
                0
            }
        }

        let mut answer = std::i32::MIN;
        dfs(&root, &mut answer);
        answer
    }
}

#[test]
fn test_max_path_sum() {
    assert_eq!(
        Solution::max_path_sum(TreeNode::from_vec(&vec![Some(1), Some(2), Some(3)])),
        6
    );
    assert_eq!(
        Solution::max_path_sum(TreeNode::from_vec(&[
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7)
        ])),
        42
    );
}
