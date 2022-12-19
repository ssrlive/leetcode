#![allow(dead_code)]

// 700. Search in a Binary Search Tree
// https://leetcode.com/problems/search-in-a-binary-search-tree/
// https://leetcode.cn/problems/search-in-a-binary-search-tree/
//
// You are given the root of a binary search tree (BST) and an integer val.
//
// Find the node in the BST that the node's value equals val and return the subtree rooted with that node. If such a node does not exist, return null.
//
// Example 1:
//
// Input: root = [4,2,7,1,3], val = 2
// Output: [2,1,3]
//
// Example 2:
//
// Input: root = [4,2,7,1,3], val = 5
// Output: []
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 5000].
// - 1 <= Node.val <= 10^7
// - root is a binary search tree.
// - 1 <= val <= 10^7
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        pub fn _search_bst(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            node.as_ref()?;
            let node_val = node.as_ref()?.borrow().val;
            match node_val.cmp(&val) {
                std::cmp::Ordering::Equal => node,
                std::cmp::Ordering::Less => _search_bst(node.as_ref()?.borrow().right.clone(), val),
                std::cmp::Ordering::Greater => _search_bst(node.as_ref()?.borrow().left.clone(), val),
            }
        }

        _search_bst(root, val)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(4), Some(2), Some(7), Some(1), Some(3)]);
    let val = 2;
    let expected = TreeNode::from_vec(&[Some(2), Some(1), Some(3)]);
    assert_eq!(Solution::search_bst(root, val), expected);
}
