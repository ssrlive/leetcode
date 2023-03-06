#![allow(dead_code)]

/*

// 2276. Count Integers in Intervals
// https://leetcode.com/problems/count-integers-in-intervals/
// https://leetcode.cn/problems/count-integers-in-intervals/
//
// Hard
//
// Given an empty set of intervals, implement a data structure that can:

    Add an interval to the set of intervals.
    Count the number of integers that are present in at least one interval.

Implement the CountIntervals class:

    CountIntervals() Initializes the object with an empty set of intervals.
    void add(int left, int right) Adds the interval [left, right] to the set of intervals.
    int count() Returns the number of integers that are present in at least one interval.

Note that an interval [left, right] denotes all the integers x where left <= x <= right.

Example 1:

Input
["CountIntervals", "add", "add", "count", "add", "count"]
[[], [2, 3], [7, 10], [], [5, 8], []]
Output
[null, null, null, 6, null, 8]

Explanation
CountIntervals countIntervals = new CountIntervals(); // initialize the object with an empty set of intervals.
countIntervals.add(2, 3);  // add [2, 3] to the set of intervals.
countIntervals.add(7, 10); // add [7, 10] to the set of intervals.
countIntervals.count();    // return 6
                           // the integers 2 and 3 are present in the interval [2, 3].
                           // the integers 7, 8, 9, and 10 are present in the interval [7, 10].
countIntervals.add(5, 8);  // add [5, 8] to the set of intervals.
countIntervals.count();    // return 8
                           // the integers 2 and 3 are present in the interval [2, 3].
                           // the integers 5 and 6 are present in the interval [5, 8].
                           // the integers 7 and 8 are present in the intervals [5, 8] and [7, 10].
                           // the integers 9 and 10 are present in the interval [7, 10].

Constraints:

    1 <= left <= right <= 10^9
    At most 105 calls in total will be made to add and count.
    At least one call will be made to count.
*/

use std::collections::BTreeMap;
struct CountIntervals {
    cnt: i32,
    mp: BTreeMap<i32, i32>,
}

impl CountIntervals {
    fn new() -> Self {
        let mp: BTreeMap<i32, i32> = BTreeMap::new();
        Self { cnt: 0, mp }
    }

    fn add(&mut self, left: i32, right: i32) {
        let (mut left, mut right) = (left, right);
        self.cnt += right - left + 1;
        if self.mp.is_empty() {
            self.mp.insert(left, right);
            return;
        }
        let (a, _) = self.mp.iter().next().unwrap();
        let (_, b) = self.mp.iter().rev().next().unwrap();
        if a - 1 > right || b + 1 < left {
            self.mp.insert(left, right);
            return;
        }
        if let Some((&key, &value)) = self.mp.range(..left + 1).rev().next() {
            if value + 1 >= left {
                if value > right {
                    self.cnt -= right - left + 1;
                } else {
                    self.cnt -= value + 1 - left;
                }
                left = key;
                right = right.max(value);
            }
        }
        let mut done = false;
        while !done {
            done = true;
            if let Some((&key, &value)) = self.mp.range(left + 1..).next() {
                if value > right {
                    break;
                }
                self.cnt -= value - key + 1;
                self.mp.remove(&key);
                done = false;
            }
        }
        if let Some((&key, &value)) = self.mp.range(left + 1..).next() {
            if right + 1 >= key {
                self.cnt -= right + 1 - key;
                right = value;
                self.mp.remove(&key);
            }
        }

        self.mp.insert(left, right);
    }

    fn count(&self) -> i32 {
        self.cnt
    }
}

#[test]
fn test() {
    let mut obj = CountIntervals::new();
    obj.add(2, 3);
    obj.add(7, 10);
    assert_eq!(obj.count(), 6);
    obj.add(5, 8);
    assert_eq!(obj.count(), 8);
}
