#![allow(dead_code)]

// 352. Data Stream as Disjoint Intervals
// https://leetcode.com/problems/data-stream-as-disjoint-intervals/
// https://leetcode.cn/problems/data-stream-as-disjoint-intervals/
//
// Given a data stream input of non-negative integers a1, a2, ..., an, summarize the numbers seen so far as a list of disjoint intervals.
//
// Implement the SummaryRanges class:
//
// SummaryRanges() Initializes the object with an empty stream.
// void addNum(int value) Adds the integer value to the stream.
// int[][] getIntervals() Returns a summary of the integers in the stream currently
// as a list of disjoint intervals [starti, endi]. The answer should be sorted by starti.
//
// Example 1:
//
// Input
// ["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
// [[], [1], [], [3], [], [7], [], [2], [], [6], []]
// Output
// [null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]
//
// Explanation
// SummaryRanges summaryRanges = new SummaryRanges();
// summaryRanges.addNum(1);      // arr = [1]
// summaryRanges.getIntervals(); // return [[1, 1]]
// summaryRanges.addNum(3);      // arr = [1, 3]
// summaryRanges.getIntervals(); // return [[1, 1], [3, 3]]
// summaryRanges.addNum(7);      // arr = [1, 3, 7]
// summaryRanges.getIntervals(); // return [[1, 1], [3, 3], [7, 7]]
// summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
// summaryRanges.getIntervals(); // return [[1, 3], [7, 7]]
// summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
// summaryRanges.getIntervals(); // return [[1, 3], [6, 7]]
//
// Constraints:
// - 0 <= value <= 10^4
// - At most 3 * 10^4 calls will be made to addNum and getIntervals.
//
// Follow up: What if there are lots of merges and the number of disjoint intervals is small compared to the size of the data stream?
//

struct SummaryRanges {
    s: std::collections::BTreeSet<i32>,
}
impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            s: std::collections::BTreeSet::new(),
        }
    }
    fn add_num(&mut self, value: i32) {
        self.s.insert(value);
    }
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        let mut curr = -1;
        let mut t = -1;
        for x in self.s.iter() {
            if curr == -1 {
                t = *x;
                curr = *x;
            } else if curr + 1 == *x {
                curr += 1;
            } else {
                v.push(vec![t, curr]);
                t = *x;
                curr = *x;
            }
        }
        v.push(vec![t, curr]);
        v
    }
}

#[test]
fn test_example() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1]]);
    summary_ranges.add_num(3);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
    summary_ranges.add_num(7);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1], vec![3, 3], vec![7, 7]]);
    summary_ranges.add_num(2);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
    summary_ranges.add_num(6);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
}
