#![allow(dead_code)]

// 92. Reverse Linked List II
// https://leetcode.com/problems/reverse-linked-list-ii/
//
// Given the head of a singly linked list and two integers left and right where left <= right,
// reverse the nodes of the list from position left to position right, and return the reversed list.
//
// Example 1:
// Input: head = [1,2,3,4,5], left = 2, right = 4
// Output: [1,4,3,2,5]
//
// Example 2:
// Input: head = [5], left = 1, right = 1
// Output: [5]
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_between<T>(
        head: Option<Box<ListNode<T>>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode<T>>> {
        if left == right {
            return head;
        }
        let mut stack = vec![];
        let mut next = head;
        while let Some(this) = next {
            stack.push(this.val);
            next = this.next;
        }
        stack[left as usize - 1..=right as usize - 1].reverse();
        let mut next = None;
        while let Some(val) = stack.pop() {
            next = Some(Box::new(ListNode { next, val }));
        }
        next
    }
}

#[test]
fn test_reverse_between() {
    assert_eq!(
        Solution::reverse_between(ListNode::from_vec(&[1, 2, 3, 4, 5]), 2, 4),
        ListNode::from_vec(&[1, 4, 3, 2, 5])
    );
    assert_eq!(
        Solution::reverse_between(ListNode::from_vec(&[5]), 1, 1),
        ListNode::from_vec(&[5])
    );
}
