#![allow(dead_code)]

/*

// 2398. Maximum Number of Robots Within Budget
// https://leetcode.com/problems/maximum-number-of-robots-within-budget/
// https://leetcode.cn/problems/maximum-number-of-robots-within-budget/
//
// Hard
//
// You have n robots. You are given two 0-indexed integer arrays, chargeTimes and runningCosts,
// both of length n. The ith robot costs chargeTimes[i] units to charge and costs runningCosts[i] units to run.
// You are also given an integer budget.

The total cost of running k chosen robots is equal to max(chargeTimes) + k * sum(runningCosts), where max(chargeTimes) is the largest charge cost among the k robots and sum(runningCosts) is the sum of running costs among the k robots.

Return the maximum number of consecutive robots you can run such that the total cost does not exceed budget.

Example 1:

Input: chargeTimes = [3,6,1,3,4], runningCosts = [2,1,3,4,5], budget = 25
Output: 3
Explanation:
It is possible to run all individual and consecutive pairs of robots within budget.
To obtain answer 3, consider the first 3 robots. The total cost will be max(3,6,1) + 3 * sum(2,1,3) = 6 + 3 * 6 = 24 which is less than 25.
It can be shown that it is not possible to run more than 3 consecutive robots within budget, so we return 3.

Example 2:

Input: chargeTimes = [11,12,19], runningCosts = [10,8,7], budget = 19
Output: 0
Explanation: No robot can be run that does not exceed the budget, so we return 0.

Constraints:

    chargeTimes.length == runningCosts.length == n
    1 <= n <= 5 * 10^4
    1 <= chargeTimes[i], runningCosts[i] <= 10^5
    1 <= budget <= 10^15
*/

struct Solution;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = charge_times.len();
        let (mut tree, mut sum) = (vec![0; 4 * n], vec![0; n + 1]);

        for i in 0..n {
            Self::add(&mut tree, 1, 0, n - 1, i, charge_times[i]);
            sum[i + 1] = sum[i] + running_costs[i] as i64;
        }

        let (mut lo, mut hi) = (0, n);
        while lo < hi {
            let md = hi - (hi - lo) / 2;
            let mut good = false;
            for i in 0..=n - md {
                let mut temp = (sum[md + i] - sum[i]) * md as i64;
                temp += Self::get(&tree, 1, 0, n - 1, i, i + md - 1) as i64;
                if temp <= budget {
                    good = true;
                    break;
                }
            }
            if good {
                lo = md;
            } else {
                hi = md - 1;
            }
        }
        lo as i32
    }

    fn add(tree: &mut Vec<i32>, u: usize, left: usize, right: usize, i: usize, val: i32) {
        if left == right {
            tree[u] = val;
            return;
        }

        let mid = left + (right - left) / 2;

        if i <= mid {
            Self::add(tree, 2 * u, left, mid, i, val);
        } else {
            Self::add(tree, 2 * u + 1, mid + 1, right, i, val);
        }

        tree[u] = i32::max(tree[2 * u], tree[2 * u + 1]);
    }

    fn get(tree: &Vec<i32>, u: usize, left: usize, right: usize, l: usize, r: usize) -> i32 {
        if left > r || right < l {
            return 0;
        }
        if left >= l && right <= r {
            return tree[u];
        }

        let mid = left + (right - left) / 2;

        let t1 = Self::get(tree, 2 * u, left, mid, l, r);
        let t2 = Self::get(tree, 2 * u + 1, mid + 1, right, l, r);

        i32::max(t1, t2)
    }
}

#[test]
fn test() {
    let charge_times = vec![3, 6, 1, 3, 4];
    let running_costs = vec![2, 1, 3, 4, 5];
    let budget = 25;
    assert_eq!(Solution::maximum_robots(charge_times, running_costs, budget), 3);

    let charge_times = vec![11, 12, 19];
    let running_costs = vec![10, 8, 7];
    let budget = 19;
    assert_eq!(Solution::maximum_robots(charge_times, running_costs, budget), 0);
}
