#![allow(dead_code)]

// 669. Trim a Binary Search Tree
// https://leetcode.com/problems/trim-a-binary-search-tree/
//
// Given the root of a binary search tree and the lowest and highest boundaries as low and high,
// trim the tree so that all its elements lies in [low, high]. Trimming the tree should not change
// the relative structure of the elements that will remain in the tree (i.e., any node's descendant
// should remain a descendant). It can be proven that there is a unique answer.
//
// Return the root of the trimmed binary search tree. Note that the root may change depending on the
// given bounds.
//
// Example 1:
//
// Input: root = [1,0,2], low = 1, high = 2
// Output: [1,null,2]
//
// Example 2:
//
// Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3
// Output: [3,2,null,1]
//
// Example 3:
//
// Input: root = [1], low = 1, high = 2
// Output: [1]
//
// Example 4:
//
// Input: root = [1,null,2], low = 1, high = 3
// Output: [1,null,2]
//
// Example 5:
//
// Input: root = [1,null,2], low = 2, high = 4
// Output: [2]
//
// Constraints:
//
// - The number of nodes in the tree in the range [1, 10^4].
// - 0 <= Node.val <= 10^4
// - The value of each node in the tree is unique.
// - root is guaranteed to be a valid binary search tree.
// - 0 <= low <= high <= 10^4
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref()?;
        let val = root.as_ref()?.borrow().val;
        if val < low {
            Solution::trim_bst(root.as_ref()?.borrow().right.clone(), low, high)
        } else if val > high {
            Solution::trim_bst(root.as_ref()?.borrow().left.clone(), low, high)
        } else {
            let left = Solution::trim_bst(root.as_ref()?.borrow().left.clone(), low, high);
            root.as_ref()?.borrow_mut().left = left;
            let right = Solution::trim_bst(root.as_ref()?.borrow().right.clone(), low, high);
            root.as_ref()?.borrow_mut().right = right;
            root
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(0), Some(2)]);
    let res = TreeNode::from_vec(&vec![Some(1), None, Some(2)]);
    assert_eq!(Solution::trim_bst(root, 1, 2), res);

    let root = TreeNode::from_vec(&[Some(3), Some(0), Some(4), None, Some(2), None, None, Some(1)]);
    let res = TreeNode::from_vec(&[Some(3), Some(2), None, Some(1)]);
    assert_eq!(Solution::trim_bst(root, 1, 3), res);

    let root = TreeNode::from_vec(&[Some(1)]);
    let res = TreeNode::from_vec(&[Some(1)]);
    assert_eq!(Solution::trim_bst(root, 1, 2), res);

    let root = TreeNode::from_vec(&[Some(1), None, Some(2)]);
    let res = TreeNode::from_vec(&[Some(1), None, Some(2)]);
    assert_eq!(Solution::trim_bst(root, 1, 3), res);

    let root = TreeNode::from_vec(&[Some(1), None, Some(2)]);
    let res = TreeNode::from_vec(&[Some(2)]);
    assert_eq!(Solution::trim_bst(root, 2, 4), res);
}
