#![allow(dead_code)]

// 337. House Robber III
// https://leetcode.com/problems/house-robber-iii/
// https://leetcode.cn/problems/house-robber-iii/
//
// The thief has found himself a new place for his thievery again. There is only
// one entrance to this area, called the "root." Besides the root, each house has
// one and only one parent house. After a tour, the smart thief realized that
// "all houses in this place forms a binary tree". It will automatically contact
// the police if two directly-linked houses were broken into on the same night.
//
// Determine the maximum amount of money the thief can rob tonight without
// alerting the police.
//
// Example 1:
//
// Input: [3,2,3,null,3,null,1]
//
//      3
//     / \
//    2   3
//     \   \
//      3   1
//
// Output: 7
// Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
//
// Example 2:
//
// Input: [3,4,5,1,3,null,1]
//
//      3
//     / \
//    4   5
//   / \   \
//  1   3   1
//
// Output: 9
// Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (a, b) = Self::rob_sub(root);
        a.max(b)
    }

    fn rob_sub(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let (left_a, left_b) = Self::rob_sub(node.left.clone());
            let (right_a, right_b) = Self::rob_sub(node.right.clone());
            let a = node.val + left_b + right_b;
            let b = left_a.max(left_b) + right_a.max(right_b);
            (a, b)
        } else {
            (0, 0)
        }
    }
}

#[test]
fn test_rob() {
    let root = TreeNode::from_vec(&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1)]);
    assert_eq!(Solution::rob(root), 7);

    let root = TreeNode::from_vec(&[Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)]);
    assert_eq!(Solution::rob(root), 9);
}
