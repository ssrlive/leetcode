#![allow(dead_code)]

// 83. Remove Duplicates from Sorted List
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
//
// Given the head of a sorted linked list, delete all duplicates such that each
// element appears only once. Return the linked list sorted as well.
//

use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn remove(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match node {
                Some(mut node) => {
                    if let Some(next_node) = &node.next {
                        if node.val == next_node.val {
                            remove(node.next.take())
                        } else {
                            Some(Box::new(ListNode {
                                val: node.val,
                                next: remove(node.next.take()),
                            }))
                        }
                    } else {
                        Some(Box::new(ListNode::new(node.val)))
                    }
                }
                None => None,
            }
        }

        remove(head)
    }
}

#[test]
fn test_delete_duplicates() {
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 2])),
        ListNode::from_vec(&[1, 2])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 2, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[
            1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3
        ])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[
            1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3
        ])),
        ListNode::from_vec(&[1, 2, 3])
    );
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_vec(&[
            1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3
        ])),
        ListNode::from_vec(&[1, 2, 3])
    );
}
