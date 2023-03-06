#![allow(dead_code)]

/*

// 2244. Minimum Rounds to Complete All Tasks
// https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/
// https://leetcode.cn/problems/minimum-rounds-to-complete-all-tasks/
//
// Medium
//
// You are given a 0-indexed integer array tasks, where tasks[i] represents the difficulty level of a task. In each round, you can complete either 2 or 3 tasks of the same difficulty level.

Return the minimum rounds required to complete all the tasks, or -1 if it is not possible to complete all the tasks.

Example 1:

Input: tasks = [2,2,3,3,2,4,4,4,4,4]
Output: 4
Explanation: To complete all the tasks, a possible plan is:
- In the first round, you complete 3 tasks of difficulty level 2.
- In the second round, you complete 2 tasks of difficulty level 3.
- In the third round, you complete 3 tasks of difficulty level 4.
- In the fourth round, you complete 2 tasks of difficulty level 4.
It can be shown that all the tasks cannot be completed in fewer than 4 rounds, so the answer is 4.

Example 2:

Input: tasks = [2,3,3]
Output: -1
Explanation: There is only 1 task of difficulty level 2, but in each round, you can only complete either 2 or 3 tasks of the same difficulty level. Hence, you cannot complete all the tasks, and the answer is -1.

Constraints:

    1 <= tasks.length <= 10^5
    1 <= tasks[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        tasks
            .into_iter()
            .fold(HashMap::<i32, i32>::new(), |mut map, task| {
                *map.entry(task).or_default() += 1;
                map
            })
            .values()
            .try_fold(0, |state, cnt| (*cnt > 1).then_some(state + (*cnt + 2) / 3))
            .unwrap_or(-1)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4], 4),
        (vec![2, 3, 3], -1),
        (vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 4),
    ];
    for (tasks, expected) in cases {
        assert_eq!(Solution::minimum_rounds(tasks), expected);
    }
}
