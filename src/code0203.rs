#![allow(dead_code)]

// 203. Remove Linked List Elements
// https://leetcode.com/problems/remove-linked-list-elements/
//
// Given the head of a linked list and an integer val,
// remove all the nodes of the linked list that has Node.val == val,
// and return the new head.
//

use super::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut walker = &mut head;
        loop {
            match walker {
                None => break,
                Some(node) if node.val == val => {
                    *walker = node.next.take();
                }
                Some(node) => {
                    walker = &mut node.next;
                }
            }
        }
        head
    }
}

#[test]
fn test_remove_elements() {
    assert_eq!(
        Solution::remove_elements(ListNode::from_vec(vec![1, 2, 6, 3, 4, 5, 6]), 6),
        ListNode::from_vec(vec![1, 2, 3, 4, 5])
    );
    assert_eq!(
        Solution::remove_elements(ListNode::from_vec(vec![1]), 1),
        ListNode::from_vec(vec![])
    );
    assert_eq!(
        Solution::remove_elements(ListNode::from_vec(vec![1, 1]), 1),
        ListNode::from_vec(vec![])
    );
    assert_eq!(
        Solution::remove_elements(ListNode::from_vec(vec![7, 7, 7, 7]), 7),
        ListNode::from_vec(vec![])
    );
}
