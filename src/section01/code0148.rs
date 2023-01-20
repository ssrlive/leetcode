#![allow(dead_code)]

// 148. Sort List
// https://leetcode.com/problems/sort-list/
// https://leetcode.cn/problems/sort-list/
//
//
// Given the head of a linked list, return the list after sorting it in ascending order.
//
// Example 1:
//
// Input: head = [4,2,1,3]
// Output: [1,2,3,4]
//
// Example 2:
//
// Input: head = [-1,5,3,4,0]
// Output: [-1,0,3,4,5]
//
// Example 3:
//
// Input: head = []
// Output: []
//
// Constraints:
//
// The number of nodes in the list is in the range [0, 5 * 10^4].
// -10^5 <= Node.val <= 10^5
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut vec = vec![];
        while let mut node @ Some(_) = head {
            head = node.as_mut()?.next.take();
            vec.push(node);
        }
        vec.sort_by_key(|node| node.as_ref().unwrap().val);
        let mut next = None;
        while let Some(mut node) = vec.pop() {
            node.as_mut()?.next = next;
            next = node;
        }
        next
    }

    pub fn sort_list_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut vec: Vec<i32> = vec![];
        while let Some(mut node) = head {
            vec.push(node.val);
            head = node.next.take();
        }
        vec.sort();
        let mut next = None;
        while let Some(val) = vec.pop() {
            next = Some(Box::new(ListNode { next, val }))
        }
        next
    }
}

#[test]
fn test1() {
    let cases = vec![
        (vec![4, 2, 1, 3], vec![1, 2, 3, 4]),
        (vec![-1, 5, 3, 4, 0], vec![-1, 0, 3, 4, 5]),
        (vec![], vec![]),
    ];
    for (input, expected) in cases {
        let input = ListNode::from_vec(&input);
        let expected = ListNode::from_vec(&expected);
        let output = Solution::sort_list(input);
        assert_eq!(output, expected);
    }
}
