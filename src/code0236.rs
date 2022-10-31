#![allow(dead_code)]

// 236. Lowest Common Ancestor of a Binary Tree
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
//
// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
//
// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined
// between two nodes p and q as the lowest node in T that has both p and q as descendants
// (where we allow a node to be a descendant of itself).”
//
// Example 1:
// Given the following binary tree:  root = [3,5,1,6,2,0,8,null,null,7,4]
//
//         3
//        / \
//       5   1
//      / \ / \
//     6  2 0  8
//       / \
//      7   4
//
// Input: root, p = 5, q = 1
// Output: 3
// Explanation: The LCA of nodes 5 and 1 is 3.
//
// Example 2:
// Given the following binary tree:  root = [3,5,1,6,2,0,8,null,null,7,4]
//
//         3
//        / \
//       5   1
//      / \ / \
//     6  2 0  8
//       / \
//      7   4
//
// Input: root, p = 5, q = 4
// Output: 5
// Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself
// according to the LCA definition.
//
// Note:
// All of the nodes' values will be unique.
// p and q are different and both values will exist in the binary tree.

use crate::treenode::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        pub fn _lowest_common_ancestor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            root.as_ref()?;
            if root == p || root == q {
                return root;
            }
            let rt = root.clone();
            let l =
                _lowest_common_ancestor(rt?.as_ref().borrow().left.clone(), p.clone(), q.clone());
            if l.is_some() {
                let rt = root.clone();
                let r = _lowest_common_ancestor(rt?.as_ref().borrow().right.clone(), p, q);
                if r.is_some() {
                    root
                } else {
                    l
                }
            } else {
                _lowest_common_ancestor(root?.as_ref().borrow().right.clone(), p, q)
            }
        }

        _lowest_common_ancestor(root, p, q)
    }
}

#[test]
fn test_lowest_common_ancestor() {
    let root = TreeNode::from_vec(&[
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);

    let node5 = root.clone().unwrap().borrow().find_node(5);
    let node1 = root.clone().unwrap().borrow().find_node(1);
    let node4 = root.clone().unwrap().borrow().find_node(4);

    let node = Solution::lowest_common_ancestor(root.clone(), node5.clone(), node1.clone());
    let val = node.unwrap().borrow().val;
    assert_eq!(val, 3);

    let node = Solution::lowest_common_ancestor(root.clone(), node5.clone(), node4.clone());
    let val = node.unwrap().borrow().val;
    assert_eq!(val, 5);
}
