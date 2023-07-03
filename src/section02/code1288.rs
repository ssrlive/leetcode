#![allow(dead_code)]

// 1288. Remove Covered Intervals
// https://leetcode.com/problems/remove-covered-intervals/
// https://leetcode.cn/problems/remove-covered-intervals/
//
// Medium
//
// Given an array intervals where intervals[i] = [li, ri] represent the interval [li, ri),
// remove all intervals that are covered by another interval in the list.
//
// The interval [a, b) is covered by the interval [c, d) if and only if c <= a and b <= d.
//
// Return the number of remaining intervals.
//
// Example 1:
//
// Input: intervals = [[1,4],[3,6],[2,8]]
// Output: 2
// Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.
//
// Example 2:
//
// Input: intervals = [[1,4],[2,3]]
// Output: 1
//
// Constraints:
//
// -    1 <= intervals.length <= 1000
// -    intervals[i].length == 2
// -    0 <= li < ri <= 10^5
// -    All the given intervals are unique.
//

struct Solution;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| if a[0] == b[0] { b[1].cmp(&a[1]) } else { a[0].cmp(&b[0]) });
        let mut count = 0;
        let mut right = 0;
        for interval in intervals {
            if interval[1] > right {
                count += 1;
                right = interval[1];
            }
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![(vec![vec![1, 4], vec![3, 6], vec![2, 8]], 2), (vec![vec![1, 4], vec![2, 3]], 1)];
    for (intervals, expected) in cases {
        assert_eq!(Solution::remove_covered_intervals(intervals), expected);
    }
}
