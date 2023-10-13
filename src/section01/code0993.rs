#![allow(dead_code)]

// 993. Cousins in Binary Tree
// https://leetcode.com/problems/cousins-in-binary-tree/
// https://leetcode.cn/problems/cousins-in-binary-tree/
//
// Given the root of a binary tree with unique values and the values of two different nodes of the tree x and y,
// return true if the nodes corresponding to the values x and y in the tree are cousins, or false otherwise.
//
// Two nodes of a binary tree are cousins if they have the same depth with different parents.
//
// Note that in a binary tree, the root node is at the depth 0, and children of each depth k node are at the depth k + 1.
//
// Example 1:
//
// Input: root = [1,2,3,4], x = 4, y = 3
// Output: false
//
// Example 2:
//
//
// Input: root = [1,2,3,null,4,null,5], x = 5, y = 4
// Output: true
//
// Example 3:
//
// Input: root = [1,2,3,null,4], x = 2, y = 3
// Output: false
//
// Constraints:
//
// - The number of nodes in the tree is in the range [2, 100].
// - 1 <= Node.val <= 100
// - Each node has a unique value.
// - x != y
// - x and y are exist in the tree.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        use std::collections::VecDeque;
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back((r, None));
        }
        while !vd.is_empty() {
            let mut x_found = None;
            let mut y_found = None;
            for _ in 0..vd.len() {
                if let Some((node, parent)) = vd.pop_front() {
                    let val = node.borrow().val;
                    if val == x {
                        x_found = parent;
                    }
                    if val == y {
                        y_found = parent;
                    }
                    if let Some(n) = node.borrow_mut().left.take() {
                        vd.push_back((n, Some(val)));
                    }
                    if let Some(n) = node.borrow_mut().right.take() {
                        vd.push_back((n, Some(val)));
                    }
                }
            }
            if let (Some(x_parent), Some(y_parent)) = (x_found, y_found) {
                return x_parent != y_parent;
            }
        }
        false
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4)]);
    assert!(!Solution::is_cousins(root, 4, 3));

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)]);
    assert!(Solution::is_cousins(root, 5, 4));

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), None, Some(4)]);
    assert!(!Solution::is_cousins(root, 2, 3));
}
