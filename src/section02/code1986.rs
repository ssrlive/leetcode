#![allow(dead_code)]

/*

// 1986. Minimum Number of Work Sessions to Finish the Tasks
// https://leetcode.com/problems/minimum-number-of-work-sessions-to-finish-the-tasks/
// https://leetcode.cn/problems/minimum-number-of-work-sessions-to-finish-the-tasks/
//
// Medium
//
// There are n tasks assigned to you. The task times are represented as an integer array tasks of length n, where the ith task takes tasks[i] hours to finish. A work session is when you work for at most sessionTime consecutive hours and then take a break.

You should finish the given tasks in a way that satisfies the following conditions:

    If you start a task in a work session, you must complete it in the same work session.
    You can start a new task immediately after finishing the previous one.
    You may complete the tasks in any order.

Given tasks and sessionTime, return the minimum number of work sessions needed to finish all the tasks following the conditions above.

The tests are generated such that sessionTime is greater than or equal to the maximum element in tasks[i].

Example 1:

Input: tasks = [1,2,3], sessionTime = 3
Output: 2
Explanation: You can finish the tasks in two work sessions.
- First work session: finish the first and the second tasks in 1 + 2 = 3 hours.
- Second work session: finish the third task in 3 hours.

Example 2:

Input: tasks = [3,1,3,1,1], sessionTime = 8
Output: 2
Explanation: You can finish the tasks in two work sessions.
- First work session: finish all the tasks except the last one in 3 + 1 + 3 + 1 = 8 hours.
- Second work session: finish the last task in 1 hour.

Example 3:

Input: tasks = [1,2,3,4,5], sessionTime = 15
Output: 1
Explanation: You can finish all the tasks in one work session.

Constraints:

    n == tasks.length
    1 <= n <= 14
    1 <= tasks[i] <= 10
    max(tasks[i]) <= sessionTime <= 15
*/

struct Solution;

impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        const INF: i32 = 15;
        let n = tasks.len();
        let mut s = vec![0; 1 << n];
        for (mask, s_mask) in s.iter_mut().enumerate() {
            let mut cur = 0;
            for (i, &task) in tasks.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    cur += task;
                }
            }
            *s_mask = cur;
        }
        let mut memo = vec![INF; 1 << n];
        memo[0] = 0;
        fn dp(mask: usize, s: &Vec<i32>, session_time: i32, memo: &mut Vec<i32>) -> i32 {
            if memo[mask] != INF {
                return memo[mask];
            }
            let mut cur = mask;
            let mut res = INF;
            while cur != 0 {
                if s[cur] <= session_time {
                    res = res.min(1 + dp(mask ^ cur, s, session_time, memo));
                }
                cur = (cur - 1) & mask;
            }
            memo[mask] = res;
            res
        }
        dp((1 << n) - 1, &s, session_time, &mut memo)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3], 3, 2),
        (vec![3, 1, 3, 1, 1], 8, 2),
        (vec![1, 2, 3, 4, 5], 15, 1),
    ];
    for (tasks, session_time, expect) in cases {
        assert_eq!(Solution::min_sessions(tasks, session_time), expect);
    }
}
