#![allow(dead_code)]

// 951. Flip Equivalent Binary Trees
// https://leetcode.com/problems/flip-equivalent-binary-trees/
// https://leetcode.cn/problems/flip-equivalent-binary-trees/
//
// For a binary tree T, we can define a flip operation as follows: choose any node, and swap the left and right child subtrees.
//
// A binary tree X is flip equivalent to a binary tree Y if and only if we can make X equal to Y after some number of flip operations.
//
// Given the roots of two binary trees root1 and root2, return true if the two trees are flip equivalent or false otherwise.
//
// Example 1:
//
// Flipped Trees Diagram
// Input: root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 = [1,3,2,null,6,4,5,null,null,null,null,8,7]
// Output: true
// Explanation: We flipped at nodes with values 1, 3, and 5.
//
// Example 2:
//
// Input: root1 = [], root2 = []
// Output: true
//
// Example 3:
//
// Input: root1 = [], root2 = [1]
// Output: false
//
// Constraints:
//
// The number of nodes in each tree is in the range [0, 100].
// Each tree will have unique node values in the range [0, 99].
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn _flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<bool> {
            if root1.is_none() && root2.is_none() {
                return Some(true);
            }
            if root1.is_none() || root2.is_none() {
                return Some(false);
            }
            if root1.as_ref()?.borrow().val != root2.as_ref()?.borrow().val {
                return Some(false);
            }
            let left1 = root1.as_ref()?.borrow().left.clone();
            let right1 = root1.as_ref()?.borrow().right.clone();
            let left2 = root2.as_ref()?.borrow().left.clone();
            let right2 = root2.as_ref()?.borrow().right.clone();
            let v = (_flip_equiv(left1.clone(), left2.clone())? && _flip_equiv(right1.clone(), right2.clone())?)
                || (_flip_equiv(left1, right2)? && _flip_equiv(right1, left2)?);
            Some(v)
        }
        _flip_equiv(root1, root2).unwrap_or(false)
    }
}

#[test]
fn test() {
    let root1 = TreeNode::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        Some(7),
        Some(8),
    ]);
    let root2 = TreeNode::from_vec(&[
        Some(1),
        Some(3),
        Some(2),
        None,
        Some(6),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        Some(8),
        Some(7),
    ]);
    assert_eq!(Solution::flip_equiv(root1, root2), true);

    let root1 = TreeNode::from_vec(&[]);
    let root2 = TreeNode::from_vec(&[]);
    assert_eq!(Solution::flip_equiv(root1, root2), true);

    let root1 = TreeNode::from_vec(&[]);
    let root2 = TreeNode::from_vec(&[Some(1)]);
    assert_eq!(Solution::flip_equiv(root1, root2), false);
}
