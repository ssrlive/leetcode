#![allow(dead_code)]

// 309. Best Time to Buy and Sell Stock with Cooldown
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/
//
// You are given an array prices where prices[i] is the price of a given stock on the ith day.
//
// Find the maximum profit you can achieve. You may complete as many transactions as you like
// (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:
//
// - After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
//
// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
//
// Example 1:
//
// Input: prices = [1,2,3,0,2]
// Output: 3
// Explanation: transactions = [buy, sell, cooldown, buy, sell]
//
// Example 2:
//
// Input: prices = [1]
// Output: 0
//
// Constraints:
//
// - 1 <= prices.length <= 5000
// - 0 <= prices[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 2]; prices.len() + 1];
        dp[0][1] = i32::MIN;
        for i in 1..=prices.len() {
            dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + prices[i - 1]);
            dp[i][1] = dp[i - 1][1].max(if i >= 2 { dp[i - 2][0] } else { 0 } - prices[i - 1]);
        }
        dp[prices.len()][0]
    }
}

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    assert_eq!(Solution::max_profit(vec![1]), 0);
}
