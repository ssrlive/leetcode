#![allow(dead_code)]

// 435. Non-overlapping Intervals
// https://leetcode.com/problems/non-overlapping-intervals/
//
// Given a collection of intervals, find the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
//
// Example 1:
//
// Input: [[1,2],[2,3],[3,4],[1,3]]
// Output: 1
// Explanation: [1,3] can be removed and the rest of intervals are non-overlapping.
//
// Example 2:
//
// Input: [[1,2],[1,2],[1,2]]
// Output: 2
// Explanation: You need to remove two [1,2] to make the rest of intervals non-overlapping.
//
// Example 3:
//
// Input: [[1,2],[2,3]]
// Output: 0
// Explanation: You don't need to remove any of the intervals since they're already non-overlapping.
//
// Note:
//
// - You may assume the interval's end point is always bigger than its start point.
// - Intervals like [1,2] and [2,3] have borders "touching" but they don't overlap each other.
//
// Solution: https://leetcode.com/problems/non-overlapping-intervals/solution/
//
// Complexity: O(n log n)
// Space: O(1)
//
// Note: This is a greedy algorithm. The idea is to sort the intervals by their end point. Then, we take the first interval and compare its end with the next intervals start. As long as they overlap, we remove the next interval and continue comparing the end with the rest. We do this until we find a non overlapping interval. Then, we can add the current interval to our set of non overlapping intervals.

struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut count = 0;
        let mut end = std::i32::MIN;
        for interval in intervals {
            if interval[0] >= end {
                end = interval[1];
            } else {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test_erase_overlap_intervals() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
        2
    );
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]), 0);
}
