#![allow(dead_code)]

// 1367. Linked List in Binary Tree
// https://leetcode.com/problems/linked-list-in-binary-tree/
// https://leetcode.cn/problems/linked-list-in-binary-tree/
//
// Medium
//
// Given a binary tree root and a linked list with head as the first node.
//
// Return True if all the elements in the linked list starting from the head correspond
// to some downward path connected in the binary tree otherwise return False.
//
// In this context downward path means a path that starts at some node and goes downwards.
//
// Example 1:
//
// Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
// Output: true
// Explanation: Nodes in blue form a subpath in the binary Tree.
//
// Example 2:
//
// Input: head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
// Output: true
//
// Example 3:
//
// Input: head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
// Output: false
// Explanation: There is no path in the binary tree that contains all the elements of the linked list from head.
//
// Constraints:
//
// -    The number of nodes in the tree will be in the range [1, 2500].
// -    The number of nodes in the list will be in the range [1, 100].
// -    1 <= Node.val <= 100 for each node in the linked list and binary tree.
//

use crate::listnode::ListNode;
use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
        type OptListNode = Option<Box<ListNode>>;

        fn _is_sub_path(head: &OptListNode, root: &OptTreeNode) -> Option<bool> {
            if head.is_none() {
                return Some(true);
            }
            if root.is_none() {
                return Some(false);
            }
            let root_ref = root.as_ref()?.borrow();
            let v = dfs(head, root)? || _is_sub_path(head, &root_ref.left)? || _is_sub_path(head, &root_ref.right)?;
            Some(v)
        }

        fn dfs(head: &OptListNode, root: &OptTreeNode) -> Option<bool> {
            if head.is_none() {
                return Some(true);
            }
            if root.is_none() {
                return Some(false);
            }
            let root_ref = root.as_ref()?.borrow();
            let v = head.as_ref()?.val == root_ref.val
                && (dfs(&head.as_ref()?.next, &root_ref.left)? || dfs(&head.as_ref()?.next, &root_ref.right)?);
            Some(v)
        }

        _is_sub_path(&head, &root).unwrap_or(false)
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[4, 2, 8]);
    let root = TreeNode::from_string("[1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]");
    assert!(Solution::is_sub_path(head, root));

    let head = ListNode::from_vec(&[1, 4, 2, 6]);
    let root = TreeNode::from_string("[1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]");
    assert!(Solution::is_sub_path(head, root));

    let head = ListNode::from_vec(&[1, 4, 2, 6, 8]);
    let root = TreeNode::from_string("[1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]");
    assert!(!Solution::is_sub_path(head, root));
}
