#![allow(dead_code)]

// 653. Two Sum IV - Input is a BST
// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/
//
// Given the root of a Binary Search Tree and a target number k, return true if there exist two elements
// in the BST such that their sum is equal to the given target.
//
// Example 1:
//
// Input: root = [5,3,6,2,4,null,7], k = 9
// Output: true
//
// Example 2:
//
// Input: root = [5,3,6,2,4,null,7], k = 28
// Output: false
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 10^4].
// - -10^4 <= Node.val <= 10^4
// - root is guaranteed to be a valid binary search tree.
// - -10^5 <= k <= 10^5
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = std::collections::HashSet::new();
        Self::find_target_helper(root, k, &mut set)
    }

    fn find_target_helper(root: Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut std::collections::HashSet<i32>) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val;
            if set.contains(&(k - val)) {
                return true;
            }
            set.insert(val);
            return Self::find_target_helper(node.borrow().left.clone(), k, set)
                || Self::find_target_helper(node.borrow().right.clone(), k, set);
        }
        false
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    assert_eq!(Solution::find_target(root, 9), true);
    let root = TreeNode::from_vec(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    assert_eq!(Solution::find_target(root, 28), false);
}
