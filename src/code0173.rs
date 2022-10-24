#![allow(dead_code)]

// 173. Binary Search Tree Iterator
// https://leetcode.com/problems/binary-search-tree-iterator/
//
// Implement the BSTIterator class that represents an iterator over the in-order traversal of a binary search tree (BST):
//
// BSTIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor.
// The pointer should be initialized to a non-existent number smaller than any element in the BST.
// boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
// int next() Moves the pointer to the right, then returns the number at the pointer.
//
// Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.
//
// You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.
//

use super::treenode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        let mut node = root;
        while let Some(n) = node {
            stack.push(n.clone());
            node = n.borrow().left.clone();
        }
        BSTIterator { stack }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let node = node.borrow_mut();
        let result = node.val;
        let mut node = node.right.clone();
        while let Some(n) = node {
            self.stack.push(n.clone());
            node = n.borrow().left.clone();
        }
        result
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[test]
fn test_bst_iterator() {
    let root = TreeNode::from_vec(vec![
        Some(7),
        Some(3),
        Some(15),
        None,
        None,
        Some(9),
        Some(20),
    ]);
    let mut iterator = BSTIterator::new(root);
    assert_eq!(iterator.next(), 3);
    assert_eq!(iterator.next(), 7);
    assert_eq!(iterator.has_next(), true);
    assert_eq!(iterator.next(), 9);
    assert_eq!(iterator.has_next(), true);
    assert_eq!(iterator.next(), 15);
    assert_eq!(iterator.has_next(), true);
    assert_eq!(iterator.next(), 20);
    assert_eq!(iterator.has_next(), false);
}
