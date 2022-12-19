#![allow(dead_code)]

// 725. Split Linked List in Parts
// https://leetcode.com/problems/split-linked-list-in-parts/
// https://leetcode.cn/problems/split-linked-list-in-parts/
//
// Given the head of a singly linked list and an integer k, split the linked list into k consecutive linked list parts.
//
// The length of each part should be as equal as possible: no two parts should have a size differing by more than one.
// This may lead to some parts being null.
//
// The parts should be in the order of occurrence in the input list, and parts occurring earlier should always
// have a size greater than or equal to parts occurring later.
//
// Return an array of the k parts.
//
// Example 1:
//
// Input: head = [1,2,3], k = 5
// Output: [[1],[2],[3],[],[]]
// Explanation:
// The first element output[0] has output[0].val = 1, output[0].next = null.
// The last element output[4] is null, but its string representation as a ListNode is [].
//
// Example 2:
//
// Input: head = [1,2,3,4,5,6,7,8,9,10], k = 3
// Output: [[1,2,3,4],[5,6,7],[8,9,10]]
// Explanation:
// The input has been split into consecutive parts with size difference at most 1, and earlier parts are a larger size than the later parts.
//
// Constraints:
//
// - The number of nodes in the list is in the range [0, 1000].
// - 0 <= Node.val <= 1000
// - 1 <= k <= 50
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        fn _split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Option<Vec<Option<Box<ListNode>>>> {
            let mut len = 0;
            let mut node = &head;
            while let Some(n) = node {
                len += 1;
                node = &n.next;
            }
            let mut head = head;
            let mut answer = Vec::with_capacity(k as usize);
            for i in 0..k as usize {
                answer.push(head);
                let mut node = answer.get_mut(i)?;
                let len2 = len / k + i32::from(i < (len % k) as usize);
                for _ in 0..len2 {
                    if let Some(n) = node {
                        node = &mut n.next;
                    }
                }
                head = node.take();
            }
            Some(answer)
        }
        _split_list_to_parts(head, k).unwrap_or_default()
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[1, 2, 3]);
    let k = 5;
    let answer = vec![
        ListNode::from_vec(&[1]),
        ListNode::from_vec(&[2]),
        ListNode::from_vec(&[3]),
        ListNode::from_vec(&[]),
        ListNode::from_vec(&[]),
    ];
    let res = Solution::split_list_to_parts(head, k);
    assert_eq!(res, answer);

    let head = ListNode::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let k = 3;
    let answer = vec![
        ListNode::from_vec(&[1, 2, 3, 4]),
        ListNode::from_vec(&[5, 6, 7]),
        ListNode::from_vec(&[8, 9, 10]),
    ];
    let res = Solution::split_list_to_parts(head, k);
    assert_eq!(res, answer);
}
