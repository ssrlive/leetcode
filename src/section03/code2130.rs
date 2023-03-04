#![allow(dead_code)]

/*

// 2130. Maximum Twin Sum of a Linked List
// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/
// https://leetcode.cn/problems/maximum-twin-sum-of-a-linked-list/
//
// Medium
//
// In a linked list of size n, where n is even, the ith node (0-indexed) of the linked list is known as the twin of the (n-1-i)th node, if 0 <= i <= (n / 2) - 1.

    For example, if n = 4, then node 0 is the twin of node 3, and node 1 is the twin of node 2. These are the only nodes with twins for n = 4.

The twin sum is defined as the sum of a node and its twin.

Given the head of a linked list with even length, return the maximum twin sum of the linked list.

Example 1:

Input: head = [5,4,2,1]
Output: 6
Explanation:
Nodes 0 and 1 are the twins of nodes 3 and 2, respectively. All have twin sum = 6.
There are no other nodes with twins in the linked list.
Thus, the maximum twin sum of the linked list is 6.

Example 2:

Input: head = [4,2,2,3]
Output: 7
Explanation:
The nodes with twins present in this linked list are:
- Node 0 is the twin of node 3 having a twin sum of 4 + 3 = 7.
- Node 1 is the twin of node 2 having a twin sum of 2 + 2 = 4.
Thus, the maximum twin sum of the linked list is max(7, 4) = 7.

Example 3:

Input: head = [1,100000]
Output: 100001
Explanation:
There is only one node with a twin in the linked list having twin sum of 1 + 100000 = 100001.

Constraints:

    The number of nodes in the list is an even integer in the range [2, 10^5].
    1 <= Node.val <= 10^5
*/

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        use std::collections::VecDeque;
        let mut n: usize = 0;

        let mut curr = &head;
        while curr.is_some() {
            n += 1;
            curr = &curr.as_ref().unwrap().next;
        }

        let h_n = n >> 1;
        let mut stk = VecDeque::<i32>::with_capacity(h_n);

        let mut i = 0;
        curr = &head;
        while curr.is_some() {
            stk.push_back(curr.as_ref().unwrap().val);
            curr = &curr.as_ref().unwrap().next;
            i += 1;
            if i == h_n {
                break;
            }
        }

        let mut ans = 0;
        while curr.is_some() {
            ans = ans.max(curr.as_ref().unwrap().val + stk.back().unwrap());
            stk.pop_back();
            curr = &curr.as_ref().unwrap().next;
        }

        ans
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[5, 4, 2, 1]);
    assert_eq!(Solution::pair_sum(head), 6);

    let head = ListNode::from_vec(&[4, 2, 2, 3]);
    assert_eq!(Solution::pair_sum(head), 7);

    let head = ListNode::from_vec(&[1, 100000]);
    assert_eq!(Solution::pair_sum(head), 100001);
}
