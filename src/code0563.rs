#![allow(dead_code)]

// 563. Binary Tree Tilt
// https://leetcode.com/problems/binary-tree-tilt/
//
// Given the root of a binary tree, return the sum of every tree node's tilt.
//
// The tilt of a tree node is the absolute difference between the sum of all left subtree node values and all right subtree node values.
// If a node does not have a left child, then the sum of the left subtree node values is treated as 0.
// The rule is similar if the node does not have a right child.
//
// Example 1:
//
// Input: root = [1,2,3]
// Output: 1
// Explanation:
// Tilt of node 2 : |0-0| = 0 (no children)
// Tilt of node 3 : |0-0| = 0 (no children)
// Tilt of node 1 : |2-3| = 1 (left subtree is just left child, so sum is 2; right subtree is just right child, so sum is 3)
// Sum of every tilt : 0 + 0 + 1 = 1
//
// Example 2:
//
// Input: root = [4,2,9,3,5,null,7]
// Output: 15
// Explanation:
// Tilt of node 3 : |0-0| = 0 (no children)
// Tilt of node 5 : |0-0| = 0 (no children)
// Tilt of node 7 : |0-0| = 0 (no children)
// Tilt of node 2 : |3-5| = 2 (left subtree is just left child, so sum is 3; right subtree is just right child, so sum is 5)
// Tilt of node 9 : |0-7| = 7 (no left child, so sum is 0; right subtree is just right child, so sum is 7)
// Tilt of node 4 : |(3+5+2)-(9+7)| = |10-16| = 6 (left subtree values are 3, 5, and 2, which sums to 10;
//                  right subtree values are 9 and 7, which sums to 16)
// Sum of every tilt : 0 + 0 + 0 + 2 + 7 + 6 = 15
//
// Example 3:
//
// Input: root = [21,7,14,1,1,2,2,3,3]
// Output: 9
//
// Constraints:
//
// - The number of nodes in the tree is in the range [0, 10^4].
// - -1000 <= Node.val <= 1000
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::find_tilt_helper(root, &mut sum);
        sum
    }

    fn find_tilt_helper(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
        if let Some(node) = root {
            let left = Self::find_tilt_helper(node.borrow().left.clone(), sum);
            let right = Self::find_tilt_helper(node.borrow().right.clone(), sum);
            *sum += (left - right).abs();
            left + right + node.borrow().val
        } else {
            0
        }
    }
}

#[test]
fn test_find_tilt() {
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::find_tilt(root), 1);

    let root = TreeNode::from_vec(&[Some(4), Some(2), Some(9), Some(3), Some(5), None, Some(7)]);
    assert_eq!(Solution::find_tilt(root), 15);

    let root = TreeNode::from_vec(&[
        Some(21),
        Some(7),
        Some(14),
        Some(1),
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
    ]);
    assert_eq!(Solution::find_tilt(root), 9);
}
