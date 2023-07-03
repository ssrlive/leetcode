#![allow(dead_code)]

// 106. Construct Binary Tree from Inorder and Postorder Traversal
// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
//
// Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary
// tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.
//
// For example, given
// inorder = [9,3,15,20,7]
// postorder = [9,15,7,20,3]
// Return the following binary tree:
//     3
//    / \
//   9  20
//     /  \
//    15   7
//

use crate::treenode::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree_recursive(inorder: &mut Vec<i32>, postorder: &mut Vec<i32>, bound: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() || (bound.is_some() && *inorder.last()? == bound?) {
                return None;
            }
            let mut root = TreeNode::new(postorder.pop()?);
            root.right = build_tree_recursive(inorder, postorder, Some(root.val));
            inorder.pop();
            root.left = build_tree_recursive(inorder, postorder, bound);
            Some(Rc::new(RefCell::new(root)))
        }

        let mut inorder = inorder;
        let mut postorder = postorder;
        build_tree_recursive(&mut inorder, &mut postorder, None)
    }
}

#[test]
fn test_build_tree() -> Result<(), Box<dyn std::error::Error>> {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let root = Solution::build_tree(inorder, postorder);
    assert_eq!(
        TreeNode::to_vec(&root),
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
    Ok(())
}
