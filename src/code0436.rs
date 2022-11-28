#![allow(dead_code)]

// 436. Find Right Interval
// https://leetcode.com/problems/find-right-interval/
//
// You are given an array of intervals, where intervals[i] = [starti, endi] and each starti is unique.
//
// The right interval for an interval i is an interval j such that startj >= endi and startj is minimized. Note that i may equal j.
//
// Return an array of right interval indices for each interval i. If no right interval exists for interval i, then put -1 at index i.
//
// Example 1:
//
// Input: intervals = [[1,2]]
// Output: [-1]
// Explanation: There is only one interval in the collection, so it outputs -1.
//
// Example 2:
//
// Input: intervals = [[3,4],[2,3],[1,2]]
// Output: [-1,0,1]
// Explanation: There is no right interval for [3,4].
// The right interval for [2,3] is [3,4] since start0 = 3 is the smallest start that is >= end1 = 3.
// The right interval for [1,2] is [2,3] since start1 = 2 is the smallest start that is >= end2 = 2.
//
// Example 3:
//
// Input: intervals = [[1,4],[2,3],[3,4]]
// Output: [-1,2,-1]
// Explanation: There is no right interval for [1,4] and [3,4].
// The right interval for [2,3] is [3,4] since start2 = 3 is the smallest start that is >= end1 = 3.
//
// Constraints:
//
// - 1 <= intervals.length <= 2 * 104
// - intervals[i].length == 2
// - -106 <= starti <= endi <= 106
// - The start point of each interval is unique.
//

struct Solution;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut btm: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
        for (i, interval) in intervals.iter().enumerate() {
            btm.insert(interval[0], i);
        }
        let mut answer: Vec<i32> = Vec::with_capacity(intervals.len());
        for interval in intervals.iter() {
            if let Some(e) = btm.range(interval[1]..).next() {
                answer.push(*e.1 as i32);
            } else {
                answer.push(-1);
            }
        }
        answer
    }
}

#[test]
fn test_find_right_interval() {
    assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
    assert_eq!(
        Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
        vec![-1, 0, 1]
    );
    assert_eq!(
        Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        vec![-1, 2, -1]
    );
}
