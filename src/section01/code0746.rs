#![allow(dead_code)]

// 746. Min Cost Climbing Stairs
// https://leetcode.com/problems/min-cost-climbing-stairs/
// https://leetcode.cn/problems/min-cost-climbing-stairs/
//
// You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
//
// You can either start from the step with index 0, or the step with index 1.
//
// Return the minimum cost to reach the top of the floor.
//
// Example 1:
//
// Input: cost = [10,15,20]
// Output: 15
// Explanation: You will start at index 1.
// - Pay 15 and climb two steps to reach the top.
// The total cost is 15.
//
// Example 2:
//
// Input: cost = [1,100,1,1,1,100,1,1,100,1]
// Output: 6
// Explanation: You will start at index 0.
// - Pay 1 and climb two steps to reach index 2.
// - Pay 1 and climb two steps to reach index 4.
// - Pay 1 and climb two steps to reach index 6.
// - Pay 1 and climb one step to reach index 7.
// - Pay 1 and climb two steps to reach index 9.
// - Pay 1 and climb one step to reach the top.
// The total cost is 6.
//
// Constraints:
//
// - 2 <= cost.length <= 1000
// - 0 <= cost[i] <= 999
//

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut m1 = 0;
        let mut m2 = 0;
        let mut i = 2usize;
        while i < cost.len() {
            let c = std::cmp::min(m1 + cost[i - 1], m2 + cost[i - 2]);
            i += 1;
            m2 = m1;
            m1 = c;
        }
        std::cmp::min(m1 + cost[cost.len() - 1], m2 + cost[cost.len() - 2])
    }
}

#[test]
fn test() {
    let cost = vec![10, 15, 20];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
}
