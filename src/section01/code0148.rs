#![allow(dead_code)]

// 148. Sort List
// https://leetcode.com/problems/sort-list/
// https://leetcode.cn/problems/sort-list/
//
// Given the head of a linked list, return the list after sorting it in ascending order.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
fn test1() -> Result<(), Box<dyn std::error::Error>> {
    let head = ListNode::from_vec(&vec![4, 2, 1, 3]);
    let head = Solution::sort_list(head);
    assert_eq!(head.ok_or("")?.to_vec(), vec![1, 2, 3, 4]);

    let head = ListNode::from_vec(&vec![-1, 5, 3, 4, 0]);
    let head = Solution::sort_list(head);
    assert_eq!(head.ok_or("")?.to_vec(), vec![-1, 0, 3, 4, 5]);
    Ok(())
}
