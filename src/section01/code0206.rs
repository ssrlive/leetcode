#![allow(dead_code)]

// 206. Reverse Linked List
// https://leetcode.com/problems/reverse-linked-list/
// https://leetcode.cn/problems/reverse-linked-list/
//
// Given the head of a singly linked list, reverse the list, and return the reversed list.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[test]
fn test_reverse_list() {
    assert_eq!(
        Solution::reverse_list(ListNode::from_vec(&vec![1, 2, 3, 4, 5])),
        ListNode::from_vec(&vec![5, 4, 3, 2, 1])
    );
    assert_eq!(
        Solution::reverse_list(ListNode::from_vec(&vec![1, 2])),
        ListNode::from_vec(&vec![2, 1])
    );
    assert_eq!(Solution::reverse_list(ListNode::from_vec(&vec![])), ListNode::from_vec(&vec![]));
}
