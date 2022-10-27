#![allow(dead_code)]

// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/
//
// Given a binary tree, find its maximum depth.
//
// The maximum depth is the number of nodes along the longest path from the root
// node down to the farthest leaf node.
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
// return its depth = 3.
//

use crate::treenode::TreeNode;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        fn _max_depth<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Option<i32> {
            if root.is_none() {
                return Some(0);
            }
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            let mut depth = 0;
            while !queue.is_empty() {
                depth += 1;
                let mut size = queue.len();
                while size > 0 {
                    let node = queue.pop_front()?;
                    if let Some(left) = node.as_ref()?.borrow().left.clone() {
                        queue.push_back(Some(left));
                    }
                    if let Some(right) = node.as_ref()?.borrow().right.clone() {
                        queue.push_back(Some(right));
                    }
                    size -= 1;
                }
            }
            Some(depth)
        }
        _max_depth(root).unwrap_or(0)
    }

    pub fn max_depth_2<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        let mut max_depth = 0;
        let mut stack = vec![(root, 0)];
        while let Some((node, mut depth)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                depth += 1;
                max_depth = max_depth.max(depth);

                stack.push((node.left.clone(), depth));
                stack.push((node.right.clone(), depth));
            }
        }
        max_depth
    }
}

#[test]
fn test_max_depth() {
    let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::max_depth_2(root), 3);
}
