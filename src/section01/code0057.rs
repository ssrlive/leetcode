#![allow(dead_code)]

// 57. Insert Interval
// https://leetcode.com/problems/insert-interval/description/
// https://leetcode.cn/problems/insert-interval/description/
//
// You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent
// the start and the end of the ith interval and intervals is sorted in ascending order by starti.
// You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
//
// Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals
// still does not have any overlapping intervals (merge overlapping intervals if necessary).
//
// Return intervals after the insertion.
//
// Example 1:
//
// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
// Output: [[1,5],[6,9]]
//
// Example 2:
//
// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
// Output: [[1,2],[3,10],[12,16]]
// Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//
// Constraints:
//
// - 0 <= intervals.length <= 10^4
// - intervals[i].length == 2
// - 0 <= starti <= endi <= 10^5
// - intervals is sorted by starti in ascending order.
// - newInterval.length == 2
// - 0 <= start <= end <= 10^5
//

struct Solution;

impl Solution {
    fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut s, mut e) = (new_interval[0], new_interval[1]);
        let pred = |x| {
            move |i: &Vec<i32>| {
                if i[0] > x {
                    std::cmp::Ordering::Greater
                } else if i[1] < x {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        };
        let li = intervals.binary_search_by(pred(s)).unwrap_or_else(|x| x);
        let ri = intervals.binary_search_by(pred(e)).unwrap_or_else(|x| x);
        let l = &intervals[..(li + usize::from(li < intervals.len() && intervals[li][1] < s))];
        let r = &intervals[(ri + usize::from(ri >= intervals.len() || intervals[ri][0] <= e)).min(intervals.len())..];
        if l.len() + r.len() != intervals.len() {
            s = s.min(intervals[l.len()][0]);
            e = e.max(intervals[intervals.len() - r.len() - 1][1]);
        }
        vec![l, &[[s, e].to_vec()], r].concat()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 3], vec![6, 9]], vec![2, 5], vec![vec![1, 5], vec![6, 9]]),
        (
            vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]],
            vec![4, 8],
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
        ),
        (vec![vec![1, 5]], vec![2, 3], vec![vec![1, 5]]),
        (vec![vec![1, 5]], vec![2, 7], vec![vec![1, 7]]),
    ];
    for (intervals, new_interval, expected) in cases {
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
}
