#![allow(dead_code)]

// 1008. Construct Binary Search Tree from Preorder Traversal
// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
// https://leetcode.cn/problems/construct-binary-search-tree-from-preorder-traversal/
//
// Given an array of integers preorder, which represents the preorder traversal of a BST (i.e., binary search tree), construct the tree and return its root.
//
// It is guaranteed that there is always possible to find a binary search tree with the given requirements for the given test cases.
//
// A binary search tree is a binary tree where for every node, any descendant of Node.left has a value strictly less than Node.val, and any descendant of Node.right has a value strictly greater than Node.val.
//
// A preorder traversal of a binary tree displays the value of the node first, then traverses Node.left, then traverses Node.right.
//
// Example 1:
//
// Input: preorder = [8,5,1,7,10,12]
// Output: [8,5,10,1,7,null,12]
//
// Example 2:
//
// Input: preorder = [1,3]
// Output: [1,null,3]
//
// Constraints:
//
// - 1 <= preorder.length <= 100
// - 1 <= preorder[i] <= 1000
// - All the values of preorder are unique.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(first) = v.first() {
                let node = Rc::new(RefCell::new(TreeNode::new(*first)));
                let i = (1..v.len()).find(|&i| v[i] > v[0]).unwrap_or(v.len());
                node.borrow_mut().left = helper(&v[1..i]);
                node.borrow_mut().right = helper(&v[i..]);
                Some(node)
            } else {
                None
            }
        }
        helper(&preorder)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![8, 5, 1, 7, 10, 12],
            vec![Some(8), Some(5), Some(10), Some(1), Some(7), None, Some(12)],
        ),
        (vec![1, 3], vec![Some(1), None, Some(3)]),
    ];
    for (preorder, expected) in cases {
        assert_eq!(Solution::bst_from_preorder(preorder), TreeNode::from_vec(&expected));
    }
}
