#![allow(dead_code)]

// 114. Flatten Binary Tree to Linked List
// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/

// Definition for a binary tree node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
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
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < v.len() {
            let node = queue.pop_front().unwrap();
            if let Some(val) = v[i] {
                node.as_ref()?.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.as_ref()?.borrow().left.clone());
            }
            i += 1;
            if i >= v.len() {
                break;
            }
            if let Some(val) = v[i] {
                node.as_ref()?.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.as_ref()?.borrow().right.clone());
            }
            i += 1;
        }
        root
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // .flatten() takes care of the edge case of an empty root,
        // in which case the stack will be empty.
        let mut stack = std::iter::once(root.clone()).flatten().collect::<Vec<_>>();
        // Create a dummy node to make it easier to append nodes without
        // dealing with the edge case of the first node
        let mut head = Rc::new(RefCell::new(TreeNode::new(0)));
        while let Some(node) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            // Push right node onto stack first, since we want to pop it last
            if let Some(right) = node_ref.right.take() {
                stack.push(right);
            }
            if let Some(left) = node_ref.left.take() {
                stack.push(left);
            }
            // Drop node_ref to be able to move node in "head = node;"
            drop(node_ref);
            head.borrow_mut().right = Some(node.clone());
            head = node;
        }
    }
}

#[test]
fn test() {
    let mut root = TreeNode::from_vec(vec![
        Some(1),
        Some(2),
        Some(5),
        Some(3),
        Some(4),
        None,
        Some(6),
    ]);
    Solution::flatten(&mut root);
    let mut v = vec![];
    while let Some(node) = root {
        v.push(node.borrow().val);
        root = node.borrow_mut().right.take();
    }
    assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
}
