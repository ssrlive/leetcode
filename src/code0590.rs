#![allow(dead_code)]

// 590. N-ary Tree Postorder Traversal
// https://leetcode.com/problems/n-ary-tree-postorder-traversal/
//
// Given the root of an n-ary tree, return the postorder traversal of its nodes' values.
//
// Nary-Tree input serialization is represented in their level order traversal.
// Each group of children is separated by the null value (See examples)
//
// Example 1:
//
// Input: root = [1,null,3,2,4,null,5,6]
// Output: [5,6,3,2,4,1]
//
// Example 2:
//
// Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
// Output: [2,6,14,11,7,3,12,8,4,13,9,10,5,1]
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 10^4].
// 0 <= Node.val <= 10^4
// The height of the n-ary tree is less than or equal to 1000.
//
// Follow up: Recursive solution is trivial, could you do it iteratively?
//

use crate::narytree::Node;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        Self::_postorder(root).unwrap_or_default()
    }

    pub fn _postorder(root: Option<Rc<RefCell<Node>>>) -> Option<Vec<i32>> {
        let mut ret = vec![];
        let mut stack = std::collections::VecDeque::new();
        if root.is_some() {
            stack.push_back(root);
        }
        while let Some(node) = stack.pop_back() {
            ret.push(node.as_ref()?.borrow().val);
            for child in node.as_ref()?.borrow().children.iter() {
                stack.push_back(child.clone());
            }
        }
        ret.reverse();
        Some(ret)
    }
}

#[test]
fn test() {
    let root = Node::from_vec(vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)]);
    assert_eq!(Solution::postorder(root), vec![5, 6, 3, 2, 4, 1]);

    let root = Node::from_vec(vec![
        Some(1),
        None,
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        Some(6),
        Some(7),
        None,
        Some(8),
        None,
        Some(9),
        Some(10),
        None,
        None,
        Some(11),
        None,
        Some(12),
        None,
        Some(13),
        None,
        None,
        Some(14),
    ]);
    assert_eq!(
        Solution::postorder(root),
        vec![2, 6, 14, 11, 7, 3, 12, 8, 4, 13, 9, 10, 5, 1]
    );
}
