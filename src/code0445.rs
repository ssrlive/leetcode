#![allow(dead_code)]

// 445. Add Two Numbers II
// https://leetcode.com/problems/add-two-numbers-ii/
//
// You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
// Example 1:
//
// Input: l1 = [7,2,4,3], l2 = [5,6,4]
// Output: [7,8,0,7]
//
// Example 2:
//
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [8,0,7]
//
// Example 3:
//
// Input: l1 = [0], l2 = [0]
// Output: [0]
//
// Constraints:
//
// - The number of nodes in each linked list is in the range [1, 100].
// - 0 <= Node.val <= 9
// - It is guaranteed that the list represents a number that does not have leading zeros.
//
// Follow up: Could you solve it without reversing the input lists?

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut stack1 = std::collections::VecDeque::new();
        let mut stack2 = std::collections::VecDeque::new();
        while let Some(node) = l1 {
            stack1.push_back(node.val);
            l1 = &node.next;
        }
        while let Some(node) = l2 {
            stack2.push_back(node.val);
            l2 = &node.next;
        }
        let mut carry = 0;
        let mut head = None;
        while !stack1.is_empty() || !stack2.is_empty() || carry > 0 {
            let mut sum = carry;
            if let Some(val) = stack1.pop_back() {
                sum += val;
            }
            if let Some(val) = stack2.pop_back() {
                sum += val;
            }
            carry = sum / 10;
            let mut node = ListNode::new(sum % 10);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

#[test]
fn test() {
    let l1 = ListNode::from_vec(&[7, 2, 4, 3]);
    let l2 = ListNode::from_vec(&[5, 6, 4]);
    let result = ListNode::from_vec(&[7, 8, 0, 7]);
    assert_eq!(Solution::add_two_numbers(l1, l2), result);
}
