#![allow(dead_code)]

// 826. Most Profit Assigning Work
// https://leetcode.com/problems/most-profit-assigning-work/
// https://leetcode.cn/problems/most-profit-assigning-work/
//
// You have n jobs and m workers. You are given three arrays: difficulty, profit, and worker where:
//
// - difficulty[i] and profit[i] are the difficulty and the profit of the ith job, and
// - worker[j] is the ability of jth worker (i.e., the jth worker can only complete a job
//   with difficulty at most worker[j]).
//
// Every worker can be assigned at most one job, but one job can be completed multiple times.
//
// - For example, if three workers attempt the same job that pays $1, then the total profit will be $3.
//   If a worker cannot complete any job, their profit is $0.
//
// Return the maximum profit we can achieve after assigning the workers to the jobs.
//
// Example 1:
//
// Input: difficulty = [2,4,6,8,10], profit = [10,20,30,40,50], worker = [4,5,6,7]
// Output: 100
// Explanation: Workers are assigned jobs of difficulty [4,4,6,6] and they get a profit of [20,20,30,30] separately.
//
// Example 2:
//
// Input: difficulty = [85,47,57], profit = [24,66,99], worker = [40,25,25]
// Output: 0
//
// Constraints:
//
// - n == difficulty.length
// - n == profit.length
// - m == worker.length
// - 1 <= n, m <= 10^4
// - 1 <= difficulty[i], profit[i], worker[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut worker = worker;
        let n = difficulty.len();
        let mut memo = vec![(0, 0); n];
        for i in 0..n {
            memo[i] = (difficulty[i], profit[i]);
        }
        memo.sort();
        worker.sort();
        let mut ci = 0;
        let mut max = 0;
        let mut result = 0;
        for v in worker {
            while ci < n && memo[ci].0 <= v {
                max = std::cmp::max(max, memo[ci].1);
                ci += 1;
            }
            ci = ci.saturating_sub(1);
            result += max;
        }

        result
    }
}

#[test]
fn test() {
    let difficulty = vec![2, 4, 6, 8, 10];
    let profit = vec![10, 20, 30, 40, 50];
    let worker = vec![4, 5, 6, 7];
    assert_eq!(Solution::max_profit_assignment(difficulty, profit, worker), 100);

    let difficulty = vec![85, 47, 57];
    let profit = vec![24, 66, 99];
    let worker = vec![40, 25, 25];
    assert_eq!(Solution::max_profit_assignment(difficulty, profit, worker), 0);
}
