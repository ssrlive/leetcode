#![allow(dead_code)]

// 2807. Insert Greatest Common Divisors in Linked List
// https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/
// https://leetcode.cn/problems/insert-greatest-common-divisors-in-linked-list/
//
// Medium
//
// Given the head of a linked list head, in which each node contains an integer value.
//
// Between every pair of adjacent nodes, insert a new node with a value equal to the greatest common divisor of them.
//
// Return the linked list after insertion.
//
// The greatest common divisor of two numbers is the largest positive integer that evenly divides both numbers.
//
// Example 1:
//
// Input: head = [18,6,10,3]
// Output: [18,6,6,2,10,1,3]
// Explanation: The 1st diagram denotes the initial linked list and the 2nd diagram denotes the linked list
// after inserting the new nodes (nodes in blue are the inserted nodes).
// - We insert the greatest common divisor of 18 and 6 = 6 between the 1st and the 2nd nodes.
// - We insert the greatest common divisor of 6 and 10 = 2 between the 2nd and the 3rd nodes.
// - We insert the greatest common divisor of 10 and 3 = 1 between the 3rd and the 4th nodes.
// There are no more adjacent nodes, so we return the linked list.
//
// Example 2:
//
// Input: head = [7]
// Output: [7]
// Explanation: The 1st diagram denotes the initial linked list and the 2nd diagram denotes the linked list after inserting the new nodes.
// There are no pairs of adjacent nodes, so we return the initial linked list.
//
// Constraints:
//
// - The number of nodes in the list is in the range [1, 5000].
// - 1 <= Node.val <= 1000
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut data = vec![];

        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            data.push(node);
        }
        data.reverse();

        let mut ret = None;
        let mut tail = &mut ret;
        let mut last = -1;

        while let Some(node) = data.pop() {
            if last != -1 {
                let d = Self::gcd(node.val, last);
                tail = &mut tail.insert(Box::new(ListNode::new(d))).next;
            }
            last = node.val;
            tail = &mut tail.insert(node).next;
        }

        ret
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a < b {
            return Self::gcd(b, a);
        }
        if a % b == 0 {
            return b;
        }
        Self::gcd(b, a % b)
    }
}

#[test]
fn test() {
    use crate::listnode::ListNode;

    let test_cases = vec![(vec![18, 6, 10, 3], vec![18, 6, 6, 2, 10, 1, 3]), (vec![7], vec![7])];

    for (nums, expect) in test_cases {
        let l1 = ListNode::from_vec(&nums);
        let l2 = ListNode::from_vec(&expect);
        assert_eq!(Solution::insert_greatest_common_divisors(l1), l2, "nums: {:?}", nums);
    }
}
