#![allow(dead_code)]

// 99. Recover Binary Search Tree
// https://leetcode.com/problems/recover-binary-search-tree/
//
// You are given the root of a binary search tree (BST),
// where the values of exactly two nodes of the tree were swapped by mistake.
// Recover the tree without changing its structure.
//
// Follow up: A solution using O(n) space is pretty straight forward.
// Could you devise a constant space solution?

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn inorder(n: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = n {
                inorder(n.borrow().left.clone(), v);
                v.push(n.clone());
                inorder(n.borrow().right.clone(), v);
            }
        }
        let mut v = vec![];
        inorder(root.clone(), &mut v);
        let mut i = 0;
        let mut j = v.len() - 1;
        while i < v.len() - 1 && v[i].borrow().val < v[i + 1].borrow().val {
            i += 1;
        }
        while j > 0 && v[j].borrow().val > v[j - 1].borrow().val {
            j -= 1;
        }
        if i < v.len() && j > 0 {
            let tmp = v[i].borrow().val;
            v[i].borrow_mut().val = v[j].borrow().val;
            v[j].borrow_mut().val = tmp;
        }
    }

    pub fn recover_tree_2(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        let mut curr = root.clone();
        let mut x = None;
        let mut y = None;
        let mut pred: Option<Rc<RefCell<TreeNode>>> = None;

        while !(stack.is_empty() && curr.is_none()) {
            while let Some(node) = curr {
                curr = node.borrow_mut().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                if let Some(p) = pred {
                    if p.borrow_mut().val > node.borrow_mut().val {
                        y = Some(node.clone());
                        if x.is_none() {
                            x = Some(p);
                        } else {
                            break;
                        }
                    }
                }
                pred = Some(node.clone());
                curr = node.borrow_mut().right.clone();
            }
        }

        let mut x = x.as_ref().unwrap().borrow_mut();
        let mut y = y.as_ref().unwrap().borrow_mut();
        std::mem::swap(&mut x.val, &mut y.val);
    }
}

#[test]
fn test_recover_tree() {
    let mut tree = TreeNode::from_vec(&[Some(1), Some(3), None, None, Some(2)]);
    Solution::recover_tree_2(&mut tree);
    assert_eq!(
        tree,
        TreeNode::from_vec(&[Some(3), Some(1), None, None, Some(2)])
    );
}
