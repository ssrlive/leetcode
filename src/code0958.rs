#![allow(dead_code)]

// 958. Check Completeness of a Binary Tree
// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
// https://leetcode.cn/problems/check-completeness-of-a-binary-tree/
//
// Given the root of a binary tree, determine if it is a complete binary tree.
//
// In a complete binary tree, every level, except possibly the last, is completely filled,
// and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.
//
// Example 1:
//
// Input: root = [1,2,3,4,5,6]
// Output: true
// Explanation: Every level before the last is full (ie. levels with node-values {1} and {2, 3}),
// and all nodes in the last level ({4, 5, 6}) are as far left as possible.
//
// Example 2:
//
// Input: root = [1,2,3,4,5,null,7]
// Output: false
// Explanation: The node with value 7 isn't as far left as possible.
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 100].
// - 1 <= Node.val <= 1000
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut flag = false;
        while let Some(node) = queue.pop_front() {
            if let Some(node) = node {
                if flag {
                    return false;
                }
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            } else {
                flag = true;
            }
        }
        true
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    assert_eq!(Solution::is_complete_tree(root), true);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7)]);
    assert_eq!(Solution::is_complete_tree(root), false);
}
