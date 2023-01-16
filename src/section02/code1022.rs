#![allow(dead_code)]

// 1022. Sum of Root To Leaf Binary Numbers
// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
// https://leetcode.cn/problems/sum-of-root-to-leaf-binary-numbers/
//
// You are given the root of a binary tree where each node has a value 0 or 1. Each root-to-leaf path represents a binary number starting with the most significant bit.
//
// For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.
// For all leaves in the tree, consider the numbers represented by the path from the root to that leaf. Return the sum of these numbers.
//
// The test cases are generated so that the answer fits in a 32-bits integer.
//
// Example 1:
//
// Input: root = [1,0,1,0,1,0,1]
// Output: 22
// Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
//
// Example 2:
//
// Input: root = [0]
// Output: 0
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 1000].
// - Node.val is 0 or 1.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recur(node: &Option<Rc<RefCell<TreeNode>>>, mut s: i32) -> i32 {
            if let Some(cur_node) = node {
                let ref_node = cur_node.borrow();
                s = (s << 1) + ref_node.val;

                match ref_node.left.is_none() && ref_node.right.is_none() {
                    true => s,
                    false => recur(&ref_node.left, s) + recur(&ref_node.right, s),
                }
            } else {
                0
            }
        }
        recur(&root, 0)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(0), Some(1), Some(0), Some(1), Some(0), Some(1)]);
    assert_eq!(Solution::sum_root_to_leaf(root), 22);

    let root = TreeNode::from_vec(&[Some(0)]);
    assert_eq!(Solution::sum_root_to_leaf(root), 0);
}
