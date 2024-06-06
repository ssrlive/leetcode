#![allow(dead_code)]

// 3169. Count Days Without Meetings
// https://leetcode.com/problems/count-days-without-meetings/
// https://leetcode.cn/problems/count-days-without-meetings/
//
// Medium
//
// You are given a positive integer days representing the total number of days an employee is available for work (starting from day 1).
// You are also given a 2D array meetings of size n where, meetings[i] = [start_i, end_i] represents the starting and ending days of meeting i (inclusive).
//
// Return the count of days when the employee is available for work but no meetings are scheduled.
//
// Note: The meetings may overlap.
//
// Example 1:
//
// Input: days = 10, meetings = [[5,7],[1,3],[9,10]]
//
// Output: 2
//
// Explanation:
//
// There is no meeting scheduled on the 4th and 8th days.
//
// Example 2:
//
// Input: days = 5, meetings = [[2,4],[1,3]]
//
// Output: 1
//
// Explanation:
//
// There is no meeting scheduled on the 5th day.
//
// Example 3:
//
// Input: days = 6, meetings = [[1,6]]
//
// Output: 0
//
// Explanation:
//
// Meetings are scheduled for all working days.
//
// Constraints:
//
// 1 <= days <= 10^9
// 1 <= meetings.length <= 10^5
// meetings[i].length == 2
// 1 <= meetings[i][0] <= meetings[i][1] <= days
//

struct Solution;

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        meetings.sort_unstable();
        let (mut work_until, mut res) = (0, 0);
        for v in meetings {
            if v[0] > work_until {
                res += v[0] - work_until - 1;
            }
            work_until = work_until.max(v[1]);
        }

        res + days - work_until
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]), 2);
    assert_eq!(Solution::count_days(5, vec![vec![2, 4], vec![1, 3]]), 1);
    assert_eq!(Solution::count_days(6, vec![vec![1, 6]]), 0);
}
