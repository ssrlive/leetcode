#![allow(dead_code)]

// 889. Construct Binary Tree from Preorder and Postorder Traversal
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
// https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
//
// Given two integer arrays, preorder and postorder where preorder is the preorder traversal of a binary tree of distinct values
// and postorder is the postorder traversal of the same tree, reconstruct and return the binary tree.
//
// If there exist multiple answers, you can return any of them.
//
// Example 1:
//
// Input: preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
// Output: [1,2,3,4,5,6,7]
//
// Example 2:
//
// Input: preorder = [1], postorder = [1]
// Output: [1]
//
// Constraints:
//
// - 1 <= preorder.length <= 30
// - 1 <= preorder[i] <= preorder.length
// - All the values of preorder are unique.
// - postorder.length == preorder.length
// - 1 <= postorder[i] <= postorder.length
// - All the values of postorder are unique.
// - It is guaranteed that preorder and postorder are the preorder traversal and postorder traversal of the same binary tree.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;
        fn helper(post: &mut HashMap<i32, usize>, preorder: &Vec<i32>, pre_idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            let cur = preorder[*pre_idx];
            let mut n = TreeNode::new(cur);
            *pre_idx += 1;
            if *pre_idx < preorder.len() && post[&preorder[*pre_idx]] < post[&cur] {
                n.left = helper(post, preorder, pre_idx);
            } else {
                return Some(Rc::new(RefCell::new(n)));
            }
            if *pre_idx < preorder.len() && post[&preorder[*pre_idx]] < post[&cur] {
                n.right = helper(post, preorder, pre_idx);
            }
            Some(Rc::new(RefCell::new(n)))
        }

        let mut post = HashMap::new();
        for (i, &v) in postorder.iter().enumerate() {
            post.insert(v, i);
        }
        let mut pre_idx = 0;
        helper(&mut post, &preorder, &mut pre_idx)
    }
}

#[test]
fn test() {
    let preorder = vec![1, 2, 4, 5, 3, 6, 7];
    let postorder = vec![4, 5, 2, 6, 7, 3, 1];
    let root = Solution::construct_from_pre_post(preorder, postorder);
    let expected = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
    assert_eq!(root, expected);

    let preorder = vec![1];
    let postorder = vec![1];
    let root = Solution::construct_from_pre_post(preorder, postorder);
    let expected = TreeNode::from_vec(&[Some(1)]);
    assert_eq!(root, expected);
}
