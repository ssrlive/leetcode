#![allow(dead_code)]

// 968. Binary Tree Cameras
// https://leetcode.com/problems/binary-tree-cameras/
// https://leetcode.cn/problems/binary-tree-cameras/
//
// You are given the root of a binary tree. We install cameras on the tree nodes where each camera
// at a node can monitor its parent, itself, and its immediate children.
//
// Return the minimum number of cameras needed to monitor all nodes of the tree.
//
// Example 1:
//
// Input: root = [0,0,null,0,0]
// Output: 1
// Explanation: One camera is enough to monitor all nodes if placed as shown.
//
// Example 2:
//
// Input: root = [0,0,null,0,null,0,null,null,0]
// Output: 2
// Explanation: At least two cameras are needed to monitor all nodes of the tree. The above image shows one of the valid configurations of camera placement.
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 1000].
// Node.val == 0
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        enum State {
            Uncovered,
            Covered,
            HasCamera,
        }

        fn dfs(node: Rc<RefCell<TreeNode>>) -> (State, i32) {
            let mut node_ref = node.borrow_mut();
            match (node_ref.left.take(), node_ref.right.take()) {
                (None, None) => (State::Uncovered, 0),
                (None, Some(child)) | (Some(child), None) => match dfs(child) {
                    (State::Uncovered, n) => (State::HasCamera, n + 1),
                    (State::Covered, n) => (State::Uncovered, n),
                    (State::HasCamera, n) => (State::Covered, n),
                },
                (Some(child1), Some(child2)) => match (dfs(child1), dfs(child2)) {
                    ((State::Uncovered, n1), (_, n2)) | ((_, n1), (State::Uncovered, n2)) => {
                        (State::HasCamera, n1 + n2 + 1)
                    }
                    ((State::HasCamera, n1), (_, n2)) | ((_, n1), (State::HasCamera, n2)) => (State::Covered, n1 + n2),
                    ((_, n1), (_, n2)) => (State::Uncovered, n1 + n2),
                },
            }
        }

        match dfs(root.unwrap()) {
            (State::Uncovered, n) => n + 1,
            (_, n) => n,
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (TreeNode::from_vec(&[Some(0), Some(0), None, Some(0), Some(0)]), 1),
        (
            TreeNode::from_vec(&[Some(0), Some(0), None, Some(0), None, Some(0), None, None, Some(0)]),
            2,
        ),
    ];
    for (root, expected) in cases {
        assert_eq!(Solution::min_camera_cover(root), expected);
    }
}
