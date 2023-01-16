#![allow(dead_code)]

// 623. Add One Row to Tree
// https://leetcode.com/problems/add-one-row-to-tree/
// https://leetcode.cn/problems/add-one-row-to-tree/
//
// Given the root of a binary tree and two integers val and depth, add a row of nodes with value val at the given depth depth.
//
// Note that the root node is at depth 1.
//
// The adding rule is:
//
// - Given the integer depth, for each not null tree node cur at the depth depth - 1,
//   create two tree nodes with value val as cur's left subtree root and right subtree root.
// - cur's original left subtree should be the left subtree of the new left subtree root.
// - cur's original right subtree should be the right subtree of the new right subtree root.
// - If depth == 1 that means there is no depth depth - 1 at all, then create a tree node with value val as
//   the new root of the whole original tree, and the original tree is the new root's left subtree.
//
// Example 1:
//
// Input: root = [4,2,6,3,1,5], val = 1, depth = 2
// Output: [4,1,1,2,null,null,6,3,1,5]
//
// Example 2:
//
// Input: root = [4,2,null,3,1], val = 1, depth = 3
// Output: [4,2,null,1,1,3,null,null,1]
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 10^4].
// - The depth of the tree is in the range [1, 10^4].
// - -100 <= Node.val <= 100
// - -10^5 <= val <= 10^5
// - 1 <= depth <= the depth of tree + 1
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut node = TreeNode::new(val);
            node.left = root;

            return Some(Rc::new(RefCell::new(node)));
        }

        let mut queue = vec![(root.clone(), 1)];
        while !queue.is_empty() {
            let (node, level) = queue.remove(0);
            if let Some(node) = node {
                if level == depth - 1 {
                    let mut node = node.borrow_mut();

                    let mut left = TreeNode::new(val);
                    left.left = node.left.take();
                    node.left = Some(Rc::new(RefCell::new(left)));

                    let mut right = TreeNode::new(val);
                    right.right = node.right.take();
                    node.right = Some(Rc::new(RefCell::new(right)));
                } else {
                    queue.push((node.borrow().left.clone(), level + 1));
                    queue.push((node.borrow().right.clone(), level + 1));
                }
            }
        }

        root
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)]);
    let val = 1;
    let depth = 2;
    let expected = TreeNode::from_vec(&[
        Some(4),
        Some(1),
        Some(1),
        Some(2),
        None,
        None,
        Some(6),
        Some(3),
        Some(1),
        Some(5),
    ]);
    assert_eq!(Solution::add_one_row(root, val, depth), expected);

    let root = TreeNode::from_vec(&[Some(4), Some(2), None, Some(3), Some(1)]);
    let val = 1;
    let depth = 3;
    let expected = TreeNode::from_vec(&[Some(4), Some(2), None, Some(1), Some(1), Some(3), None, None, Some(1)]);
    assert_eq!(Solution::add_one_row(root, val, depth), expected);
}
