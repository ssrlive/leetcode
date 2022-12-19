#![allow(dead_code)]

// 322. Coin Change
// https://leetcode.com/problems/coin-change/
// https://leetcode.cn/problems/coin-change/
//
// You are given an integer array coins representing coins of different denominations
// and an integer amount representing a total amount of money.
//
// Return the fewest number of coins that you need to make up that amount.
// If that amount of money cannot be made up by any combination of the coins, return -1.
//
// You may assume that you have an infinite number of each kind of coin.
//
// Example 1:
//
// Input: coins = [1,2,5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1
//
// Example 2:
//
// Input: coins = [2], amount = 3
// Output: -1
//
// Example 3:
//
// Input: coins = [1], amount = 0
// Output: 0
//
// Constraints:
//
// - 1 <= coins.length <= 12
// - 1 <= coins[i] <= 2^31 - 1
// - 0 <= amount <= 10^4
//

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for &coin in coins.iter() {
                if i - coin >= 0 {
                    dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - coin) as usize] + 1);
                }
            }
        }
        if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[test]
fn test_coin_change() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
}
