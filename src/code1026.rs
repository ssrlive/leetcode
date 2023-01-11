#![allow(dead_code)]

// 1026. Maximum Difference Between Node and Ancestor
// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
// https://leetcode.cn/problems/maximum-difference-between-node-and-ancestor/
//
// Given the root of a binary tree, find the maximum value v for which there exist
// different nodes a and b where v = |a.val - b.val| and a is an ancestor of b.
//
// A node a is an ancestor of b if either: any child of a is equal to b or any child of a is an ancestor of b.
//
// Example 1:
//
// Input: root = [8,3,10,1,6,null,14,null,null,4,7,13]
// Output: 7
// Explanation: We have various ancestor-node differences, some of which are given below :
// |8 - 3| = 5
// |3 - 7| = 4
// |8 - 1| = 7
// |10 - 13| = 3
// Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
//
// Example 2:
//
// Input: root = [1,null,2,null,0,3]
// Output: 3
//
// Constraints:
//
// - The number of nodes in the tree is in the range [2, 5000].
// - 0 <= Node.val <= 10^5
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_diff(root: Option<Rc<RefCell<TreeNode>>>, ancestors: &mut Vec<i32>, max: &mut i32) {
            if let Some(node) = root {
                let mut node = node.as_ref().borrow_mut();
                for ancestor in ancestors.iter() {
                    *max = std::cmp::max(*max, (*ancestor - node.val).abs());
                }
                ancestors.push(node.val);
                if node.left.is_some() {
                    max_diff(node.left.take(), ancestors, max);
                }
                if node.right.is_some() {
                    max_diff(node.right.take(), ancestors, max);
                }
                ancestors.pop();
            }
        }

        let mut ancestors = Vec::new();
        let mut res = 0;
        max_diff(root, &mut ancestors, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(8),
        Some(3),
        Some(10),
        Some(1),
        Some(6),
        None,
        Some(14),
        None,
        None,
        Some(4),
        Some(7),
        Some(13),
    ]);
    assert_eq!(Solution::max_ancestor_diff(root), 7);

    let root = TreeNode::from_vec(&[Some(1), None, Some(2), None, Some(0), Some(3)]);
    assert_eq!(Solution::max_ancestor_diff(root), 3);
}
