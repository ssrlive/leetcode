#![allow(dead_code)]

// 1290. Convert Binary Number in a Linked List to Integer
// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
// https://leetcode.cn/problems/convert-binary-number-in-a-linked-list-to-integer/
//
// Easy
//
// Given head which is a reference node to a singly-linked list. The value of each node in the
// linked list is either 0 or 1. The linked list holds the binary representation of a number.
//
// Return the decimal value of the number in the linked list.
//
// The most significant bit is at the head of the linked list.
//
// Example 1:
//
// Input: head = [1,0,1]
// Output: 5
// Explanation: (101) in base 2 = (5) in base 10
//
// Example 2:
//
// Input: head = [0]
// Output: 0
//
// Constraints:
//
// -    The Linked List is not empty.
// -    Number of nodes will not exceed 30.
// -    Each node's value is either 0 or 1.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let mut ans = 0;
        while let Some(node) = head {
            ans = ans * 2 + node.val;
            head = node.next;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_decimal_value(ListNode::from_vec(&[1, 0, 1])), 5);
    assert_eq!(Solution::get_decimal_value(ListNode::from_vec(&[0])), 0);
}
