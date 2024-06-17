#![allow(dead_code)]

// 23. Merge k Sorted Lists
// https://leetcode.com/problems/merge-k-sorted-lists/
// https://leetcode.cn/problems/merge-k-sorted-lists/
//
// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
//
// Merge all the linked-lists into one sorted linked-list and return it.
//

use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut head = None;
        let mut tail = &mut head;
        loop {
            let mut min: Option<&Box<ListNode>> = None;
            let mut min_index = 0;
            for (i, item) in lists.iter().enumerate() {
                if let Some(node) = item {
                    if min.is_none() || node.val < min?.val {
                        min = Some(node);
                        min_index = i;
                    }
                }
            }
            if min.is_none() {
                break;
            }
            let mut node = lists[min_index].take()?;
            lists[min_index] = node.next.take();
            *tail = Some(node);
            tail = &mut tail.as_mut()?.next;
        }
        head
    }

    pub fn merge_k_lists_2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Collect all values from all nodes in all lists.
        let mut vals = Vec::new();
        lists.iter().for_each(|list| {
            let mut head = list.as_ref();
            while head.is_some() {
                vals.push(head.unwrap().val);
                head = head.unwrap().next.as_ref();
            }
        });
        vals.sort_unstable();

        // Build the returned list in reverse, starting with None.
        let mut head = None;
        vals.iter().rev().for_each(|&val| {
            head = Some(Box::new(ListNode { val, next: head.take() }));
        });
        head
    }

    pub fn merge_k_lists_3(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn _merge_k_lists_3(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
            let f = |(_, x): &(usize, &Option<Box<ListNode>>)| x.as_ref().map_or(i32::MAX, |x| x.val);
            let index = lists.iter().enumerate().min_by_key(f)?.0;
            let mut head = lists[index].take()?;
            lists[index] = head.next.take();
            head.next = _merge_k_lists_3(lists);
            Some(head)
        }
        _merge_k_lists_3(lists)
    }
}

#[test]
fn test_merge_k_lists() {
    let lists = vec![
        None,
        ListNode::from_vec(&[1, 4, 5]),
        ListNode::from_vec(&[1, 3, 4]),
        ListNode::from_vec(&[2, 6]),
    ];
    let result = ListNode::from_vec(&[1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(Solution::merge_k_lists_3(lists), result);

    let lists = vec![
        None,
        ListNode::from_vec(&[1, 4, 5]),
        ListNode::from_vec(&[1, 3, 4]),
        ListNode::from_vec(&[2, 6]),
    ];
    let result = ListNode::from_vec(&[1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(Solution::merge_k_lists(lists), result);

    let lists = vec![
        ListNode::from_vec(&[1, 4, 5]),
        ListNode::from_vec(&[1, 3, 4]),
        ListNode::from_vec(&[2, 6]),
    ];
    let result = ListNode::from_vec(&[1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(Solution::merge_k_lists_2(lists), result);

    let lists = vec![];
    let result = None;
    assert_eq!(Solution::merge_k_lists_2(lists), result);

    let lists = vec![None];
    let result = None;
    assert_eq!(Solution::merge_k_lists_2(lists), result);
}
