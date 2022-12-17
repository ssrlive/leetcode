#![allow(dead_code)]

// 817. Linked List Components
// https://leetcode.com/problems/linked-list-components/
//
// You are given the head of a linked list containing unique integer values and an integer array nums that is a subset of the linked list values.
//
// Return the number of connected components in nums where two values are connected if they appear consecutively in the linked list.
//
// Example 1:
//
// Input: head = [0,1,2,3], nums = [0,1,3]
// Output: 2
// Explanation: 0 and 1 are connected, so [0, 1] and [3] are the two connected components.
//
// Example 2:
//
// Input: head = [0,1,2,3,4], nums = [0,3,1,4]
// Output: 2
// Explanation: 0 and 1 are connected, 3 and 4 are connected, so [0, 1] and [3, 4] are the two connected components.
//
// Constraints:
//
// - The number of nodes in the linked list is n.
// - 1 <= n <= 10^4
// - 0 <= Node.val < n
// - All the values Node.val are unique.
// - 1 <= nums.length <= n
// - 0 <= nums[i] < n
// - All the values of nums are unique.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut count = 0;
        let mut current = &head;
        while let Some(node) = current {
            if nums.binary_search(&node.val).is_ok() {
                count += 1;
                while let Some(node) = current {
                    if nums.binary_search(&node.val).is_err() {
                        break;
                    }
                    current = &node.next;
                }
            } else {
                current = &node.next;
            }
        }
        count
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[0, 1, 2, 3]);
    let nums = vec![0, 1, 3];
    assert_eq!(Solution::num_components(head, nums), 2);

    let head = ListNode::from_vec(&[0, 1, 2, 3, 4]);
    let nums = vec![0, 3, 1, 4];
    assert_eq!(Solution::num_components(head, nums), 2);
}
