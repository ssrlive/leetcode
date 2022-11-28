#![allow(dead_code)]

// 450. Delete Node in a BST
// https://leetcode.com/problems/delete-node-in-a-bst/
//
// Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
//
// Basically, the deletion can be divided into two stages:
//
// Search for a node to remove.
// If the node is found, delete the node.
//
// Example 1:
//
// Input: root = [5,3,6,2,4,null,7], key = 3
// Output: [5,4,6,2,null,null,7]
// Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
// One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
// Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.
//
// Example 2:
//
// Input: root = [5,3,6,2,4,null,7], key = 0
// Output: [5,3,6,2,4,null,7]
// Explanation: The tree does not contain a node with value = 0.
//
// Example 3:
//
// Input: root = [], key = 0
// Output: []
//
// Constraints:
//
// - The number of nodes in the tree is in the range [0, 104].
// - -105 <= Node.val <= 105
// - Each node has a unique value.
// - root is a valid binary search tree.
// - -105 <= key <= 105
//
// Follow up: Could you solve it with time complexity O(height of tree)?
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            if r.borrow().val.cmp(&key).is_eq() {
                let mut r = r.borrow_mut();
                if r.left.as_ref().is_none() && r.right.as_ref().is_none() {
                    return None;
                } else if r.left.as_ref().is_some() && r.right.as_ref().is_none() {
                    return r.left.take();
                } else if r.left.as_ref().is_none() && r.right.as_ref().is_some() {
                    return r.right.take();
                } else {
                    let mini = Self::min(r.right.clone()).unwrap().borrow().val;
                    r.val = mini;
                    r.right = Self::delete_node(r.right.clone(), mini);
                }
            } else if r.borrow().val.cmp(&key).is_gt() {
                let mut t = r.borrow_mut();
                t.left = Self::delete_node(t.left.clone(), key);
            } else {
                let mut t = r.borrow_mut();
                t.right = Self::delete_node(t.right.clone(), key);
            }
            return Some(r);
        }
        None
    }
    fn min(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut temp = None;
        while root.as_ref().is_some() {
            temp = root.clone();
            root = root?.borrow_mut().left.clone();
        }
        temp
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    let key = 3;
    let expected = TreeNode::from_vec(&[Some(5), Some(4), Some(6), Some(2), None, None, Some(7)]);
    assert_eq!(Solution::delete_node(root, key), expected);

    let root = TreeNode::from_vec(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    let key = 0;
    let expected = TreeNode::from_vec(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    assert_eq!(Solution::delete_node(root, key), expected);

    let root = TreeNode::from_vec(&[]);
    let key = 0;
    let expected = TreeNode::from_vec(&[]);
    assert_eq!(Solution::delete_node(root, key), expected);
}
