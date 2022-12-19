#![allow(dead_code)]

// 814. Binary Tree Pruning
// https://leetcode.com/problems/binary-tree-pruning/
// https://leetcode.cn/problems/binary-tree-pruning/
//
// Given the root of a binary tree, return the same tree where every subtree (of the given tree) not containing a 1 has been removed.
//
// A subtree of a node node is node plus every node that is a descendant of node.
//
// Example 1:
//
// Input: root = [1,null,0,0,1]
// Output: [1,null,0,null,1]
// Explanation:
// Only the red nodes satisfy the property "every subtree not containing a 1".
// The diagram on the right represents the answer.
//
// Example 2:
//
// Input: root = [1,0,1,0,0,0,1]
// Output: [1,null,1,null,1]
//
// Example 3:
//
// Input: root = [1,1,0,1,1,0,1,0]
// Output: [1,1,0,1,1,null,1]
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 200].
// Node.val is either 0 or 1.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn solve(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let left = solve(node.borrow().left.clone());
                let right = solve(node.borrow().right.clone());
                if left.is_none() && right.is_none() && node.borrow().val == 0 {
                    return None;
                }
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
                return Some(node);
            }
            None
        }

        solve(root)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), None, Some(0), Some(0), Some(1)]);
    let ans = TreeNode::from_vec(&[Some(1), None, Some(0), None, Some(1)]);
    assert_eq!(Solution::prune_tree(root), ans);

    let root = TreeNode::from_vec(&[Some(1), Some(0), Some(1), Some(0), Some(0), Some(0), Some(1)]);
    let ans = TreeNode::from_vec(&[Some(1), None, Some(1), None, Some(1)]);
    assert_eq!(Solution::prune_tree(root), ans);

    let root = TreeNode::from_vec(&[Some(1), Some(1), Some(0), Some(1), Some(1), Some(0), Some(1), Some(0)]);
    let ans = TreeNode::from_vec(&[Some(1), Some(1), Some(0), Some(1), Some(1), None, Some(1)]);
    assert_eq!(Solution::prune_tree(root), ans);
}
