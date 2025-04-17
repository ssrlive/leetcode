#![allow(dead_code)]

/*

// 2071. Maximum Number of Tasks You Can Assign
// https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/
// https://leetcode.cn/problems/maximum-number-of-tasks-you-can-assign/
//
// Hard
//
// You have n tasks and m workers. Each task has a strength requirement stored in a 0-indexed integer array tasks, with the ith task requiring tasks[i] strength to complete. The strength of each worker is stored in a 0-indexed integer array workers, with the jth worker having workers[j] strength. Each worker can only be assigned to a single task and must have a strength greater than or equal to the task's strength requirement (i.e., workers[j] >= tasks[i]).

Additionally, you have pills magical pills that will increase a worker's strength by strength. You can decide which workers receive the magical pills, however, you may only give each worker at most one magical pill.

Given the 0-indexed integer arrays tasks and workers and the integers pills and strength, return the maximum number of tasks that can be completed.

Example 1:

Input: tasks = [3,2,1], workers = [0,3,3], pills = 1, strength = 1
Output: 3
Explanation:
We can assign the magical pill and tasks as follows:
- Give the magical pill to worker 0.
- Assign worker 0 to task 2 (0 + 1 >= 1)
- Assign worker 1 to task 1 (3 >= 2)
- Assign worker 2 to task 0 (3 >= 3)

Example 2:

Input: tasks = [5,4], workers = [0,0,0], pills = 1, strength = 5
Output: 1
Explanation:
We can assign the magical pill and tasks as follows:
- Give the magical pill to worker 0.
- Assign worker 0 to task 0 (0 + 5 >= 5)

Example 3:

Input: tasks = [10,15,30], workers = [0,10,10,10,10], pills = 3, strength = 10
Output: 2
Explanation:
We can assign the magical pills and tasks as follows:
- Give the magical pill to worker 0 and worker 1.
- Assign worker 0 to task 0 (0 + 10 >= 10)
- Assign worker 1 to task 1 (10 + 10 >= 15)
The last pill is not given because it will not make any worker strong enough for the last task.

Constraints:

    n == tasks.length
    m == workers.length
    1 <= n, m <= 5 * 10^4
    0 <= pills <= m
    0 <= tasks[i], workers[j], strength <= 10^9
*/

struct Solution;

impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        use std::collections::VecDeque;

        fn check(tasks: &[i32], workers: &[i32], pills: i32, strength: i32, k: usize) -> bool {
            let mut pills = pills;
            if k > workers.len() {
                return false;
            }
            let mut t = 0;
            let mut q = VecDeque::new();
            for i in (0..k).rev() {
                if q.is_empty() && t < k {
                    q.push_front(tasks[t]);
                    t += 1;
                }
                if *q.back().unwrap() <= workers[i] {
                    q.pop_back();
                } else {
                    if pills == 0 {
                        return false;
                    }
                    if *q.back().unwrap() > workers[i] + strength {
                        return false;
                    }
                    while t < k && tasks[t] <= workers[i] + strength {
                        q.push_front(tasks[t]);
                        t += 1;
                    }
                    q.pop_front();
                    pills -= 1;
                }
            }
            true
        }

        let (mut l, mut r) = (0, tasks.len());
        let mut tasks = tasks;
        let mut workers = workers;
        tasks.sort();
        workers.sort_by(|a, b| b.cmp(a));
        while l < r {
            let mid = (l + r).div_ceil(2);
            if check(&tasks, &workers, pills, strength, mid) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        l as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 2, 1], vec![0, 3, 3], 1, 1, 3),
        (vec![5, 4], vec![0, 0, 0], 1, 5, 1),
        (vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10, 2),
    ];
    for (t, w, pills, strength, expected) in cases {
        assert_eq!(Solution::max_task_assign(t, w, pills, strength), expected);
    }
}
