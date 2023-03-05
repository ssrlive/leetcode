#![allow(dead_code)]

/*

// 2181. Merge Nodes in Between Zeros
// https://leetcode.com/problems/merge-nodes-in-between-zeros/
// https://leetcode.cn/problems/merge-nodes-in-between-zeros/
//
// Medium
//
// You are given the head of a linked list, which contains a series of integers separated by 0's. The beginning and end of the linked list will have Node.val == 0.

For every two consecutive 0's, merge all the nodes lying in between them into a single node whose value is the sum of all the merged nodes. The modified list should not contain any 0's.

Return the head of the modified linked list.

Example 1:

Input: head = [0,3,1,0,4,5,2,0]
Output: [4,11]
Explanation:
The above figure represents the given linked list. The modified list contains
- The sum of the nodes marked in green: 3 + 1 = 4.
- The sum of the nodes marked in red: 4 + 5 + 2 = 11.

Example 2:

Input: head = [0,1,0,3,0,2,2,0]
Output: [1,3,4]
Explanation:
The above figure represents the given linked list. The modified list contains
- The sum of the nodes marked in green: 1 = 1.
- The sum of the nodes marked in red: 3 = 3.
- The sum of the nodes marked in yellow: 2 + 2 = 4.

Constraints:

    The number of nodes in the list is in the range [3, 2 * 10^5].
    0 <= Node.val <= 1000
    There are no two consecutive nodes with Node.val == 0.
    The beginning and end of the linked list have Node.val == 0.
*/

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tot = 0;
        let mut tail = &mut head.unwrap();
        let mut tail2 = &mut dummy;
        while let Some(ref mut node) = tail.next {
            tot += node.val;
            if node.val == 0 {
                tail2.next = Some(Box::new(ListNode::new(tot)));
                if let Some(ref mut post) = tail2.next {
                    tail2 = post;
                }
                tot = 0;
            }
            tail = node;
        }
        dummy.next
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[0, 3, 1, 0, 4, 5, 2, 0]);
    let res = ListNode::from_vec(&[4, 11]);
    assert_eq!(Solution::merge_nodes(head), res);

    let head = ListNode::from_vec(&[0, 1, 0, 3, 0, 2, 2, 0]);
    let res = ListNode::from_vec(&[1, 3, 4]);
    assert_eq!(Solution::merge_nodes(head), res);
}
