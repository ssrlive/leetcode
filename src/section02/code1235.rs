#![allow(dead_code)]

// 1235. Maximum Profit in Job Scheduling
// https://leetcode.com/problems/maximum-profit-in-job-scheduling/
// https://leetcode.cn/problems/maximum-profit-in-job-scheduling/
//
// Hard
//
// We have n jobs, where every job is scheduled to be done from startTime[i] to endTime[i], obtaining a profit of profit[i].
//
// You're given the startTime, endTime and profit arrays, return the maximum profit you can take such that there are no two jobs in the subset with overlapping time range.
//
// If you choose a job that ends at time X you will be able to start another job that starts at time X.
//
// Example 1:
//
// Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
// Output: 120
// Explanation: The subset chosen is the first and fourth job.
// Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
//
// Example 2:
//
// Input: startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,70,60]
// Output: 150
// Explanation: The subset chosen is the first, fourth and fifth job.
// Profit obtained 150 = 20 + 70 + 60.
//
// Example 3:
//
// Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
// Output: 6
//
// Constraints:
//
// -    1 <= startTime.length == endTime.length == profit.length <= 5 * 10^4
// -    1 <= startTime[i] < endTime[i] <= 10^9
// -    1 <= profit[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        let len = profit.len();
        let mut index = (0..len).collect::<Vec<_>>();
        index.sort_by_cached_key(|&i| end_time[i]);
        let mut btm = BTreeMap::new();
        btm.insert(0, 0);
        for &i in &index {
            if let Some((_, &p)) = btm.range(..=start_time[i]).last() {
                if p + profit[i] > *btm.iter().last().unwrap().1 {
                    btm.insert(end_time[i], p + profit[i]);
                }
            }
        }
        *btm.iter().last().unwrap().1
    }
}

#[test]
fn test() {
    let start_time = vec![1, 2, 3, 3];
    let end_time = vec![3, 4, 5, 6];
    let profit = vec![50, 10, 40, 70];
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 120);

    let start_time = vec![1, 2, 3, 4, 6];
    let end_time = vec![3, 5, 10, 6, 9];
    let profit = vec![20, 20, 100, 70, 60];
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 150);

    let start_time = vec![1, 1, 1];
    let end_time = vec![2, 3, 4];
    let profit = vec![5, 6, 4];
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 6);
}
