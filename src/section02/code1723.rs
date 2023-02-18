#![allow(dead_code)]

/*

// 1723. Find Minimum Time to Finish All Jobs
Hard
774
20
Companies

You are given an integer array jobs, where jobs[i] is the amount of time it takes to complete the ith job.

There are k workers that you can assign jobs to. Each job should be assigned to exactly one worker. The working time of a worker is the sum of the time it takes to complete all jobs assigned to them. Your goal is to devise an optimal assignment such that the maximum working time of any worker is minimized.

Return the minimum possible maximum working time of any assignment.

Example 1:

Input: jobs = [3,2,3], k = 3
Output: 3
Explanation: By assigning each person one job, the maximum time is 3.

Example 2:

Input: jobs = [1,2,4,7,8], k = 2
Output: 11
Explanation: Assign the jobs the following way:
Worker 1: 1, 2, 8 (working time = 1 + 2 + 8 = 11)
Worker 2: 4, 7 (working time = 4 + 7 = 11)
The maximum working time is 11.

Constraints:

    1 <= k <= jobs.length <= 12
    1 <= jobs[i] <= 10^7
*/

struct Solution;

impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        fn check(jobs: &[i32], k: i32, limit: i32) -> bool {
            let mut workloads = vec![0; k as usize];
            backtrack(jobs, &mut workloads, 0, limit)
        }

        fn backtrack(jobs: &[i32], workloads: &mut [i32], i: usize, limit: i32) -> bool {
            if i >= jobs.len() {
                return true;
            }
            let cur = jobs[i];
            for j in 0..workloads.len() {
                if workloads[j] + cur <= limit {
                    workloads[j] += cur;
                    if backtrack(jobs, workloads, i + 1, limit) {
                        return true;
                    }
                    workloads[j] -= cur;
                }
                if workloads[j] == 0 || workloads[j] + cur == limit {
                    break;
                }
            }
            false
        }

        let mut jobs = jobs;
        jobs.sort();
        let mut left = jobs.iter().sum::<i32>() / k;
        let mut right = jobs.iter().sum::<i32>();
        while left < right {
            let mid = (left + right) / 2;
            if check(&jobs, k, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_time_required(vec![3, 2, 3], 3), 3);
    assert_eq!(Solution::minimum_time_required(vec![1, 2, 4, 7, 8], 2), 11);
}
