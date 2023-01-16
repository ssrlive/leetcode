#![allow(dead_code)]

// 257. Binary Tree Paths
// https://leetcode.com/problems/binary-tree-paths/
// https://leetcode.cn/problems/binary-tree-paths/
//
// Given the root of a binary tree, return all root-to-leaf paths in any order.
//
// A leaf is a node with no children.
//
// Example 1:
//
// Input: root = [1,2,3,null,5]
// Output: ["1->2->5","1->3"]
//
// Example 2:
//
// Input: root = [1]
// Output: ["1"]
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 100].
// -100 <= Node.val <= 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        if root.is_none() {
            return vec![];
        }
        let root = root.unwrap();
        let left = &(*root).borrow().left;
        let right = &(*root).borrow().right;
        let val = (*root).borrow().val;
        match (left, right) {
            (None, None) => vec![format!("{}", val)],
            (Some(n), None) | (None, Some(n)) => {
                let mut output = Vec::<String>::new();
                for s in Solution::binary_tree_paths(Some(n.clone())) {
                    output.push(format!("{}->{}", val, s));
                }
                output
            }
            (Some(n_left), Some(n_right)) => {
                let mut output = Vec::<String>::new();
                for s in Solution::binary_tree_paths(Some(n_left.clone())).as_slice() {
                    output.push(format!("{}->{}", val, s));
                }
                for s in Solution::binary_tree_paths(Some(n_right.clone())).as_slice() {
                    output.push(format!("{}->{}", val, s));
                }
                output
            }
        }
    }
}

#[test]
fn test_257() {
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), None, Some(5)]);
    assert_eq!(Solution::binary_tree_paths(root), vec!["1->2->5", "1->3"]);

    let root = TreeNode::from_vec(&[Some(1)]);
    assert_eq!(Solution::binary_tree_paths(root), vec!["1"]);
}
