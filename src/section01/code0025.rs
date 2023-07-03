#![allow(dead_code)]

// 25. Reverse Nodes in k-Group
// https://leetcode.com/problems/reverse-nodes-in-k-group/
// https://leetcode.cn/problems/reverse-nodes-in-k-group/
//
// Given a linked list, reverse the nodes of a linked list k at a time and
// return its modified list.
//
// k is a positive integer and is less than or equal to the length of the
// linked list. If the number of nodes is not a multiple of k then left-out
// nodes in the end should remain as it is.
//
// Example:
//
// Given this linked list: 1->2->3->4->5
//
// For k = 2, you should return: 2->1->4->3->5
//
// For k = 3, you should return: 3->2->1->4->5
//
// Note:
//
// Only constant extra memory is allowed.
// You may not alter the values in the list's nodes, only nodes itself may be
// changed.

use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        pub fn _reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            let mut head = head;
            let mut tail = &mut head;
            let mut count = 0;
            while count < k {
                if tail.is_none() {
                    return head;
                }
                tail = &mut tail.as_mut()?.next;
                count += 1;
            }
            let mut new_head = _reverse_k_group(tail.take(), k);
            while count > 0 {
                let mut node = head.take();
                head = node.as_mut()?.next.take();
                node.as_mut()?.next = new_head;
                new_head = node;
                count -= 1;
            }
            new_head
        }
        _reverse_k_group(head, k)
    }
}

#[test]
fn test_reverse_k_group() {
    assert_eq!(
        Solution::reverse_k_group(ListNode::from_vec(&[1, 2, 3, 4, 5]), 2),
        ListNode::from_vec(&[2, 1, 4, 3, 5])
    );
    assert_eq!(
        Solution::reverse_k_group(ListNode::from_vec(&[1, 2, 3, 4, 5]), 3),
        ListNode::from_vec(&[3, 2, 1, 4, 5])
    );
    assert_eq!(
        Solution::reverse_k_group(ListNode::from_vec(&[1, 2, 3, 4, 5]), 1),
        ListNode::from_vec(&[1, 2, 3, 4, 5])
    );
    assert_eq!(Solution::reverse_k_group(ListNode::from_vec(&[1]), 1), ListNode::from_vec(&[1]));
    assert_eq!(Solution::reverse_k_group(ListNode::from_vec(&[]), 1), ListNode::from_vec(&[]));
}
