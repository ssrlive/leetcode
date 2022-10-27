#![allow(dead_code)]

// 82. Remove Duplicates from Sorted List II
// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
//
// Given the head of a sorted linked list, delete all nodes that have duplicate
// numbers, leaving only distinct numbers from the original list. Return the
// linked list sorted as well.
//

use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates<T: PartialEq>(
        head: Option<Box<ListNode<T>>>,
    ) -> Option<Box<ListNode<T>>> {
        match head {
            None => None,
            Some(mut n) => {
                match n.next {
                    None => Some(n),
                    Some(n2) if n2.val != n.val => {
                        n.next = Self::delete_duplicates(Some(n2));
                        Some(n)
                    }
                    Some(mut n2) => {
                        // n is a dupe, discard n and find the next node with different value
                        while n2.val == n.val {
                            match n2.next {
                                None => return None,
                                Some(n3) => n2 = n3,
                            };
                        }
                        Self::delete_duplicates(Some(n2))
                    }
                }
            }
        }
    }
}

#[test]
fn test_delete_duplicates() {
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 2, 3, 3, 4, 4, 5])),
        ListNode::from_vec(&[1, 2, 5])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 3])),
        ListNode::from_vec(&[2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3])),
        ListNode::from_vec(&[3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3, 3])),
        ListNode::from_vec(&[])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3, 3, 3])),
        ListNode::from_vec(&[])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3, 3, 3, 3])),
        ListNode::from_vec(&[])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[])
    );
}
