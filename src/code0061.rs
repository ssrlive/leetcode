#![allow(dead_code)]

// 61. Rotate List
// https://leetcode.com/problems/rotate-list/
// https://leetcode.cn/problems/rotate-list/
//
// Given the head of a linked list, rotate the list to the right by k places.
//

use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut len = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            len += 1;
            cur = &node.next;
        }
        if len == 0 || len == 1 {
            return head;
        }
        let k = len - k % len;
        if k == len {
            return head;
        }
        let mut head = head;
        let mut cur = head.as_mut();
        for _ in 0..k - 1 {
            cur = cur?.next.as_mut();
        }
        let mut new_head = cur?.next.take();
        let mut cur = new_head.as_mut();
        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = head;
                break;
            }
            cur = node.next.as_mut();
        }
        new_head
    }
}

#[test]
fn test_rotate_right() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2, 3, 4, 5]), 2),
        ListNode::from_vec(&[4, 5, 1, 2, 3])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[0, 1, 2]), 4),
        ListNode::from_vec(&[2, 0, 1])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 1),
        ListNode::from_vec(&[2, 1])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 2),
        ListNode::from_vec(&[1, 2])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 3),
        ListNode::from_vec(&[2, 1])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 4),
        ListNode::from_vec(&[1, 2])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 5),
        ListNode::from_vec(&[2, 1])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 6),
        ListNode::from_vec(&[1, 2])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 7),
        ListNode::from_vec(&[2, 1])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 8),
        ListNode::from_vec(&[1, 2])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 9),
        ListNode::from_vec(&[2, 1])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 10),
        ListNode::from_vec(&[1, 2])
    );
    assert_eq!(
        Solution::rotate_right(ListNode::from_vec(&[1, 2]), 11),
        ListNode::from_vec(&[2, 1])
    );
}
