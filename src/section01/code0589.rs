#![allow(dead_code)]

// 589. N-ary Tree Preorder Traversal
// https://leetcode.com/problems/n-ary-tree-preorder-traversal/
// https://leetcode.cn/problems/n-ary-tree-preorder-traversal/
//
// Given the root of an n-ary tree, return the preorder traversal of its nodes' values.
//
// Nary-Tree input serialization is represented in their level order traversal.
// Each group of children is separated by the null value (See examples)
//
// Example 1:
//
// Input: root = [1,null,3,2,4,null,5,6]
// Output: [1,3,5,6,2,4]
//
// Example 2:
//
// Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
// Output: [1,2,3,6,7,11,14,4,8,12,5,9,13,10]
//
// Constraints:
//
// - The number of nodes in the tree is in the range [0, 10^4].
// - 0 <= Node.val <= 10^4
// - The height of the n-ary tree is less than or equal to 1000.
//
// Follow up: Recursive solution is trivial, could you do it iteratively?
//

use crate::narytree::Node;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        Self::_preorder(root).unwrap_or_default()
    }

    pub fn _preorder(root: Option<Rc<RefCell<Node>>>) -> Option<Vec<i32>> {
        let mut ret = vec![];
        let mut stack = std::collections::VecDeque::new();
        if root.is_some() {
            stack.push_back(root);
        }
        while let Some(node) = stack.pop_back() {
            ret.push(node.as_ref()?.borrow().val);
            for child in node.as_ref()?.borrow().children.iter().rev() {
                stack.push_back(child.clone());
            }
        }
        Some(ret)
    }
}

#[test]
fn test() {
    let root = Node::from_vec(vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)]);
    assert_eq!(Solution::preorder(root), vec![1, 3, 5, 6, 2, 4]);

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
    assert_eq!(Solution::preorder(root), vec![1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10]);
}
