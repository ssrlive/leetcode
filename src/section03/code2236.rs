#![allow(dead_code)]

/*

// 2236. Root Equals Sum of Children
// https://leetcode.com/problems/root-equals-sum-of-children/
// https://leetcode.cn/problems/root-equals-sum-of-children/
//
// Easy
//
// You are given the root of a binary tree that consists of exactly 3 nodes: the root, its left child, and its right child.

Return true if the value of the root is equal to the sum of the values of its two children, or false otherwise.

Example 1:

Input: root = [10,4,6]
Output: true
Explanation: The values of the root, its left child, and its right child are 10, 4, and 6, respectively.
10 is equal to 4 + 6, so we return true.

Example 2:

Input: root = [5,3,1]
Output: false
Explanation: The values of the root, its left child, and its right child are 5, 3, and 1, respectively.
5 is not equal to 3 + 1, so we return false.

Constraints:

    The tree consists only of the root, its left child, and its right child.
    -100 <= Node.val <= 100
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root_node = root.as_ref().unwrap().borrow();
        let left_node = root_node.left.as_ref().unwrap().borrow();
        let right_node = root_node.right.as_ref().unwrap().borrow();
        root_node.val == left_node.val + right_node.val
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(10), Some(4), Some(6)]);
    assert!(Solution::check_tree(root));
    let root = TreeNode::from_vec(&[Some(5), Some(3), Some(1)]);
    assert!(!Solution::check_tree(root));
}
