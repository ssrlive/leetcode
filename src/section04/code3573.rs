#![allow(dead_code)]

// 3573. Best Time to Buy and Sell Stock V
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-v/
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-v/
//
// Medium
//
// You are given an integer array prices where prices[i] is the price of a stock in dollars on the ith day, and an integer k.
//
// You are allowed to make at most k transactions, where each transaction can be either of the following:
//
//     Normal transaction: Buy on day i, then sell on a later day j where i < j. You profit prices[j] - prices[i].
//
//     Short selling transaction: Sell on day i, then buy back on a later day j where i < j. You profit prices[i] - prices[j].
//
// Note that you must complete each transaction before starting another. Additionally, you can't buy
// or sell on the same day you are selling or buying back as part of a previous transaction.
//
// Return the maximum total profit you can earn by making at most k transactions.
//
// Example 1:
//
// Input: prices = [1,7,9,8,2], k = 2
//
// Output: 14
//
// Explanation:
// We can make $14 of profit through 2 transactions:
//
//     A normal transaction: buy the stock on day 0 for $1 then sell it on day 2 for $9.
//     A short selling transaction: sell the stock on day 3 for $8 then buy back on day 4 for $2.
//
// Example 2:
//
// Input: prices = [12,16,19,19,8,1,19,13,9], k = 3
//
// Output: 36
//
// Explanation:
// We can make $36 of profit through 3 transactions:
//
//     A normal transaction: buy the stock on day 0 for $12 then sell it on day 2 for $19.
//     A short selling transaction: sell the stock on day 3 for $19 then buy back on day 4 for $8.
//     A normal transaction: buy the stock on day 5 for $1 then sell it on day 6 for $19.
//
// Constraints:
//
//     2 <= prices.length <= 10^3
//     1 <= prices[i] <= 10^9
//     1 <= k <= prices.length / 2
//

struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let prices: Vec<i64> = prices.iter().map(|&x| x as i64).collect();
        let k = k as i64;

        let n = prices.len();
        let mn = -1e14 as i64;
        let mut curr = vec![vec![mn; 3]; (k + 1) as usize];
        let mut next = vec![vec![mn; 3]; (k + 1) as usize];
        // Base case at day n
        for k in 0..=k {
            next[k as usize][0] = 0;
        }
        for i in (0..n).rev() {
            for k in 0..=k {
                // State 0
                curr[k as usize][0] = next[k as usize][0];
                curr[k as usize][0] = curr[k as usize][0].max(next[k as usize][1] - prices[i]); // start buy
                curr[k as usize][0] = curr[k as usize][0].max(next[k as usize][2] + prices[i]); // start sell

                // States 1 and 2 carry forward
                curr[k as usize][1] = next[k as usize][1];
                curr[k as usize][2] = next[k as usize][2];

                if k > 0 {
                    // Complete transactions
                    curr[k as usize][1] = curr[k as usize][1].max(next[(k - 1) as usize][0] + prices[i]); // sell the bought stock
                    curr[k as usize][2] = curr[k as usize][2].max(next[(k - 1) as usize][0] - prices[i]); // buy the sold stock
                }
            }

            next.clone_from(&curr);
        }
        next[k as usize][0]
    }
}

#[test]
fn test() {
    let prices = vec![1, 7, 9, 8, 2];
    let k = 2;
    assert_eq!(Solution::maximum_profit(prices, k), 14);

    let prices = vec![12, 16, 19, 19, 8, 1, 19, 13, 9];
    let k = 3;
    assert_eq!(Solution::maximum_profit(prices, k), 36);
}
