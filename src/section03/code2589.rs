#![allow(dead_code)]

/*

// 2589. Minimum Time to Complete All Tasks
// https://leetcode.com/problems/minimum-time-to-complete-all-tasks/
// https://leetcode.cn/problems/minimum-time-to-complete-all-tasks/
//
// Hard
//
// There is a computer that can run an unlimited number of tasks at the same time.
// You are given a 2D integer array tasks where tasks[i] = [starti, endi, durationi] indicates
// that the ith task should run for a total of durationi seconds (not necessarily continuous)
// within the inclusive time range [starti, endi].

You may turn on the computer only when it needs to run a task. You can also turn it off if it is idle.

Return the minimum time during which the computer should be turned on to complete all tasks.

Example 1:

Input: tasks = [[2,3,1],[4,5,1],[1,5,2]]
Output: 2
Explanation:
- The first task can be run in the inclusive time range [2, 2].
- The second task can be run in the inclusive time range [5, 5].
- The third task can be run in the two inclusive time ranges [2, 2] and [5, 5].
The computer will be on for a total of 2 seconds.

Example 2:

Input: tasks = [[1,3,2],[2,5,3],[5,6,2]]
Output: 4
Explanation:
- The first task can be run in the inclusive time range [2, 3].
- The second task can be run in the inclusive time ranges [2, 3] and [5, 5].
- The third task can be run in the two inclusive time range [5, 6].
The computer will be on for a total of 4 seconds.

Constraints:

    1 <= tasks.length <= 2000
    tasks[i].length == 3
    1 <= starti, endi <= 2000
    1 <= durationi <= endi - starti + 1
*/

struct Solution;

impl Solution {
    pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut line = vec![false; 2001];
        let mut tasks = tasks;
        tasks.sort_by(|t1, t2| t1[1].cmp(&t2[1]));
        for t in tasks.iter() {
            let st = t[0] as usize;
            let end = t[1] as usize;
            let mut d = t[2];
            d -= line[st..end + 1].iter().filter(|&&x| x).count() as i32;
            let mut i = end;
            while d > 0 {
                d -= i32::from(!line[i]);
                line[i] = true;
                i -= 1;
            }
        }
        line.iter().filter(|&&x| x).count() as i32
    }
}

#[test]
fn test() {
    let tasks = vec![vec![2, 3, 1], vec![4, 5, 1], vec![1, 5, 2]];
    assert_eq!(Solution::find_minimum_time(tasks), 2);
    let tasks = vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]];
    assert_eq!(Solution::find_minimum_time(tasks), 4);
    let tasks = vec![vec![8, 19, 1], vec![3, 20, 1], vec![1, 20, 2], vec![6, 13, 3]];
    assert_eq!(Solution::find_minimum_time(tasks), 3);
}
