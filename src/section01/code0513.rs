#![allow(dead_code)]

// 513. Find Bottom Left Tree Value
// https://leetcode.com/problems/find-bottom-left-tree-value/
// https://leetcode.cn/problems/find-bottom-left-tree-value/
//
// Given the root of a binary tree, return the leftmost value in the last row of the tree.
//
// Example 1:
//
// Input: root = [2,1,3]
// Output: 1
//
// Example 2:
//
// Input: root = [1,2,3,4,null,5,6,null,null,7]
// Output: 7
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 10^4].
// - -2^31 <= Node.val <= 2^31 - 1
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn _find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            let mut queue = vec![];
            let mut result = 0;

            if let node @ Some(_) = root {
                queue.push(node);
            }

            while !queue.is_empty() {
                let mut next_queue = vec![];

                for node in queue.iter() {
                    if let left @ Some(_) = node.as_ref()?.borrow().left.clone() {
                        next_queue.push(left);
                    }

                    if let right @ Some(_) = node.as_ref()?.borrow().right.clone() {
                        next_queue.push(right);
                    }
                }

                if !next_queue.is_empty() {
                    queue = next_queue;
                } else {
                    result = queue.get(0)?.as_ref()?.borrow().val;
                    break;
                }
            }

            Some(result)
        }

        _find_bottom_left_value(root).unwrap_or_default()
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(2), Some(1), Some(3)]);
    assert_eq!(Solution::find_bottom_left_value(root), 1);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, None, Some(7)]);
    assert_eq!(Solution::find_bottom_left_value(root), 7);
}
