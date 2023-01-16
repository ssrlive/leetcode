#![allow(dead_code)]

// 1171. Remove Zero Sum Consecutive Nodes from Linked List
// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
// https://leetcode.cn/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
//
// Given the head of a linked list, we repeatedly delete consecutive sequences of nodes
// that sum to 0 until there are no such sequences.
//
// After doing so, return the head of the final linked list.  You may return any such answer.
//
// (Note that in the examples below, all sequences are serializations of ListNode objects.)
//
// Example 1:
//
// Input: head = [1,2,-3,3,1]
// Output: [3,1]
// Note: The answer [1,2,1] would also be accepted.
//
// Example 2:
//
// Input: head = [1,2,3,-3,4]
// Output: [1,2,4]
//
// Example 3:
//
// Input: head = [1,2,3,-3,-2]
// Output: [1]
//
// Constraints:
//
// - The given linked list will contain between 1 and 1000 nodes.
// - Each node in the linked list has -1000 <= node.val <= 1000.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::collections::HashMap;
        let mut head = head;
        let mut data = vec![];
        let (mut index, mut sum) = (0, 0);
        let mut from_index: HashMap<i32, i32> = HashMap::new();
        let mut to_index = HashMap::new();

        to_index.insert(0, -1);
        while let Some(mut node) = head {
            head = node.next.take();
            sum += node.val;
            node.next = None;
            data.push(node);

            if let Some(i) = to_index.get(&sum) {
                data.pop();
                let (start, end) = ((*i + 1) as usize, index as usize);
                for k in start..end {
                    if let Some(a) = from_index.get(&(k as i32)) {
                        to_index.remove(a);
                        from_index.remove(&(k as i32));
                        data.pop();
                    }
                }
                continue;
            }

            to_index.insert(sum, index);
            from_index.insert(index, sum);
            index += 1;
        }

        let mut ret = None;
        let mut tail = &mut ret;
        for p in data {
            tail = &mut tail.insert(p).next;
        }

        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, -3, 3, 1], vec![3, 1]),
        (vec![1, 2, 3, -3, 4], vec![1, 2, 4]),
        (vec![1, 2, 3, -3, -2], vec![1]),
    ];
    for (head, expected) in cases {
        assert_eq!(
            Solution::remove_zero_sum_sublists(ListNode::from_vec(&head)),
            ListNode::from_vec(&expected)
        );
    }
}
