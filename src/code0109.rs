#![allow(dead_code)]

// 109. Convert Sorted List to Binary Search Tree
// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// https://leetcode.cn/problems/convert-sorted-list-to-binary-search-tree/
//
// Given a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
//
// For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
//
// Example:
//
// Given the sorted linked list: [-10,-3,0,5,9],
//
// One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
//
//       0
//      / \
//    -3   9
//    /   /
//  -10  5

use crate::listnode::ListNode;
use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
            if head == tail {
                return None;
            }
            let mut slow = &head;
            let mut fast = &head;
            while fast != &tail && fast.as_ref()?.next != tail {
                slow = &slow.as_ref()?.next;
                fast = &fast.as_ref()?.next.as_ref()?.next;
            }
            let mut node = TreeNode::new(slow.as_ref()?.val);
            node.left = helper(head.clone(), slow.clone());
            node.right = helper(slow.as_ref()?.next.clone(), tail);
            Some(Rc::new(RefCell::new(node)))
        }
        helper(head, None)
    }
}

#[test]
fn test_sorted_list_to_bst() -> Result<(), Box<dyn std::error::Error>> {
    let head = ListNode::from_vec(&[-10, -3, 0, 5, 9]);
    let tree = TreeNode::from_vec(&[Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]);
    assert_eq!(Solution::sorted_list_to_bst(head), tree);
    assert_eq!(
        tree.as_ref().ok_or("")?.borrow().preorder_traversal(),
        vec![0, -3, -10, 9, 5]
    );
    assert_eq!(
        tree.as_ref().ok_or("")?.borrow().inorder_traversal(),
        vec![-10, -3, 0, 5, 9]
    );
    assert_eq!(
        tree.as_ref().ok_or("")?.borrow().postorder_traversal(),
        vec![-10, -3, 5, 9, 0]
    );
    Ok(())
}
