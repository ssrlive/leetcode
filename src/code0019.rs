#![allow(dead_code)]

// 19. Remove Nth Node From End of List
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//
// Given the head of a linked list, remove the nth node from the end of the list and return its head.
//
// Follow up: Could you do this in one pass?

use super::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let cnt = std::iter::successors(head.as_ref(), |last| last.next.as_ref()).count();
        if cnt < n as usize {
            return head;
        }
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = (0..cnt - n as usize).fold(dummy.as_mut(), |curr, _| curr?.next.as_mut());
        prev?.next = prev.as_mut()?.next.as_mut()?.next.take();
        dummy?.next.take()
    }
}

#[test]
fn test_remove_nth_from_end() {
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from_vec(&vec![1, 2, 3, 4, 5]), 2),
        ListNode::from_vec(&vec![1, 2, 3, 5])
    );
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from_vec(&vec![1]), 1),
        ListNode::from_vec(&vec![])
    );
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from_vec(&vec![1, 2]), 1),
        ListNode::from_vec(&vec![1])
    );
}
