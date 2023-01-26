#![allow(dead_code)]

// 1335. Minimum Difficulty of a Job Schedule
// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/
// https://leetcode.cn/problems/minimum-difficulty-of-a-job-schedule/
//
// Hard
//
// You want to schedule a list of jobs in d days. Jobs are dependent (i.e To work on the ith job, you have to finish all the jobs j where 0 <= j < i).
//
// You have to finish at least one task every day. The difficulty of a job schedule is the sum of difficulties of each day of the d days.
// The difficulty of a day is the maximum difficulty of a job done on that day.
//
// You are given an integer array jobDifficulty and an integer d. The difficulty of the ith job is jobDifficulty[i].
//
// Return the minimum difficulty of a job schedule. If you cannot find a schedule for the jobs return -1.
//
// Example 1:
//
// Input: jobDifficulty = [6,5,4,3,2,1], d = 2
// Output: 7
// Explanation: First day you can finish the first 5 jobs, total difficulty = 6.
// Second day you can finish the last job, total difficulty = 1.
// The difficulty of the schedule = 6 + 1 = 7
//
// Example 2:
//
// Input: jobDifficulty = [9,9,9], d = 4
// Output: -1
// Explanation: If you finish a job per day you will still have a free day. you cannot find a schedule for the given jobs.
//
// Example 3:
//
// Input: jobDifficulty = [1,1,1], d = 3
// Output: 3
// Explanation: The schedule is one job per day. total difficulty will be 3.
//
// Constraints:
//
// -    1 <= jobDifficulty.length <= 300
// -    0 <= jobDifficulty[i] <= 1000
// -    1 <= d <= 10
//

struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;
        let mut dp = vec![vec![i32::MAX; d]; n];

        for k in 0..d {
            if k == 0 {
                let mut mx = 0;
                for i in 0..n {
                    mx = mx.max(job_difficulty[i]);
                    dp[i][0] = mx;
                }
            } else {
                for i in k..n {
                    let mut mx = 0;
                    for j in (k..=i).rev() {
                        mx = mx.max(job_difficulty[j]);
                        if dp[j - 1][k - 1] == i32::MAX {
                            break;
                        }
                        dp[i][k] = dp[i][k].min(mx + dp[j - 1][k - 1]);
                    }
                }
            }
        }
        if dp[n - 1][d - 1] == i32::MAX {
            return -1;
        }
        dp[n - 1][d - 1]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![6, 5, 4, 3, 2, 1], 2, 7),
        (vec![9, 9, 9], 4, -1),
        (vec![1, 1, 1], 3, 3),
        (vec![7, 1, 7, 1, 7, 1], 3, 15),
        (vec![11, 111, 22, 222, 33, 333, 44, 444], 6, 843),
    ];
    for (job_difficulty, d, expected) in cases {
        assert_eq!(Solution::min_difficulty(job_difficulty, d), expected);
    }
}
