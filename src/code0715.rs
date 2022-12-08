#![allow(dead_code)]

// 715. Range Module
// https://leetcode.com/problems/range-module/
//
// A Range Module is a module that tracks ranges of numbers. Design a data structure to track the
// ranges represented as half-open intervals and query about them.
//
// A half-open interval [left, right) denotes all the real numbers x where left <= x < right.
//
// Implement the RangeModule class:
//
// RangeModule() Initializes the object of the data structure.
// void addRange(int left, int right) Adds the half-open interval [left, right), tracking every real number in that interval.
//   Adding an interval that partially overlaps with currently tracked numbers should add any numbers in the interval
//   [left, right) that are not already tracked.
// boolean queryRange(int left, int right) Returns true if every real number in the interval [left, right)
//   is currently being tracked, and false otherwise.
// void removeRange(int left, int right) Stops tracking every real number currently being tracked in the half-open interval [left, right).
//
// Example 1:
//
// Input
// ["RangeModule", "addRange", "removeRange", "queryRange", "queryRange", "queryRange"]
// [[], [10, 20], [14, 16], [10, 14], [13, 15], [16, 17]]
// Output
// [null, null, null, true, false, true]
//
// Explanation
// RangeModule rangeModule = new RangeModule();
// rangeModule.addRange(10, 20);
// rangeModule.removeRange(14, 16);
// rangeModule.queryRange(10, 14); // return True,(Every number in [10, 14) is being tracked)
// rangeModule.queryRange(13, 15); // return False,(Numbers like 14, 14.03, 14.17 in [13, 15) are not being tracked)
// rangeModule.queryRange(16, 17); // return True, (The number 16 in [16, 17) is still being tracked, despite the remove operation)
//
// Constraints:
//
// - 1 <= left < right <= 10^9
// - At most 10^4 calls will be made to addRange, queryRange, and removeRange.
//

#[derive(Debug)]
struct RangeModule {
    ranges: std::collections::VecDeque<(i32, i32)>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: std::collections::VecDeque::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        if self._add_range(left, right).is_none() {
            println!("add_range({}, {}) failed", left, right);
        }
    }
    fn _add_range(&mut self, left: i32, right: i32) -> Option<()> {
        let mut left = left;
        let mut right = right;
        let mut i = 0;
        while i < self.ranges.len() {
            let (l, r) = self.ranges.get(i)?;
            let (l, r) = (*l, *r);
            if right < l {
                break;
            } else if left > r {
                i += 1;
            } else {
                left = left.min(l);
                right = right.max(r);
                self.ranges.remove(i);
            }
        }
        self.ranges.insert(i, (left, right));
        Some(())
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self._query_range(left, right).unwrap_or_default()
    }
    fn _query_range(&self, left: i32, right: i32) -> Option<bool> {
        let mut i = 0;
        let mut j = self.ranges.len() as i32 - 1;
        while i <= j {
            let mid = (i + j) / 2;
            let (l, r) = self.ranges.get(mid as usize)?;
            let (l, r) = (*l, *r);
            if right < l {
                j = mid - 1;
            } else if left > r {
                i = mid + 1;
            } else {
                return Some(left >= l && right <= r);
            }
        }
        Some(false)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        if self._remove_range(left, right).is_none() {
            println!("remove_range({}, {}) failed", left, right);
        }
    }
    fn _remove_range(&mut self, left: i32, right: i32) -> Option<()> {
        let mut i = 0;
        while i < self.ranges.len() {
            let (l, r) = self.ranges.get(i)?;
            let (l, r) = (*l, *r);
            if right < l {
                break;
            } else if left > r {
                i += 1;
            } else {
                if left > l {
                    self.ranges.insert(i, (l, left));
                    i += 1;
                }
                if right < r {
                    self.ranges.insert(i, (right, r));
                    i += 1;
                }
                self.ranges.remove(i);
            }
        }
        Some(())
    }
}

#[test]
fn test() {
    let mut range_module = RangeModule::new();
    range_module.add_range(10, 20);
    range_module.remove_range(14, 16);
    assert_eq!(range_module.query_range(10, 14), true);
    assert_eq!(range_module.query_range(13, 15), false);
    assert_eq!(range_module.query_range(16, 17), true);

    let mut range_module = RangeModule::new();
    range_module.add_range(10, 180);
    range_module.add_range(150, 200);
    range_module.add_range(250, 500);
    assert_eq!(range_module.query_range(50, 100), true);
    assert_eq!(range_module.query_range(180, 300), false);
    assert_eq!(range_module.query_range(600, 1000), false);
    range_module.remove_range(50, 150);
    assert_eq!(range_module.query_range(50, 100), false);
}
