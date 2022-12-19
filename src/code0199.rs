#![allow(dead_code)]

// 199. Binary Tree Right Side View
// https://leetcode.com/problems/binary-tree-right-side-view/
// https://leetcode.cn/problems/binary-tree-right-side-view/
//
// Given a binary tree, imagine yourself standing on the right side of it,
// return the values of the nodes you can see ordered from top to bottom.
//

use super::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if let Some(root) = root {
            let mut queue = vec![root];
            while !queue.is_empty() {
                let mut next_queue = vec![];
                queue.iter().for_each(|node| {
                    if let Some(ref left) = node.borrow().left {
                        next_queue.push(left.clone());
                    }
                    if let Some(ref right) = node.borrow().right {
                        next_queue.push(right.clone());
                    }
                });
                result.push(queue.last().unwrap().borrow().val);
                queue = next_queue;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::right_side_view(TreeNode::from_vec(&vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4)
        ])),
        vec![1, 3, 4]
    );
    assert_eq!(
        Solution::right_side_view(TreeNode::from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7)
        ])),
        vec![1, 3, 7]
    );
}
