#![allow(dead_code)]

// 86. Partition List
// https://leetcode.com/problems/partition-list/

use super::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode<i32>>>, x: i32) -> Option<Box<ListNode<i32>>> {
        let mut head = head;
        let mut less_head = Some(Box::new(ListNode::new(0)));
        let mut less_tail = &mut less_head;
        let mut greater_head = Some(Box::new(ListNode::new(0)));
        let mut greater_tail = &mut greater_head;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                less_tail.as_mut()?.next = Some(node);
                less_tail = &mut less_tail.as_mut()?.next;
            } else {
                greater_tail.as_mut()?.next = Some(node);
                greater_tail = &mut greater_tail.as_mut()?.next;
            }
        }
        less_tail.as_mut()?.next = greater_head?.next;
        less_head?.next
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&vec![1, 4, 3, 2, 5, 2]);
    let x = 3;
    let res = ListNode::from_vec(&vec![1, 2, 2, 4, 3, 5]);
    assert_eq!(Solution::partition(head, x), res);
}
