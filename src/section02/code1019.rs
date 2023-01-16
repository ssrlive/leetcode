#![allow(dead_code)]

// 1019. Next Greater Node In Linked List
// https://leetcode.com/problems/next-greater-node-in-linked-list/
// https://leetcode.cn/problems/next-greater-node-in-linked-list/
//
// You are given the head of a linked list with n nodes.
//
// For each node in the list, find the value of the next greater node. That is, for each node, find the value of the first node that is next to it and has a strictly larger value than it.
//
// Return an integer array answer where answer[i] is the value of the next greater node of the ith node (1-indexed). If the ith node does not have a next greater node, set answer[i] = 0.
//
// Example 1:
//
// Input: head = [2,1,5]
// Output: [5,5,0]
//
// Example 2:
//
// Input: head = [2,7,4,3,5]
// Output: [7,0,5,5,0]
//
// Constraints:
//
// - The number of nodes in the list is n.
// - 1 <= n <= 10^4
// - 1 <= Node.val <= 10^9
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack: Vec<[i32; 2]> = vec![];
        let mut counter = 0;
        let mut cur = &head;
        while cur.is_some() {
            let val = cur.as_ref().unwrap().val;
            while !stack.is_empty() && (stack.last().unwrap()[1] < val) {
                res[stack.last().unwrap()[0] as usize] = val;
                stack.pop();
            }
            cur = &cur.as_ref().unwrap().next;
            stack.push([counter, val]);
            res.push(0);
            counter += 1;
        }
        res
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[2, 1, 5]);
    assert_eq!(Solution::next_larger_nodes(head), vec![5, 5, 0]);

    let head = ListNode::from_vec(&[2, 7, 4, 3, 5]);
    assert_eq!(Solution::next_larger_nodes(head), vec![7, 0, 5, 5, 0]);
}
