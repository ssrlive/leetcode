#![allow(dead_code)]

// 101. Symmetric Tree
// https://leetcode.com/problems/symmetric-tree/
//
// Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).
//
// For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
//
//     1
//    / \
//   2   2
//  / \ / \
// 3  4 4  3
//
// But the following [1,2,2,null,3,null,3] is not:
//
//     1
//    / \
//   2   2
//    \   \
//    3    3
//
// Note:
// Bonus points if you could solve it both recursively and iteratively.
//

use crate::treenode::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric<T: PartialEq>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
        fn _is_symmetric<T: PartialEq>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Option<bool> {
            if root.is_none() {
                return Some(true);
            }
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root.clone());
            queue.push_back(root);
            while !queue.is_empty() {
                let left = queue.pop_front()?;
                let right = queue.pop_front()?;
                if left.is_none() && right.is_none() {
                    continue;
                }
                if left.is_none() || right.is_none() {
                    return Some(false);
                }
                if left.as_ref()?.borrow().val != right.as_ref()?.borrow().val {
                    return Some(false);
                }
                queue.push_back(left.as_ref()?.borrow().left.clone());
                queue.push_back(right.as_ref()?.borrow().right.clone());
                queue.push_back(left.as_ref()?.borrow().right.clone());
                queue.push_back(right.as_ref()?.borrow().left.clone());
            }
            Some(true)
        }
        _is_symmetric(root).unwrap_or(false)
    }

    pub fn is_symmetric_2<T: PartialEq>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
        fn compare<T: PartialEq>(
            l: Option<Rc<RefCell<TreeNode<T>>>>,
            r: Option<Rc<RefCell<TreeNode<T>>>>,
        ) -> bool {
            match (l, r) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(l), Some(r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    return compare(l.borrow().left.clone(), r.borrow().right.clone())
                        && compare(l.borrow().right.clone(), r.borrow().left.clone());
                }
            }
        }
        match root {
            Some(r) => compare(r.borrow().left.clone(), r.borrow().right.clone()),
            None => true,
        }
    }
}

#[test]
fn test_is_symmetric() {
    assert_eq!(
        Solution::is_symmetric_2(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3)
        ])),
        true
    );
    assert_eq!(
        Solution::is_symmetric_2(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3)
        ])),
        false
    );
}
