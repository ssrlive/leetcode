#![allow(dead_code)]

// 703. Kth Largest Element in a Stream
// https://leetcode.com/problems/kth-largest-element-in-a-stream/
// https://leetcode.cn/problems/kth-largest-element-in-a-stream/
//
// Design a class to find the kth largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.
//
// Implement KthLargest class:
//
// KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
// int add(int val) Appends the integer val to the stream and returns the element representing the kth largest element in the stream.
//
// Example 1:
//
// Input
// ["KthLargest", "add", "add", "add", "add", "add"]
// [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
// Output
// [null, 4, 5, 5, 8, 8]
//
// Explanation
// KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
// kthLargest.add(3);   // return 4
// kthLargest.add(5);   // return 5
// kthLargest.add(10);  // return 5
// kthLargest.add(9);   // return 8
// kthLargest.add(4);   // return 8
//
// Constraints:
//
// - 1 <= k <= 10^4
// - 0 <= nums.length <= 10^4
// - -10^4 <= nums[i] <= 10^4
// - -10^4 <= val <= 10^4
// At most 10^4 calls will be made to add.
// It is guaranteed that there will be at least k elements in the array when you search for the k-th element.
//
// Follow up: Could you solve it without sorting the stream?

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
        }
        Self { k: k as usize, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self._add(val).unwrap_or(-1)
    }
    fn _add(&mut self, val: i32) -> Option<i32> {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k {
            self.heap.pop();
        }
        Some(self.heap.peek()?.0)
    }
}

#[test]
fn test() {
    let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(kth_largest.add(3), 4);
    assert_eq!(kth_largest.add(5), 5);
    assert_eq!(kth_largest.add(10), 5);
    assert_eq!(kth_largest.add(9), 8);
    assert_eq!(kth_largest.add(4), 8);
}
