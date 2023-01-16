#![allow(dead_code)]

// 230. Kth Smallest Element in a BST
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// https://leetcode.cn/problems/kth-smallest-element-in-a-bst/
//
// Given a binary search tree, write a function kthSmallest to find the kth smallest element in it.
//
// Note:
// You may assume k is always valid, 1 ≤ k ≤ BST's total elements.
//
// Example 1:
//
// Input: root = [3,1,4,null,2], k = 1
//    3
//   / \
//  1   4
//   \
//    2
// Output: 1
//
// Example 2:
//
// Input: root = [5,3,6,2,4,null,null,1], k = 3
//        5
//       / \
//      3   6
//     / \
//    2   4
//   /
//  1
// Output: 3
//
// Follow up:
// What if the BST is modified (insert/delete operations) often and you need to find the kth smallest frequently?
// How would you optimize the kthSmallest routine?
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn kth_smallest_helper(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, count: &mut i32, result: &mut i32) {
            if let Some(root) = root {
                let root = root.borrow();
                kth_smallest_helper(&root.left, k, count, result);
                *count += 1;
                if *count == k {
                    *result = root.val;
                    return;
                }
                kth_smallest_helper(&root.right, k, count, result);
            }
        }

        let mut result = 0;
        let mut count = 0;
        kth_smallest_helper(&root, k, &mut count, &mut result);
        result
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(3), Some(1), Some(4), None, Some(2)]);
    assert_eq!(Solution::kth_smallest(root, 1), 1);

    let root = TreeNode::from_vec(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)]);
    assert_eq!(Solution::kth_smallest(root, 3), 3);
}
