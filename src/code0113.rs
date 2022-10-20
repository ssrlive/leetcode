#![allow(dead_code)]

// 113. Path Sum II
// https://leetcode.com/problems/path-sum-ii/

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
                node.as_ref()?.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*val))));
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            node_rc: Rc<RefCell<TreeNode>>,
            target_sum: i32,
            path: &mut Vec<i32>,
            sum: i32,
            mut rez: Vec<Vec<i32>>,
        ) -> Vec<Vec<i32>> {
            let mut node_ref = node_rc.borrow_mut();
            path.push(node_ref.val);
            let sum = sum + node_ref.val;
            match (node_ref.left.take(), node_ref.right.take()) {
                (None, None) => {
                    if sum == target_sum {
                        rez.push(path.clone())
                    }
                }
                (None, Some(right_rc)) => rez = backtrack(right_rc, target_sum, path, sum, rez),
                (Some(left_rc), None) => rez = backtrack(left_rc, target_sum, path, sum, rez),
                (Some(left_rc), Some(right_rc)) => {
                    rez = backtrack(right_rc, target_sum, path, sum, rez);
                    rez = backtrack(left_rc, target_sum, path, sum, rez)
                }
            }
            path.pop();
            rez
        }

        match root {
            Some(root_rc) => backtrack(root_rc, target_sum, &mut vec![], 0, vec![]),
            None => vec![],
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(vec![
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
        Some(1),
    ]);
    let target_sum = 22;
    let rez = Solution::path_sum(root, target_sum);
    let expected = vec![vec![5, 8, 4, 5], vec![5, 4, 11, 2]];
    assert_eq!(rez, expected);
}
