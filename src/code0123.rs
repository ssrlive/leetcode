#![allow(dead_code)]

// 123. Best Time to Buy and Sell Stock III
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
//
// You are given an array prices where prices[i] is the price of a given stock on the ith day.
//
// Find the maximum profit you can achieve. You may complete at most two transactions.
//
// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let (mut fwd, mut bwd) = (vec![0; n], vec![0; n]);
        let (mut b, mut s) = (std::i32::MAX, 0);
        prices.iter().enumerate().for_each(|(i, p)| {
            fwd[i] = if i == 0 { 0 } else { std::cmp::max(fwd[i - 1], p - b) };
            b = std::cmp::min(b, *p);
        });
        (0..n).rev().for_each(|i| {
            let p = prices[i];
            bwd[i] = if i == n - 1 {
                0
            } else {
                std::cmp::max(bwd[i + 1], s - p)
            };
            s = std::cmp::max(s, p);
        });
        (0..n).fold(0, |acc, i| std::cmp::max(fwd[i] + bwd[i], acc))
    }
}

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
