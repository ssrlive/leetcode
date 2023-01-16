#![allow(dead_code)]

// 105. Construct Binary Tree from Preorder and Inorder Traversal
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
// https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
//
// Given preorder and inorder traversal of a tree, construct the binary tree.
//
// Note:
// You may assume that duplicates do not exist in the tree.
//
// For example, given
//
// preorder = [3,9,20,15,7]
// inorder = [9,3,15,20,7]
//
// Return the following binary tree:
//
//     3
//    / \
//   9  20
//     /  \
//    15   7
//

use crate::treenode::TreeNode;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &mut Vec<i32>, inorder: &mut Vec<i32>, bound: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() || (bound.is_some() && *inorder.last()? == bound?) {
                return None;
            }
            let mut root = TreeNode::new(preorder.pop()?);
            root.left = build(preorder, inorder, Some(root.val));
            inorder.pop();
            root.right = build(preorder, inorder, bound);
            Some(Rc::new(RefCell::new(root)))
        }

        let mut preorder = preorder;
        preorder.reverse();
        let mut inorder = inorder;
        inorder.reverse();
        build(&mut preorder, &mut inorder, None)
    }
}

#[test]
fn test_build_tree() -> Result<(), Box<dyn std::error::Error>> {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let root = Solution::build_tree(preorder, inorder);
    assert_eq!(
        root.as_ref().ok_or("")?.borrow().to_vec(),
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
    Ok(())
}
