#![allow(dead_code)]

// 21. Merge Two Sorted Lists
// https://leetcode.com/problems/merge-two-sorted-lists/
//
// You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.

use super::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
        let mut curr = dummy.as_mut();
        let mut list1 = list1;
        let mut list2 = list2;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref()?.val < list2.as_ref()?.val {
                let next = list1.as_mut()?.next.take();
                curr.as_mut()?.next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut()?.next.take();
                curr.as_mut()?.next = list2;
                list2 = next;
            }
            curr = curr?.next.as_mut();
        }
        if list1.is_some() {
            curr.as_mut()?.next = list1;
        }
        if list2.is_some() {
            curr.as_mut()?.next = list2;
        }
        dummy?.next
    }

    pub fn merge_two_lists_2(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn _merge_two_lists(
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match (list1, list2) {
                (None, None) => None,
                (Some(x), None) | (None, Some(x)) => Some(x),
                (Some(mut x), Some(mut y)) => {
                    if x.val >= y.val {
                        y.next = _merge_two_lists(Some(x), y.next);
                        Some(y)
                    } else {
                        x.next = _merge_two_lists(Some(y), x.next);
                        Some(x)
                    }
                }
            }
        }
        _merge_two_lists(list1, list2)
    }
}

#[test]
fn test() {
    let l1 = ListNode::from_vec(vec![1, 2, 4]);
    let l2 = ListNode::from_vec(vec![1, 3, 4]);
    let l3 = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]);
    assert_eq!(Solution::merge_two_lists_2(l1, l2), l3);

    let l1 = ListNode::from_vec(vec![]);
    let l2 = ListNode::from_vec(vec![]);
    let l3 = ListNode::from_vec(vec![]);
    assert_eq!(Solution::merge_two_lists_2(l1, l2), l3);

    let l1 = ListNode::from_vec(vec![]);
    let l2 = ListNode::from_vec(vec![0]);
    let l3 = ListNode::from_vec(vec![0]);
    assert_eq!(Solution::merge_two_lists_2(l1, l2), l3);
}
