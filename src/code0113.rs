#![allow(dead_code)]

// 113. Path Sum II
// https://leetcode.com/problems/path-sum-ii/

use super::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum<T: PartialEq + Clone + std::ops::Add<Output = T> + Default + Copy>(
        root: Option<Rc<RefCell<TreeNode<T>>>>,
        target_sum: T,
    ) -> Vec<Vec<T>> {
        fn backtrack<T: PartialEq + Clone + std::ops::Add<Output = T> + Default + Copy>(
            node_rc: Rc<RefCell<TreeNode<T>>>,
            target_sum: T,
            path: &mut Vec<T>,
            sum: T,
            mut rez: Vec<Vec<T>>,
        ) -> Vec<Vec<T>> {
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
            Some(root_rc) => backtrack(root_rc, target_sum, &mut vec![], T::default(), vec![]),
            None => vec![],
        }
    }
}

#[test]
fn test_path_sum() {
    let root = TreeNode::from_vec(&vec![
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
