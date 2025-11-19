#![allow(dead_code)]

// 3683. Earliest Time to Finish One Task
// https://leetcode.com/problems/earliest-time-to-finish-one-task/
// https://leetcode.cn/problems/earliest-time-to-finish-one-task/
//
// Easy
//
// You are given a 2D integer array tasks where tasks[i] = [si, ti].
//
// Each [si, ti] in tasks represents a task with start time si that takes ti units of time to finish.
//
// Return the earliest time at which at least one task is finished.
//
// Example 1:
//
// Input: tasks = [[1,6],[2,3]]
//
// Output: 5
//
// Explanation:
//
// The first task starts at time t = 1 and finishes at time 1 + 6 = 7. The second task finishes at time 2 + 3 = 5. You can finish one task at time 5.
//
// Example 2:
//
// Input: tasks = [[100,100],[100,100],[100,100]]
//
// Output: 200
//
// Explanation:
//
// All three tasks finish at time 100 + 100 = 200.
//
// Constraints:
//
// 1 <= tasks.length <= 100
// tasks[i] = [si, ti]
// 1 <= si, ti <= 100
//

struct Solution;

impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut ans = i32::MAX;
        for task in &tasks {
            ans = (task[0] + task[1]).min(ans);
        }
        ans
    }
}

#[test]
fn test() {
    let tasks = vec![vec![1, 6], vec![2, 3]];
    assert_eq!(Solution::earliest_time(tasks), 5);

    let tasks = vec![vec![100, 100], vec![100, 100], vec![100, 100]];
    assert_eq!(Solution::earliest_time(tasks), 200);
}
