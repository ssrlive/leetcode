#![allow(dead_code)]

/*

// 2336. Smallest Number in Infinite Set
// https://leetcode.com/problems/smallest-number-in-infinite-set/
// https://leetcode.cn/problems/smallest-number-in-infinite-set/
//
// Medium
//
// You have a set which contains all positive integers [1, 2, 3, 4, 5, ...].

Implement the SmallestInfiniteSet class:

    SmallestInfiniteSet() Initializes the SmallestInfiniteSet object to contain all positive integers.
    int popSmallest() Removes and returns the smallest integer contained in the infinite set.
    void addBack(int num) Adds a positive integer num back into the infinite set, if it is not already in the infinite set.

Example 1:

Input
["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
[[], [2], [], [], [], [1], [], [], []]
Output
[null, null, 1, 2, 3, null, 1, 4, 5]

Explanation
SmallestInfiniteSet smallestInfiniteSet = new SmallestInfiniteSet();
smallestInfiniteSet.addBack(2);    // 2 is already in the set, so no change is made.
smallestInfiniteSet.popSmallest(); // return 1, since 1 is the smallest number, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 2, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 3, and remove it from the set.
smallestInfiniteSet.addBack(1);    // 1 is added back to the set.
smallestInfiniteSet.popSmallest(); // return 1, since 1 was added back to the set and
                                   // is the smallest number, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 4, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 5, and remove it from the set.

Constraints:

    1 <= num <= 1000
    At most 1000 calls will be made in total to popSmallest and addBack.
*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
struct SmallestInfiniteSet {
    heap: BinaryHeap<Reverse<i32>>,
    seen: HashSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        const RANGE: i32 = 1e3 as i32 + 1;
        Self {
            heap: {
                let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(RANGE as usize);
                for num in 1..RANGE {
                    heap.push(Reverse(num));
                }
                heap
            },
            seen: {
                let mut seen: HashSet<i32> = HashSet::with_capacity(RANGE as usize);
                for num in 1..RANGE {
                    seen.insert(num);
                }
                seen
            },
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(top)) = self.heap.pop() {
            self.seen.remove(&top);
            return top;
        }
        -1
    }

    fn add_back(&mut self, num: i32) {
        if self.seen.insert(num) {
            self.heap.push(Reverse(num));
        }
    }
}

#[test]
fn test() {
    let mut smallest_infinite_set = SmallestInfiniteSet::new();
    smallest_infinite_set.add_back(2);
    assert_eq!(smallest_infinite_set.pop_smallest(), 1);
    assert_eq!(smallest_infinite_set.pop_smallest(), 2);
    assert_eq!(smallest_infinite_set.pop_smallest(), 3);
    smallest_infinite_set.add_back(1);
    assert_eq!(smallest_infinite_set.pop_smallest(), 1);
    assert_eq!(smallest_infinite_set.pop_smallest(), 4);
    assert_eq!(smallest_infinite_set.pop_smallest(), 5);
}
