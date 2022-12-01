#![allow(dead_code)]

// 515. Find Largest Value in Each Tree Row
// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
//
// Given the root of a binary tree, return an array of the largest value in each row of the tree (0-indexed).
//
// Example 1:
//
// Input: root = [1,3,2,5,3,null,9]
// Output: [1,3,9]
//
// Example 2:
//
// Input: root = [1,2,3]
// Output: [1,3]
//
// Constraints:
//
// - The number of nodes in the tree will be in the range [0, 104].
// - -2^31 <= Node.val <= 2^31 - 1
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        let mut queue = vec![root];
        while !queue.is_empty() {
            let mut max = None;
            let mut next_queue = vec![];
            for node in queue.into_iter().flatten() {
                let node = node.borrow();
                if let Some(_max) = max {
                    if node.val > _max {
                        max = Some(node.val);
                    }
                } else {
                    max = Some(node.val);
                }
                next_queue.push(node.left.clone());
                next_queue.push(node.right.clone());
            }
            if let Some(max) = max {
                result.push(max);
            }
            queue = next_queue;
        }

        result
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)]);
    let result = Solution::largest_values(root);
    assert_eq!(result, vec![1, 3, 9]);
}
