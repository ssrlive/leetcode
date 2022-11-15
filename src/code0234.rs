#![allow(dead_code)]

// 234. Palindrome Linked List
// https://leetcode.com/problems/palindrome-linked-list/
//
// Given the head of a singly linked list, return true if it is a palindrome or false otherwise.
//
// Example 1:
// Input: head = [1,2,2,1]
// Output: true
//
// Example 2:
// Input: head = [1,2]
// Output: false
//
// Constraints:
// The number of nodes in the list is in the range [1, 105].
// 0 <= Node.val <= 9
//
// Follow up: Could you do it in O(n) time and O(1) space?
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let (mut v, mut i) = ([0; 100000], 0);
        let mut head = &head;
        while let Some(node) = head {
            v[i] = node.val;
            head = &node.next;
            i += 1;
        }
        v.iter().take(i).rev().take(i / 2).eq(v.iter().take(i / 2))
    }
}

#[test]
fn test_is_palindrome() {
    assert_eq!(Solution::is_palindrome(ListNode::from_vec(&[1, 2, 2, 1])), true);
    assert_eq!(Solution::is_palindrome(ListNode::from_vec(&[1, 2])), false);
}
