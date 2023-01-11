#![allow(dead_code)]

// 1080. Insufficient Nodes in Root to Leaf Paths
// https://leetcode.com/problems/insufficient-nodes-in-root-to-leaf-paths/
// https://leetcode.cn/problems/insufficient-nodes-in-root-to-leaf-paths/
//
// Given the root of a binary tree and an integer limit, delete all insufficient nodes in
// the tree simultaneously, and return the root of the resulting binary tree.
//
// A node is insufficient if every root to leaf path intersecting this node has a sum strictly less than limit.
//
// A leaf is a node with no children.
//
// Example 1:
//
// Input: root = [1,2,3,4,-99,-99,7,8,9,-99,-99,12,13,-99,14], limit = 1
// Output: [1,2,3,4,null,null,7,8,9,null,14]
//
// Example 2:
//
// Input: root = [5,4,8,11,null,17,4,7,1,null,null,5,3], limit = 22
// Output: [5,4,8,11,null,17,4,7,null,null,null,5]
//
// Example 3:
//
// Input: root = [1,2,-3,-5,null,4,null], limit = -1
// Output: [1,null,-3,4]
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 5000].
// - -10^5 <= Node.val <= 10^5
// - -10^9 <= limit <= 10^9
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref()?;
        let val = root.as_ref()?.borrow().val;
        if root.as_ref()?.borrow().left.is_none() && root.as_ref()?.borrow().right.is_none() {
            if val < limit {
                return None;
            } else {
                return root;
            }
        }
        if root.as_ref()?.borrow().left.is_some() {
            let left = Solution::sufficient_subset(root.as_ref()?.borrow_mut().left.take(), limit - val);
            root.as_ref()?.borrow_mut().left = left;
        }
        if root.as_ref()?.borrow().right.is_some() {
            let right = Solution::sufficient_subset(root.as_ref()?.borrow_mut().right.take(), limit - val);
            root.as_ref()?.borrow_mut().right = right;
        }

        if root.as_ref()?.borrow().left.is_none() && root.as_ref()?.borrow().right.is_none() {
            None
        } else {
            root
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(-99),
        Some(-99),
        Some(7),
        Some(8),
        Some(9),
        Some(-99),
        Some(-99),
        Some(12),
        Some(13),
        Some(-99),
        Some(14),
    ]);
    let limit = 1;
    let output = TreeNode::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        Some(7),
        Some(8),
        Some(9),
        None,
        Some(14),
    ]);
    assert_eq!(Solution::sufficient_subset(root, limit), output);

    let root = TreeNode::from_vec(&[
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(17),
        Some(4),
        Some(7),
        Some(1),
        None,
        None,
        Some(5),
        Some(3),
    ]);
    let limit = 22;
    let output = TreeNode::from_vec(&[
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(17),
        Some(4),
        Some(7),
        None,
        None,
        None,
        Some(5),
    ]);
    assert_eq!(Solution::sufficient_subset(root, limit), output);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(-3), Some(-5), None, Some(4), None]);
    let limit = -1;
    let output = TreeNode::from_vec(&[Some(1), None, Some(-3), Some(4)]);
    assert_eq!(Solution::sufficient_subset(root, limit), output);
}
