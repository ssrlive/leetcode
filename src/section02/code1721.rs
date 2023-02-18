#![allow(dead_code)]

/*

// 1721. Swapping Nodes in a Linked List
Medium
3.6K
122
Companies

You are given the head of a linked list, and an integer k.

Return the head of the linked list after swapping the values of the kth node from the beginning and the kth node from the end (the list is 1-indexed).

Example 1:

Input: head = [1,2,3,4,5], k = 2
Output: [1,4,3,2,5]

Example 2:

Input: head = [7,9,6,6,7,8,3,0,9,5], k = 5
Output: [7,9,6,6,8,7,3,0,9,5]

Constraints:

    The number of nodes in the list is n.
    1 <= k <= n <= 10^5
    0 <= Node.val <= 100
*/

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        {
            let mut node = &head;
            while let Some(n) = node {
                v.push(n.val);
                node = &n.next;
            }
        }
        let len = v.len();
        v.swap(k as usize - 1, len - k as usize);
        let mut answer = None;
        for &n in v.iter().rev() {
            answer = Some(Box::new(ListNode { val: n, next: answer }));
        }
        answer
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
    let k = 2;
    let answer = ListNode::from_vec(&[1, 4, 3, 2, 5]);
    assert_eq!(Solution::swap_nodes(head, k), answer);

    let head = ListNode::from_vec(&[7, 9, 6, 6, 7, 8, 3, 0, 9, 5]);
    let k = 5;
    let answer = ListNode::from_vec(&[7, 9, 6, 6, 8, 7, 3, 0, 9, 5]);
    assert_eq!(Solution::swap_nodes(head, k), answer);
}
