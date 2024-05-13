#![allow(dead_code)]

// 1038. Binary Search Tree to Greater Sum Tree
// https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
// https://leetcode.cn/problems/binary-search-tree-to-greater-sum-tree/
//
// Given the root of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the
// original BST is changed to the original key plus the sum of all keys greater than the original key in BST.
//
// As a reminder, a binary search tree is a tree that satisfies these constraints:
//
// The left subtree of a node contains only nodes with keys less than the node's key.
// The right subtree of a node contains only nodes with keys greater than the node's key.
// Both the left and right subtrees must also be binary search trees.
//
// Example 1:
//
// Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
// Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
//
// Example 2:
//
// Input: root = [0,null,1]
// Output: [1,null,1]
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 100].
// - 0 <= Node.val <= 100
// - All the values in the tree are unique.
//
// Note: This question is the same as 538: https://leetcode.com/problems/convert-bst-to-greater-tree/
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        let mut stack = vec![];
        let mut node = root.clone();
        #[allow(clippy::assigning_clones)]
        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                stack.push(n.clone());
                node = n.borrow().right.clone();
            }
            if let Some(n) = stack.pop() {
                sum += n.borrow().val;
                n.borrow_mut().val = sum;
                node = n.borrow().left.clone();
            }
        }
        root
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(4),
        Some(1),
        Some(6),
        Some(0),
        Some(2),
        Some(5),
        Some(7),
        None,
        None,
        None,
        Some(3),
        None,
        None,
        None,
        Some(8),
    ]);
    let root = Solution::bst_to_gst(root);
    let root = TreeNode::to_vec(&root);
    assert_eq!(
        root,
        vec![
            Some(30),
            Some(36),
            Some(21),
            Some(36),
            Some(35),
            Some(26),
            Some(15),
            None,
            None,
            None,
            Some(33),
            None,
            None,
            None,
            Some(8)
        ]
    );

    let root = TreeNode::from_vec(&[Some(0), None, Some(1)]);
    let root = Solution::bst_to_gst(root);
    let root = TreeNode::to_vec(&root);
    assert_eq!(root, vec![Some(1), None, Some(1)]);
}
