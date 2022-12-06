#![allow(dead_code)]

// 662. Maximum Width of Binary Tree
// https://leetcode.com/problems/maximum-width-of-binary-tree/
//
// Given the root of a binary tree, return the maximum width of the given tree.
//
// The maximum width of a tree is the maximum width among all levels.
//
// The width of one level is defined as the length between the end-nodes (the leftmost and rightmost non-null nodes), where the null nodes between the end-nodes that would be present in a complete binary tree extending down to that level are also counted into the length calculation.
//
// It is guaranteed that the answer will in the range of a 32-bit signed integer.
//
// Example 1:
//
// Input: root = [1,3,2,5,3,null,9]
// Output: 4
// Explanation: The maximum width exists in the third level with length 4 (5,3,null,9).
//
// Example 2:
//
// Input: root = [1,3,2,5,null,null,9,6,null,7]
// Output: 7
// Explanation: The maximum width exists in the fourth level with length 7 (6,null,null,null,null,null,7).
//
// Example 3:
//
// Input: root = [1,3,2,5]
// Output: 2
// Explanation: The maximum width exists in the second level with length 2 (3,2).
//
// Constraints:
//
// -The number of nodes in the tree is in the range [1, 3000].
// - -100 <= Node.val <= 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut queue = vec![(root, 0)];

        while !queue.is_empty() {
            let mut next = vec![];
            let mut min = 0;
            let mut max = 0;

            for (node, index) in queue {
                if let Some(node) = node {
                    let node = node.borrow();
                    let left = node.left.clone();
                    let right = node.right.clone();

                    if next.is_empty() {
                        min = index;
                    }

                    max = index;
                    next.push((left, index * 2));
                    next.push((right, index * 2 + 1));
                }
            }

            result = result.max(max - min + 1);
            queue = next;
        }

        result
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)]);
    assert_eq!(Solution::width_of_binary_tree(root), 4);

    let root = TreeNode::from_vec(&[
        Some(1),
        Some(3),
        Some(2),
        Some(5),
        None,
        None,
        Some(9),
        Some(6),
        None,
        Some(7),
    ]);
    assert_eq!(Solution::width_of_binary_tree(root), 7);

    let root = TreeNode::from_vec(&[Some(1), Some(3), Some(2), Some(5)]);
    assert_eq!(Solution::width_of_binary_tree(root), 2);
}
