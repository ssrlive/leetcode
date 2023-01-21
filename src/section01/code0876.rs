#![allow(dead_code)]

// 876. Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/
// https://leetcode.cn/problems/middle-of-the-linked-list/
//
// Easy
//
// Given the head of a singly linked list, return the middle node of the linked list.
//
// If there are two middle nodes, return the second middle node.
//
// Example 1:
//
// Input: head = [1,2,3,4,5]
// Output: [3,4,5]
// Explanation: The middle node of the list is node 3.
//
// Example 2:
//
// Input: head = [1,2,3,4,5,6]
// Output: [4,5,6]
// Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
//
// Constraints:
//
// - The number of nodes in the list is in the range [1, 100].
// - 1 <= Node.val <= 100
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref()?.next.is_some() {
            slow = &slow.as_ref()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }
        slow.clone()
    }
}

#[test]
fn test_middle_node() {
    let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
    let answer = ListNode::from_vec(&[3, 4, 5]);
    assert_eq!(Solution::middle_node(head), answer);

    let head = ListNode::from_vec(&[1, 2, 3, 4, 5, 6]);
    let answer = ListNode::from_vec(&[4, 5, 6]);
    assert_eq!(Solution::middle_node(head), answer);
}
