#![allow(dead_code)]

// 24. Swap Nodes in Pairs
// https://leetcode.com/problems/swap-nodes-in-pairs/
//
// Given a linked list, swap every two adjacent nodes and return its head.
// You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
//

use super::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn swap_pairs<T: Default + Copy>(
        head: Option<Box<ListNode<T>>>,
    ) -> Option<Box<ListNode<T>>> {
        let mut head = head;
        let mut list = ListNode::new(T::default());
        let mut tail = &mut list.next;
        let mut temp = None;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            match temp.take() {
                None => temp = Some(node),
                Some(temp) => {
                    node.next = Some(temp);
                    *tail = Some(node);
                    tail = &mut tail.as_mut()?.next.as_mut()?.next;
                }
            }
        }
        *tail = temp;
        list.next.take()
    }
}

#[test]
fn test_swap_pairs() {
    assert_eq!(
        Solution::swap_pairs(ListNode::from_vec(&[1, 2, 3, 4])),
        ListNode::from_vec(&[2, 1, 4, 3])
    );
    assert_eq!(
        Solution::swap_pairs(ListNode::from_vec(&[1, 2, 3, 4, 5])),
        ListNode::from_vec(&[2, 1, 4, 3, 5])
    );
    assert_eq!(
        Solution::swap_pairs(ListNode::from_vec(&[1])),
        ListNode::from_vec(&[1])
    );
    assert_eq!(
        Solution::swap_pairs(ListNode::<i32>::from_vec(&[])),
        ListNode::from_vec(&[])
    );
}
