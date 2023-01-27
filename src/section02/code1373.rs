#![allow(dead_code)]

// 1373. Maximum Sum BST in Binary Tree
// https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/
// https://leetcode.cn/problems/maximum-sum-bst-in-binary-tree/
//
// Hard
//
// Given a binary tree root, return the maximum sum of all keys of any sub-tree which is also a Binary Search Tree (BST).
//
// Assume a BST is defined as follows:
//
//     The left subtree of a node contains only nodes with keys less than the node's key.
//     The right subtree of a node contains only nodes with keys greater than the node's key.
//     Both the left and right subtrees must also be binary search trees.
//
// Example 1:
//
// Input: root = [1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]
// Output: 20
// Explanation: Maximum sum in a valid Binary search tree is obtained in root node with key equal to 3.
//
// Example 2:
//
// Input: root = [4,3,null,1,2]
// Output: 2
// Explanation: Maximum sum in a valid Binary search tree is obtained in a single root node with key equal to 2.
//
// Example 3:
//
// Input: root = [-4,-2,-5]
// Output: 0
// Explanation: All values are negatives. Return an empty BST.
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [1, 4 * 10^4].
// -    -4 * 10^4 <= Node.val <= 4 * 10^4
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> (i32, i32, i32) {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();
                let (left_min, left_max, left_sum) = dfs(&node.left, max_sum);
                let (right_min, right_max, right_sum) = dfs(&node.right, max_sum);
                if (node.left.is_none() || left_max < node.val) && (node.right.is_none() || right_min > node.val) {
                    let sum = left_sum + right_sum + node.val;
                    *max_sum = (*max_sum).max(sum);
                    (left_min.min(node.val), right_max.max(node.val), sum)
                } else {
                    (i32::MIN, i32::MAX, 0)
                }
            } else {
                (i32::MAX, i32::MIN, 0)
            }
        }
        let mut max_sum = 0;
        dfs(&root, &mut max_sum);
        max_sum
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]");
    assert_eq!(Solution::max_sum_bst(root), 20);
    let root = TreeNode::from_string("[4,3,null,1,2]");
    assert_eq!(Solution::max_sum_bst(root), 2);
    let root = TreeNode::from_string("[-4,-2,-5]");
    assert_eq!(Solution::max_sum_bst(root), 0);
}
