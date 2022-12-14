#![allow(dead_code)]

// 295. Find Median from Data Stream
// https://leetcode.com/problems/find-median-from-data-stream/
// https://leetcode.cn/problems/find-median-from-data-stream/
//
// The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value and the median is the mean of the two middle values.
//
// - For example, for arr = [2,3,4], the median is 3.
// - For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
//
// Implement the MedianFinder class:
//
// - MedianFinder() initializes the MedianFinder object.
// - void addNum(int num) adds the integer num from the data stream to the data structure.
// - double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.
//
// Example 1:
//
// Input
// ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
// [[], [1], [2], [], [3], []]
// Output
// [null, null, null, 1.5, null, 2.0]
//
// Explanation
// MedianFinder medianFinder = new MedianFinder();
// medianFinder.addNum(1);    // arr = [1]
// medianFinder.addNum(2);    // arr = [1, 2]
// medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
// medianFinder.addNum(3);    // arr[1, 2, 3]
// medianFinder.findMedian(); // return 2.0
//
// Constraints:
//
// - -10^5 <= num <= 10^5
// - There will be at least one element in the data structure before calling findMedian.
// - At most 5 * 10^4 calls will be made to addNum and findMedian.
//
// Follow up:
//
// If all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
// If 99% of all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
//

// use std::cmp::Reverse;

struct MedianFinder {
    small: std::collections::BinaryHeap<i32>,
    large: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            small: std::collections::BinaryHeap::new(),
            large: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self._add_num(num).unwrap_or_default();
    }
    fn _add_num(&mut self, num: i32) -> Option<()> {
        if self.small.len() >= self.large.len() {
            self.small.push(num);
            self.large.push(std::cmp::Reverse(self.small.pop()?));
        } else {
            self.large.push(std::cmp::Reverse(num));
            self.small.push(self.large.pop()?.0);
        }
        Some(())
    }

    pub fn find_median(&self) -> f64 {
        self._find_median().unwrap_or_default()
    }
    fn _find_median(&self) -> Option<f64> {
        match self.small.len().cmp(&self.large.len()) {
            std::cmp::Ordering::Equal => Some((self.small.peek()? + self.large.peek()?.0) as f64 / 2.0),
            std::cmp::Ordering::Greater => Some(*self.small.peek()? as f64),
            std::cmp::Ordering::Less => Some(self.large.peek()?.0 as f64),
        }
    }
}

#[test]
fn test_median_finder() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    median_finder.add_num(2);
    assert_eq!(median_finder.find_median(), 1.5);
    median_finder.add_num(3);
    assert_eq!(median_finder.find_median(), 2.0);
}
