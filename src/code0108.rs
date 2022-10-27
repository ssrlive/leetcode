#![allow(dead_code)]

// 108. Convert Sorted Array to Binary Search Tree
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
//
// Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
//
// For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
//
// Example:
//
// Given the sorted array: [-10,-3,0,5,9],
//
// One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
//
//       0
//      / \
//    -3   9
//    /   /
//  -10  5

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            let mut node = TreeNode::new(nums[mid]);
            node.left = helper(&nums[..mid]);
            node.right = helper(&nums[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
        helper(&nums)
    }
}

#[test]
fn test_sorted_array_to_bst() {
    let root = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
    let res = root.unwrap().borrow().to_vec();
    assert_eq!(res, vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]);
}
