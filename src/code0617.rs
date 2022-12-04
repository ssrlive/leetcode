#![allow(dead_code)]

// 617. Merge Two Binary Trees
// https://leetcode.com/problems/merge-two-binary-trees/
//
// You are given two binary trees root1 and root2.
//
// Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.
//
// Return the merged tree.
//
// Note: The merging process must start from the root nodes of both trees.
//
// Example 1:
//
// Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
// Output: [3,4,5,5,4,null,7]
//
// Example 2:
//
// Input: root1 = [1], root2 = [1,2]
// Output: [2,2]
//
// Constraints:
//
// - The number of nodes in both trees is in the range [0, 2000].
// - -10^4 <= Node.val <= 10^4
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let mut node1 = node1.borrow_mut();
                let mut node2 = node2.borrow_mut();
                let mut node = TreeNode::new(node1.val + node2.val);
                node.left = Solution::merge_trees(node1.left.take(), node2.left.take());
                node.right = Solution::merge_trees(node1.right.take(), node2.right.take());
                Some(Rc::new(RefCell::new(node)))
            }
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (None, None) => None,
        }
    }
}

#[test]
fn test() {
    let root1 = TreeNode::from_vec(&[Some(1), Some(3), Some(2), Some(5)]);
    let root2 = TreeNode::from_vec(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(7)]);
    let root = TreeNode::from_vec(&[Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)]);
    assert_eq!(Solution::merge_trees(root1, root2), root);
}
