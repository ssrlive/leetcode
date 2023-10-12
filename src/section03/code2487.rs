#![allow(dead_code)]

// 2487. Remove Nodes From Linked List
// https://leetcode.com/problems/remove-nodes-from-linked-list/
// https://leetcode.cn/problems/remove-nodes-from-linked-list/
//
// You are given the head of a linked list.
//
// Remove every node which has a node with a strictly greater value anywhere to the right side of it.
//
// Return the head of the modified linked list.
//
// Example 1:
//
// Input: head = [5,2,13,3,8]
// Output: [13,8]
// Explanation: The nodes that should be removed are 5, 2 and 3.
// - Node 13 is to the right of node 5.
// - Node 13 is to the right of node 2.
// - Node 8 is to the right of node 3.
//
// Example 2:
//
// Input: head = [1,1,1,1]
// Output: [1,1,1,1]
// Explanation: Every node has value 1, so no nodes are removed.
//
// Constraints:
//
// - The number of the nodes in the given list is in the range [1, 10^5].
// - 1 <= Node.val <= 10^5
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn remove_nodes_from_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack: Vec<Box<ListNode>> = Vec::new();
        let mut node: Option<Box<ListNode>> = head;
        while let Some(mut curr) = node {
            while let Some(top) = stack.last() {
                if top.val < curr.val {
                    stack.pop();
                } else {
                    break;
                }
            }
            node = curr.next.take();
            stack.push(curr);
        }
        while let Some(mut curr) = stack.pop() {
            curr.next = node;
            node = Some(curr);
        }
        node
    }
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let head = ListNode::from_vec(&[5, 2, 13, 3, 8]);
    let head = Solution::remove_nodes_from_linked_list(head);
    assert_eq!(head.as_ref().ok_or("")?.to_vec(), vec![13, 8]);

    let head = ListNode::from_vec(&[1, 1, 1, 1]);
    let head = Solution::remove_nodes_from_linked_list(head);
    assert_eq!(head.as_ref().ok_or("")?.to_vec(), vec![1, 1, 1, 1]);

    Ok(())
}
