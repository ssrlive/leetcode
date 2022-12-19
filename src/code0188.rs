#![allow(dead_code)]

// 188. Best Time to Buy and Sell Stock IV
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv/
//
// You are given an integer array prices where prices[i] is the price of a given stock on the ith day, and an integer k.
//
// Find the maximum profit you can achieve. You may complete at most k transactions.
//
// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
//

struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; prices.len()]; k as usize + 1];
        for i in 1..=k {
            let mut max = dp[i as usize - 1][0] - prices[0];
            for (j, item) in prices.iter().enumerate().skip(1) {
                dp[i as usize][j] = std::cmp::max(dp[i as usize][j - 1], item + max);
                max = std::cmp::max(max, dp[i as usize - 1][j] - item);
            }
        }
        dp[k as usize][prices.len() - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
    assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
}
