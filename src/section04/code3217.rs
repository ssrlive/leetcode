#![allow(dead_code)]

// 3217. Delete Nodes From Linked List Present in Array
// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/
// https://leetcode.cn/problems/delete-nodes-from-linked-list-present-in-array/
//
// Medium
//
// You are given an array of integers nums and the head of a linked list. Return the head of the modified
// linked list after removing all nodes from the linked list that have a value that exists in nums.
//
// Example 1:
//
// Input: nums = [1,2,3], head = [1,2,3,4,5]
//
// Output: [4,5]
//
// Explanation:
//
// Remove the nodes with values 1, 2, and 3.
//
// Example 2:
//
// Input: nums = [1], head = [1,2,1,2,1,2]
//
// Output: [2,2,2]
//
// Explanation:
//
// Remove the nodes with value 1.
//
// Example 3:
//
// Input: nums = [5], head = [1,2,3,4]
//
// Output: [1,2,3,4]
//
// Explanation:
//
// No node has value 5.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^5
//     All elements in nums are unique.
//     The number of nodes in the given list is in the range [1, 10^5].
//     1 <= Node.val <= 10^5
//     The input is generated such that there is at least one node in the linked list that has a value not present in nums.
//

use crate::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        use std::collections::HashSet;
        let set: HashSet<i32> = HashSet::from_iter(nums);
        let mut dummy: ListNode = ListNode::new(0);
        let mut current: &mut ListNode = &mut dummy;
        while let Some(mut inner) = head {
            if set.contains(&inner.val) {
                head = inner.next;
            } else {
                head = inner.next.take();
                current.next = Some(inner);
                if current.next.is_none() {
                    break;
                }

                current = current.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
    let result = Solution::modified_list(nums, head).unwrap().to_vec();
    assert_eq!(result, vec![4, 5]);

    let nums = vec![1];
    let head = ListNode::from_vec(&[1, 2, 1, 2, 1, 2]);
    let result = Solution::modified_list(nums, head).unwrap().to_vec();
    assert_eq!(result, vec![2, 2, 2]);

    let nums = vec![5];
    let head = ListNode::from_vec(&[1, 2, 3, 4]);
    let result = Solution::modified_list(nums, head).unwrap().to_vec();
    assert_eq!(result, vec![1, 2, 3, 4]);
}
