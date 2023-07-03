#![allow(dead_code)]

/*

// 1669. Merge In Between Linked Lists
// https://leetcode.com/problems/merge-in-between-linked-lists/
// https://leetcode.cn/problems/merge-in-between-linked-lists/
//
// Medium
//
// You are given two linked lists: list1 and list2 of sizes n and m respectively.

Remove list1's nodes from the ath node to the bth node, and put list2 in their place.

The blue edges and nodes in the following figure indicate the result:

Build the result list and return its head.

Example 1:

Input: list1 = [0,1,2,3,4,5], a = 3, b = 4, list2 = [1000000,1000001,1000002]
Output: [0,1,2,1000000,1000001,1000002,5]
Explanation: We remove the nodes 3 and 4 and put the entire list2 in their place. The blue edges and nodes in the above figure indicate the result.

Example 2:

Input: list1 = [0,1,2,3,4,5,6], a = 2, b = 5, list2 = [1000000,1000001,1000002,1000003,1000004]
Output: [0,1,1000000,1000001,1000002,1000003,1000004,6]
Explanation: The blue edges and nodes in the above figure indicate the result.

Constraints:

    3 <= list1.length <= 10^4
    1 <= a <= b < list1.length - 1
    1 <= list2.length <= 10^4
*/

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut cur1 = &mut list1;
        for _ in 0..a - 1 {
            cur1 = &mut cur1.as_deref_mut().unwrap().next;
        }

        let mut rest = std::mem::replace(&mut cur1.as_deref_mut().unwrap().next, list2);

        while cur1.as_deref_mut().unwrap().next.is_some() {
            cur1 = &mut cur1.as_deref_mut().unwrap().next;
        }

        for _ in 0..=b - a {
            rest = rest.unwrap().next;
        }

        cur1.as_deref_mut().unwrap().next = rest;

        list1
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            ListNode::from_vec(&vec![0, 1, 2, 3, 4, 5]),
            3,
            4,
            ListNode::from_vec(&vec![1000000, 1000001, 1000002]),
            ListNode::from_vec(&vec![0, 1, 2, 1000000, 1000001, 1000002, 5]),
        ),
        (
            ListNode::from_vec(&vec![0, 1, 2, 3, 4, 5, 6]),
            2,
            5,
            ListNode::from_vec(&vec![1000000, 1000001, 1000002, 1000003, 1000004]),
            ListNode::from_vec(&vec![0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6]),
        ),
    ];
    for (list1, a, b, list2, e) in cases {
        assert_eq!(Solution::merge_in_between(list1, a, b, list2), e);
    }
}
