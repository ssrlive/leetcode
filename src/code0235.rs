#![allow(dead_code)]

// 235. Lowest Common Ancestor of a Binary Search Tree
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
//
// Given a binary search tree (BST), find the lowest common ancestor (LCA) of two given nodes in the BST.
//
// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two
// nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node
// to be a descendant of itself).”
//
// Given binary search tree:  root = [6,2,8,0,4,7,9,null,null,3,5]
//
// Example 1:
// Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
// Output: 6
// Explanation: The LCA of nodes 2 and 8 is 6.
//
// Example 2:
// Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
// Output: 2
// Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself
//              according to the LCA definition.
//
// Note:
// All of the nodes' values will be unique.
// p and q are different and both values will exist in the BST.

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn _recurse_impl(root: &Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(root_node) = root {
                let root_node = root_node.borrow();
                let value = root_node.val;
                if value > max {
                    return _recurse_impl(&root_node.left, min, max);
                }
                if value < min {
                    return _recurse_impl(&root_node.right, min, max);
                }
                return root.clone();
            }
            None
        }

        let p = p?.borrow().val;
        let q = q?.borrow().val;
        let (min, max) = if p < q { (p, q) } else { (q, p) };
        _recurse_impl(&root, min, max)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ]);
    let p = TreeNode::from_vec(&[Some(2)]);
    let q = TreeNode::from_vec(&[Some(8)]);
    let result = Solution::lowest_common_ancestor(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 6);

    let p = TreeNode::from_vec(&[Some(2)]);
    let q = TreeNode::from_vec(&[Some(3)]);
    let result = Solution::lowest_common_ancestor(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 2);
}
