#![allow(dead_code)]

// 757. Set Intersection Size At Least Two
// https://leetcode.com/problems/set-intersection-size-at-least-two/
// https://leetcode.cn/problems/set-intersection-size-at-least-two/
//
// You are given a 2D integer array intervals where intervals[i] = [starti, endi] represents
// all the integers from starti to endi inclusively.
//
// A containing set is an array nums where each interval from intervals has at least two integers in nums.
//
// - For example, if intervals = [[1,3], [3,7], [8,9]], then [1,2,4,7,8,9] and [2,3,4,8,9] are containing sets.
//
// Return the minimum possible size of a containing set.
//
// Example 1:
//
// Input: intervals = [[1,3],[3,7],[8,9]]
// Output: 5
// Explanation: let nums = [2, 3, 4, 8, 9].
// It can be shown that there cannot be any containing array of size 4.
//
// Example 2:
//
// Input: intervals = [[1,3],[1,4],[2,5],[3,5]]
// Output: 3
// Explanation: let nums = [2, 3, 4].
// It can be shown that there cannot be any containing array of size 2.
//
// Example 3:
//
// Input: intervals = [[1,2],[2,3],[2,4],[4,5]]
// Output: 5
// Explanation: let nums = [1, 2, 3, 4, 5].
// It can be shown that there cannot be any containing array of size 4.
//
// Constraints:
//
// - 1 <= intervals.length <= 3000
// - intervals[i].length == 2
// - 0 <= starti < endi <= 10^8
//

struct Solution;

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        fn inside(x: i32, interval: &[i32]) -> bool {
            interval[0] <= x && x <= interval[1]
        }

        let mut intervals = intervals;
        intervals.sort_by(|a, b| if a[0] != b[0] { a[0].cmp(&b[0]) } else { b[1].cmp(&a[1]) });
        let mut n = 0;
        for interval in intervals.clone() {
            while n > 0 && interval[1] <= intervals[n - 1][1] {
                n -= 1;
            }
            intervals[n] = interval;
            n += 1;
        }
        let mut cnt = 0;
        let mut x = -1;
        let mut y = -1;
        for item in intervals.iter().take(n) {
            if !inside(y, item) {
                cnt += 2;
                x = item[1] - 1;
                y = x + 1;
            } else if !inside(x, item) {
                cnt += 1;
                x = y;
                y = item[1];
            }
        }
        cnt
    }
}

#[test]
fn test() {
    let intervals = vec![vec![1, 3], vec![3, 7], vec![8, 9]];
    assert_eq!(Solution::intersection_size_two(intervals), 5);
    let intervals = vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]];
    assert_eq!(Solution::intersection_size_two(intervals), 3);
    let intervals = vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]];
    assert_eq!(Solution::intersection_size_two(intervals), 5);
    let intervals = vec![vec![1, 3], vec![3, 7], vec![5, 7], vec![7, 8]];
    assert_eq!(Solution::intersection_size_two(intervals), 5);
}
