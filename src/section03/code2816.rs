#![allow(dead_code)]

// 2816. Double a Number Represented as a Linked List
// https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/
// https://leetcode.cn/problems/double-a-number-represented-as-a-linked-list/
//
// Medium
//
// You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.
//
// Return the head of the linked list after doubling it.
//
// Example 1:
//
// Input: head = [1,8,9]
// Output: [3,7,8]
// Explanation: The figure above corresponds to the given linked list which represents the number 189.
// Hence, the returned linked list represents the number 189 * 2 = 378.
//
// Example 2:
//
// Input: head = [9,9,9]
// Output: [1,9,9,8]
// Explanation: The figure above corresponds to the given linked list which represents the number 999.
// Hence, the returned linked list reprersents the number 999 * 2 = 1998.
//
// Constraints:
//
// The number of nodes in the list is in the range [1, 104]
// 0 <= Node.val <= 9
// The input is generated such that the list represents a number that does not have leading zeros, except the number 0 itself.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut temp = vec![];

        while let Some(node) = head {
            head = node.next;
            temp.push(node.val);
        }

        let mut carry = 0;
        let mut data = vec![];
        while carry > 0 || !temp.is_empty() {
            if let Some(a) = temp.pop() {
                carry += 2 * a;
            }
            data.push(carry % 10);
            carry /= 10;
        }

        let mut ret = None;
        let mut tail = &mut ret;

        while let Some(d) = data.pop() {
            tail = &mut tail.insert(Box::new(ListNode::new(d))).next;
        }

        ret
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[1, 8, 9]);
    let ret = ListNode::from_vec(&[3, 7, 8]);
    assert_eq!(Solution::double_it(head), ret);

    let head = ListNode::from_vec(&[9, 9, 9]);
    let ret = ListNode::from_vec(&[1, 9, 9, 8]);
    assert_eq!(Solution::double_it(head), ret);
}
