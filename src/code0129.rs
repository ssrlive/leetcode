#![allow(dead_code)]

// 129. Sum Root to Leaf Numbers
// https://leetcode.com/problems/sum-root-to-leaf-numbers/
//
// You are given the root of a binary tree containing digits from 0 to 9 only.
//
// Each root-to-leaf path in the tree represents a number.
// - For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
//
// Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.
//
// A leaf node is a node with no children.

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if v.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v[0]?))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < v.len() {
            let node = queue.pop_front()?;
            if let Some(val) = v.get(i)? {
                node.as_ref()?.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*val))));
                queue.push_back(node.as_ref()?.borrow().left.clone());
            }
            i += 1;
            if i >= v.len() {
                break;
            }
            if let Some(val) = v.get(i)? {
                node.as_ref()?.borrow_mut().right =
                    Some(Rc::new(RefCell::new(TreeNode::new(*val))));
                queue.push_back(node.as_ref()?.borrow().right.clone());
            }
            i += 1;
        }
        root
    }
}

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn solve(root: Option<&Rc<RefCell<TreeNode>>>, ans: &mut i32, number: &mut i32) {
            if let Some(root) = root {
                let node = root.borrow();
                *number = *number * 10 + node.val;

                if node.left.is_none() && node.right.is_none() {
                    *ans += *number;
                }

                solve(node.left.as_ref(), ans, number);

                solve(node.right.as_ref(), ans, number);

                *number /= 10;
            }
        }

        let mut ans: i32 = 0;
        solve(root.as_ref(), &mut ans, &mut 0);
        ans
    }
}

#[test]
fn test_sum_numbers() {
    let root = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::sum_numbers(root), 25);
    let root = TreeNode::from_vec(vec![Some(4), Some(9), Some(0), Some(5), Some(1)]);
    assert_eq!(Solution::sum_numbers(root), 1026);
}
