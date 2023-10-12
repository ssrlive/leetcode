#![allow(dead_code)]

// 732. My Calendar III
// https://leetcode.com/problems/my-calendar-iii/
// https://leetcode.cn/problems/my-calendar-iii/
//
// A k-booking happens when k events have some non-empty intersection (i.e., there is some time that is common to all k events.)
//
// You are given some events [startTime, endTime), after each given event, return an integer k representing
// the maximum k-booking between all the previous events.
//
// Implement the MyCalendarThree class:
//
// - MyCalendarThree() Initializes the object.
// - int book(int startTime, int endTime) Returns an integer k representing the largest integer such that there exists a k-booking in the calendar.
//
// Example 1:
//
// Input
// ["MyCalendarThree", "book", "book", "book", "book", "book", "book"]
// [[], [10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]]
// Output
// [null, 1, 1, 2, 3, 3, 3]
//
// Explanation
// MyCalendarThree myCalendarThree = new MyCalendarThree();
// myCalendarThree.book(10, 20); // return 1
// myCalendarThree.book(50, 60); // return 1
// myCalendarThree.book(10, 40); // return 2
// myCalendarThree.book(5, 15); // return 3
// myCalendarThree.book(5, 10); // return 3
// myCalendarThree.book(25, 55); // return 3
//
// Constraints:
//
// - 0 <= startTime < endTime <= 10^9
// - At most 400 calls will be made to book.
//

use std::collections::BTreeMap;
use std::iter::{once, FromIterator};

struct MyCalendarThree {
    data: BTreeMap<i32, i32>,
    max: i32,
}

impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {
            data: BTreeMap::from_iter(once((0, 0))),
            max: 0i32,
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> i32 {
        self._book(start, end).unwrap_or_default()
    }
    fn _book(&mut self, start: i32, end: i32) -> Option<i32> {
        let r = &mut self.max;
        let mut cur = *self.data.range(..=start).next_back()?.1;

        self.data.entry(start).or_insert(cur);
        for (_, v) in self.data.range_mut(start..end) {
            cur = *v;
            *v += 1;
            *r = std::cmp::max(*r, *v);
        }
        self.data.entry(end).or_insert(cur);

        Some(*r)
    }
}

#[test]
fn test() {
    let mut calendar = MyCalendarThree::new();
    assert_eq!(calendar.book(10, 20), 1);
    assert_eq!(calendar.book(50, 60), 1);
    assert_eq!(calendar.book(10, 40), 2);
    assert_eq!(calendar.book(5, 15), 3);
    assert_eq!(calendar.book(5, 10), 3);
    assert_eq!(calendar.book(25, 55), 3);
}
