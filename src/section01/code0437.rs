#![allow(dead_code)]

// 437. Path Sum III
// https://leetcode.com/problems/path-sum-iii/
// https://leetcode.cn/problems/path-sum-iii/
//
// Given the root of a binary tree and an integer targetSum, return the number of paths where the sum of the values along the path equals targetSum.
//
// The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).
//
// Example 1:
//
// Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
// Output: 3
// Explanation: The paths that sum to 8 are shown.
//
// Example 2:
//
// Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
// Output: 3
//
// Constraints:
//
// - The number of nodes in the tree is in the range [0, 1000].
// - -10^9 <= Node.val <= 10^9
// - -1000 <= targetSum <= 1000
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        pub fn _path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> Option<i64> {
            if root.is_none() {
                return Some(0);
            }
            let mut answer = 0_i64;
            let mut stack = Vec::new();
            stack.push((root.clone(), 0));
            while !stack.is_empty() {
                let (node, sum) = stack.pop()?;
                let sum = sum + node.as_ref()?.borrow().val as i64;
                if sum == target_sum {
                    answer += 1;
                }
                let left = node.as_ref()?.borrow().left.clone();
                if left.is_some() {
                    stack.push((left, sum));
                }
                let right = node.as_ref()?.borrow().right.clone();
                if right.is_some() {
                    stack.push((right, sum));
                }
            }
            let v = answer
                + _path_sum(root.as_ref()?.borrow().left.clone(), target_sum).unwrap_or_default()
                + _path_sum(root.as_ref()?.borrow().right.clone(), target_sum).unwrap_or_default();
            Some(v)
        }
        _path_sum(root, target_sum as i64).unwrap_or_default() as i32
    }
}

#[test]
fn test_path_sum() {
    assert_eq!(
        Solution::path_sum(
            TreeNode::from_vec(&[
                Some(10),
                Some(5),
                Some(-3),
                Some(3),
                Some(2),
                None,
                Some(11),
                Some(3),
                Some(-2),
                None,
                Some(1)
            ]),
            8
        ),
        3
    );
    assert_eq!(
        Solution::path_sum(
            TreeNode::from_vec(&[
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                Some(5),
                Some(1)
            ]),
            22
        ),
        3
    );
}
