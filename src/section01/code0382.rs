#![allow(dead_code)]

// 382. Linked List Random Node
// https://leetcode.com/problems/linked-list-random-node/
// https://leetcode.cn/problems/linked-list-random-node/
//
// Given a singly linked list, return a random node's value from the linked list. Each node must have the same probability of being chosen.
//
// Implement the Solution class:
//
// Solution(ListNode head) Initializes the object with the head of the singly-linked list head.
// int getRandom() Chooses a node randomly from the list and returns its value. All the nodes of the list should be equally likely to be chosen.
//
// Example 1:
//
// Input
// ["Solution", "getRandom", "getRandom", "getRandom", "getRandom", "getRandom"]
// [[[1, 2, 3]], [], [], [], [], []]
// Output
// [null, 1, 3, 2, 2, 3]
//
// Explanation
// Solution solution = new Solution([1, 2, 3]);
// solution.getRandom(); // return 1
// solution.getRandom(); // return 3
// solution.getRandom(); // return 2
// solution.getRandom(); // return 2
// solution.getRandom(); // return 3
// // getRandom() should return either 1, 2, or 3 randomly. Each element should have equal probability of returning.
//
// Constraints:
//
// - The number of nodes in the linked list will be in the range [1, 10^4].
// - -10^4 <= Node.val <= 10^4
// - At most 10^4 calls will be made to getRandom.
//
// Follow up:
//
// - What if the linked list is extremely large and its length is unknown to you?
// - Could you solve this efficiently without using extra space?
//

use crate::listnode::ListNode;

struct Solution {
    head: Option<Box<ListNode>>,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    fn get_random(&self) -> i32 {
        let mut head = &self.head;
        let mut count = 0;
        let mut result = 0;
        while let Some(node) = head {
            count += 1;
            if rand::random::<f64>() < 1.0 / count as f64 {
                result = node.val;
            }
            head = &node.next;
        }
        result
    }
}

#[test]
fn test() {
    let head = ListNode::from_vec(&[1, 2, 3]);
    let solution = Solution::new(head);
    let mut count = vec![0; 3];
    for _ in 0..1000 {
        let val = solution.get_random();
        count[val as usize - 1] += 1;
    }
    // assert_eq!(count, vec![333, 333, 334]);
}
