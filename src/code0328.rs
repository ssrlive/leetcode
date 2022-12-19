#![allow(dead_code)]

// 328. Odd Even Linked List
// https://leetcode.com/problems/odd-even-linked-list/
// https://leetcode.cn/problems/odd-even-linked-list/
//
// Given the head of a singly linked list, group all the nodes with odd indices together
// followed by the nodes with even indices, and return the reordered list.
//
// The first node is considered odd, and the second node is even, and so on.
//
// Note that the relative order inside both the even and odd groups should remain as it was in the input.
//
// You must solve the problem in O(1) extra space complexity and O(n) time complexity.
//
// Example 1:
//
// Input: head = [1,2,3,4,5]
// Output: [1,3,5,2,4]
// Example 2:
//
// Input: head = [2,1,3,5,6,4,7]
// Output: [2,3,6,7,1,5,4]
//
// Constraints:
//
// - The number of nodes in the linked list is in the range [0, 10^4].
// - -10^6 <= Node.val <= 10^6
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let (mut ret, mut part_two) = (None, None);
        let (mut tail1, mut tail2) = (&mut ret, &mut part_two);
        let mut flag = 0i32;

        while let Some(mut node) = head {
            head = node.next.take();
            flag = 1 - flag;
            if flag == 1 {
                tail1 = &mut tail1.insert(node).next;
            } else {
                tail2 = &mut tail2.insert(node).next;
            }
        }

        if let Some(node) = part_two {
            let _ = tail1.insert(node);
        }

        ret
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
    let ret = ListNode::from_vec(&[1, 3, 5, 2, 4]);
    assert_eq!(Solution::odd_even_list(head), ret);

    let head = ListNode::from_vec(&[2, 1, 3, 5, 6, 4, 7]);
    let ret = ListNode::from_vec(&[2, 3, 6, 7, 1, 5, 4]);
    assert_eq!(Solution::odd_even_list(head), ret);
}
