#![allow(dead_code)]

// 111. Minimum Depth of Binary Tree
// https://leetcode.com/problems/minimum-depth-of-binary-tree/
//
// Given a binary tree, find its minimum depth.
//
// The minimum depth is the number of nodes along the shortest path from the
// root node down to the nearest leaf node.
//
// Note: A leaf is a node with no children.
//
// Example:
//
// Given binary tree [3,9,20,null,null,15,7],
//
//     3
//    / \
//   9  20
//     /  \
//    15   7
//
// return its minimum depth = 2.
//
// Example 2:
//
// Given binary tree [2,null,3,null,4,null,5,null,6],
//
//      2
//       \
//        3
//         \
//          4
//           \
//            5
//             \
//              6
//
// return its minimum depth = 5.
//
// Note: A leaf is a node with no children.

use crate::treenode::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn _min_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            if let Some(node) = node {
                let node = node.borrow();
                match (_min_depth(&node.left), _min_depth(&node.right)) {
                    (Some(h1), Some(h2)) => Some(h1.min(h2) + 1),
                    (Some(h1), None) => Some(h1 + 1),
                    (None, Some(h2)) => Some(h2 + 1),
                    (None, None) => Some(1),
                }
            } else {
                None
            }
        }

        _min_depth(&root).unwrap_or(0)
    }
}

#[test]
fn test_min_depth() {
    assert_eq!(
        Solution::min_depth(TreeNode::from_vec(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7)
        ])),
        2
    );
    assert_eq!(
        Solution::min_depth(TreeNode::from_vec(&[
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6)
        ])),
        5
    );
}
