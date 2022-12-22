#![allow(dead_code)]

// 2. Add Two Numbers
// https://leetcode.com/problems/add-two-numbers/
// https://leetcode-cn.com/problems/add-two-numbers/
//
// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
// Example 1:
//
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
//
// Example 2:
//
// Input: l1 = [0], l2 = [0]
// Output: [0]
//
// Example 3:
//
// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]
//
// Constraints:
//
// - The number of nodes in each linked list is in the range [1, 100].
// - 0 <= Node.val <= 9
// - It is guaranteed that the list represents a number that does not have leading zeros.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(n) = l1 {
                sum += n.val;
                l1 = n.next;
            }
            if let Some(n) = l2 {
                sum += n.val;
                l2 = n.next;
            }
            carry = sum / 10;
            cur.as_mut()?.val = sum % 10;
            cur.as_mut()?.next = Some(Box::new(ListNode::new(carry)));
            cur = &mut cur.as_mut()?.next;
        }
        if carry == 0 {
            cur.take();
        }
        head
    }
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let res = Solution::add_two_numbers(ListNode::from_vec(&[2, 4, 3]), ListNode::from_vec(&[5, 6, 4]));
    let res = res.ok_or_else(|| "")?.to_vec();
    assert_eq!(res, &[7, 0, 8]);

    let res = Solution::add_two_numbers(ListNode::from_vec(&[0]), ListNode::from_vec(&[0]));
    let res = res.ok_or_else(|| "")?.to_vec();
    assert_eq!(res, &[0]);

    let res = Solution::add_two_numbers(
        ListNode::from_vec(&[9, 9, 9, 9, 9, 9, 9]),
        ListNode::from_vec(&[9, 9, 9, 9]),
    );
    let res = res.ok_or_else(|| "")?.to_vec();
    assert_eq!(res, &[8, 9, 9, 9, 0, 0, 0, 1]);

    Ok(())
}
