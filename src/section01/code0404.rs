#![allow(dead_code)]

// 404. Sum of Left Leaves
// https://leetcode.com/problems/sum-of-left-leaves/
// https://leetcode.cn/problems/sum-of-left-leaves/
//
// Given the root of a binary tree, return the sum of all left leaves.
//
// A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.
//
// Example 1:
//
// Input: root = [3,9,20,null,null,15,7]
// Output: 24
// Explanation: There are two left leaves in the binary tree, with values 9 and 15 respectively.
//
// Example 2:
//
// Input: root = [1]
// Output: 0
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 1000].
// - -1000 <= Node.val <= 1000
//

use crate::treenode::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut sum = 0;
            if let Some(left) = root.borrow().left.clone() {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    sum += left.borrow().val;
                } else {
                    sum += Self::sum_of_left_leaves(Some(left));
                }
            }
            sum += Self::sum_of_left_leaves(root.borrow().right.clone());
            sum
        } else {
            0
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sum_of_left_leaves(TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])),
        24
    );
    assert_eq!(Solution::sum_of_left_leaves(TreeNode::from_vec(&[Some(1)])), 0);
}
