#![allow(dead_code)]

// 56. Merge Intervals
// https://leetcode.com/problems/merge-intervals/
// https://leetcode.cn/problems/merge-intervals/
//
// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
//
// Example 1:
//
// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
//
// Example 2:
//
// Input: intervals = [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//
// Constraints:
//
// - 1 <= intervals.length <= 10^4
// - intervals[i].length == 2
// - 0 <= starti <= endi <= 10^4
//

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));

        let mut res = vec![];
        for interval in intervals {
            let prev_interval = match res.last_mut() {
                Some(a) => a,
                None => {
                    res.push(interval);
                    continue;
                }
            };
            if prev_interval[1] >= interval[0] {
                // NOTE: overlaps
                *prev_interval = vec![prev_interval[0], std::cmp::max(interval[1], prev_interval[1])];
            } else {
                res.push(interval);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![]], vec![vec![]]),
        (vec![vec![1]], vec![vec![1]]),
        (
            vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        ),
        (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
        (vec![vec![1, 4], vec![2, 3]], vec![vec![1, 4]]),
        (vec![vec![1, 4], vec![0, 4]], vec![vec![0, 4]]),
        (vec![vec![1, 4], vec![0, 0]], vec![vec![0, 0], vec![1, 4]]),
        (vec![vec![1, 4], vec![0, 1]], vec![vec![0, 4]]),
        (vec![vec![1, 4], vec![2, 3]], vec![vec![1, 4]]),
        (vec![vec![1, 4], vec![2, 4]], vec![vec![1, 4]]),
        (vec![vec![1, 4], vec![2, 5]], vec![vec![1, 5]]),
        (vec![vec![1, 4], vec![2, 6]], vec![vec![1, 6]]),
    ];
    for (intervals, expect) in cases {
        assert_eq!(Solution::merge(intervals), expect);
    }
}
