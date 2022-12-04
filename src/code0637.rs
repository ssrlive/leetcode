#![allow(dead_code)]

// 637. Average of Levels in Binary Tree
// https://leetcode.com/problems/average-of-levels-in-binary-tree/
//
// Given the root of a binary tree, return the average value of the nodes on each level in the form of an array.
// Answers within 10^-5 of the actual answer will be accepted.
//
// Example 1:
//
// Input: root = [3,9,20,null,null,15,7]
// Output: [3.00000,14.50000,11.00000]
// Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
// Hence return [3, 14.5, 11].
//
// Example 2:
//
// Input: root = [3,9,20,15,7]
// Output: [3.00000,14.50000,11.00000]
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 10^4].
// -2^31 <= Node.val <= 2^31 - 1
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = vec![];
        let mut queue = vec![];
        if let Some(root) = root {
            queue.push(root);
        }
        while !queue.is_empty() {
            let mut sum = 0.0;
            let mut count = 0;
            let mut next_queue = vec![];
            for node in queue {
                let node = node.borrow();
                sum += node.val as f64;
                count += 1;
                if let Some(left) = node.left.clone() {
                    next_queue.push(left);
                }
                if let Some(right) = node.right.clone() {
                    next_queue.push(right);
                }
            }
            result.push(sum / count as f64);
            queue = next_queue;
        }
        result
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let result = Solution::average_of_levels(root);
    assert_eq!(result, vec![3.0, 14.5, 11.0]);

    let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), Some(15), Some(7)]);
    let result = Solution::average_of_levels(root);
    assert_eq!(result, vec![3.0, 14.5, 11.0]);

    let root = TreeNode::from_vec(&[Some(2147483647), Some(2147483647), Some(2147483647)]);
    let result = Solution::average_of_levels(root);
    assert_eq!(result, vec![2147483647.00000, 2147483647.00000]);
}
