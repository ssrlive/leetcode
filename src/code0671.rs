#![allow(dead_code)]

// 671. Second Minimum Node In a Binary Tree
// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
//
// Given a non-empty special binary tree consisting of nodes with the non-negative value, where each
// node in this tree has exactly two or zero sub-node. If the node has two sub-nodes, then this node's
// value is the smaller value among its two sub-nodes.
//
// Given such a binary tree, you need to output the second minimum value in the set made of all the
// nodes' value in the whole tree.
//
// If no such second minimum value exists, output -1 instead.
//
// Example 1:
//
// Input:
//     2
//    / \
//   2   5
//      / \
//     5   7
//
// Output: 5
// Explanation: The smallest value is 2, the second smallest value is 5.
//
// Example 2:
//
// Input:
//     2
//    / \
//   2   2
//
// Output: -1
// Explanation: The smallest value is 2, but there isn't any second smallest value.
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 25].
// - 1 <= Node.val <= 2^31 - 1
// - root.val == min(root.left.val, root.right.val) for each internal node of the tree.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursive_helper(node: &Option<Rc<RefCell<TreeNode>>>, min: &mut Option<i32>, second_min: &mut Option<i32>) {
            if let Some(node) = node {
                let val = node.borrow().val;
                if min.is_none() || val < min.unwrap() {
                    *second_min = *min;
                    *min = Some(val);
                } else if val > min.unwrap() && (second_min.is_none() || val < second_min.unwrap()) {
                    *second_min = Some(val);
                }
                recursive_helper(&node.borrow().left, min, second_min);
                recursive_helper(&node.borrow().right, min, second_min);
            }
        }
        let mut min = None;
        let mut second_min = None;
        recursive_helper(&root, &mut min, &mut second_min);
        second_min.unwrap_or(-1)

        /*
        // Iterative solution, but slower than recursive solution.
        let mut min = None;
        let mut second_min = None;
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let val = node.borrow().val;
                if min.is_none() || val < min.unwrap() {
                    second_min = min;
                    min = Some(val);
                } else if val > min.unwrap() && (second_min.is_none() || val < second_min.unwrap()) {
                    second_min = Some(val);
                }
                stack.push(node.borrow().left.clone());
                stack.push(node.borrow().right.clone());
            }
        }
        second_min.unwrap_or(-1)
        */
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(2), Some(2), Some(5), None, None, Some(5), Some(7)]);
    assert_eq!(Solution::find_second_minimum_value(root), 5);
    let root = TreeNode::from_vec(&[Some(2), Some(2), Some(2)]);
    assert_eq!(Solution::find_second_minimum_value(root), -1);
}
