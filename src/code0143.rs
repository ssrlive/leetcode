#![allow(dead_code)]

// 143. Reorder List
// https://leetcode.com/problems/reorder-list/
// https://leetcode.cn/problems/reorder-list/
//
// You are given the head of a singly linked-list. The list can be represented as:
//
// L0 → L1 → … → Ln - 1 → Ln
//
// Reorder the list to be on the following form:
//
// L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
//
// You may not modify the values in the list's nodes. Only nodes themselves may be changed.

use super::listnode::ListNode;

struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Find how many nodes are in the list;
        let mut count = 0;
        let mut list = &*head;

        while let Some(node) = list {
            list = &node.next;
            count += 1;
        }

        // If there are less than two nodes, then there is nothing to do
        if count <= 2 {
            return;
        }

        // Reach the middle of the list in order to split in to two lists
        let mut half = head.as_mut();
        for _ in 0..count / 2 {
            match half {
                // SAFETY: we have counted the number of nodes, so we know there are more nodes
                None => unsafe { std::hint::unreachable_unchecked() },
                Some(node) => {
                    half = node.next.as_mut();
                }
            }
        }

        // Reverse the second half
        let mut half = match half {
            // SAFETY: we have counted the number of nodes, so we know there are more nodes
            None => unsafe { std::hint::unreachable_unchecked() },
            Some(node) => node.next.take(),
        };

        let mut reversed = ListNode::new(0);
        while let Some(mut node) = half.take() {
            half = node.next.take();
            node.next = reversed.next.take();
            reversed.next = Some(node);
        }

        // merge the two lists, tail points to the node in the first list
        // that has to be disconnected in order to put a node from the reversed
        // list in its place. Then it's reattached as "next" of the new node
        let mut tail = match head.as_mut() {
            // SAFETY: we know there is node at HEAD
            None => unsafe { std::hint::unreachable_unchecked() },
            Some(node) => &mut node.next,
        };

        while tail.is_some() && reversed.next.is_some() {
            // SAFETY: We know there is a reversed.next because we already checked it
            let mut rev = reversed.next.take().unwrap();
            reversed.next = rev.next.take();

            rev.next = tail.take();
            *tail = Some(rev);
            tail = &mut tail.as_mut().unwrap().next;
            if let Some(node) = tail {
                tail = &mut node.next;
            }
        }
    }
}

#[test]
fn test_reorder_list() {
    let mut list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
    Solution::reorder_list(&mut list);
    assert_eq!(list, ListNode::from_vec(&vec![1, 5, 2, 4, 3]));

    let mut list = ListNode::from_vec(&vec![1, 2, 3, 4]);
    Solution::reorder_list(&mut list);
    assert_eq!(list, ListNode::from_vec(&vec![1, 4, 2, 3]));

    let mut list = ListNode::from_vec(&vec![1, 2, 3, 4, 5, 6]);
    Solution::reorder_list(&mut list);
    assert_eq!(list, ListNode::from_vec(&vec![1, 6, 2, 5, 3, 4]));

    let mut list = ListNode::from_vec(&vec![1, 2, 3, 4, 5, 6, 7]);
    Solution::reorder_list(&mut list);
    assert_eq!(list, ListNode::from_vec(&vec![1, 7, 2, 6, 3, 5, 4]));
}
