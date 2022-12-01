#![allow(dead_code)]

// 539. Minimum Time Difference
// https://leetcode.com/problems/minimum-time-difference/
//
// Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.
//
// Example 1:
//
// Input: timePoints = ["23:59","00:00"]
// Output: 1
//
// Example 2:
//
// Input: timePoints = ["00:00","23:59","00:00"]
// Output: 0
//
// Constraints:
//
// - 2 <= timePoints.length <= 2 * 104
// - timePoints[i] is in the format "HH:MM".
//

struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points = time_points
            .into_iter()
            .map(|time_point| {
                let mut time_point = time_point.split(':');
                let hour = time_point.next().unwrap().parse::<i32>().unwrap();
                let minute = time_point.next().unwrap().parse::<i32>().unwrap();
                hour * 60 + minute
            })
            .collect::<Vec<_>>();
        time_points.sort_unstable();
        let mut min_diff = i32::MAX;
        for i in 1..time_points.len() {
            min_diff = min_diff.min(time_points[i] - time_points[i - 1]);
        }
        min_diff.min(24 * 60 - time_points[time_points.len() - 1] + time_points[0])
    }
}

#[test]
fn test() {
    let time_points = vec!["23:59", "00:00"]
        .into_iter()
        .map(|time_point| time_point.to_string())
        .collect();
    assert_eq!(Solution::find_min_difference(time_points), 1);
    let time_points = vec!["00:00", "23:59", "00:00"]
        .into_iter()
        .map(|time_point| time_point.to_string())
        .collect();
    assert_eq!(Solution::find_min_difference(time_points), 0);
}
