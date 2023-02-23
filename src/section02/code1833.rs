#![allow(dead_code)]

/*

// 1833. Maximum Ice Cream Bars
// https://leetcode.com/problems/maximum-ice-cream-bars/
// https://leetcode.cn/problems/maximum-ice-cream-bars/
//
// Medium
//
// It is a sweltering summer day, and a boy wants to buy some ice cream bars.

At the store, there are n ice cream bars. You are given an array costs of length n, where costs[i] is the price of the ith ice cream bar in coins. The boy initially has coins coins to spend, and he wants to buy as many ice cream bars as possible.

Note: The boy can buy the ice cream bars in any order.

Return the maximum number of ice cream bars the boy can buy with coins coins.

You must solve the problem by counting sort.

Example 1:

Input: costs = [1,3,2,4,1], coins = 7
Output: 4
Explanation: The boy can buy ice cream bars at indices 0,1,2,4 for a total price of 1 + 3 + 2 + 1 = 7.

Example 2:

Input: costs = [10,6,8,7,7,8], coins = 5
Output: 0
Explanation: The boy cannot afford any of the ice cream bars.

Example 3:

Input: costs = [1,6,3,1,2,5], coins = 20
Output: 6
Explanation: The boy can buy all the ice cream bars for a total price of 1 + 6 + 3 + 1 + 2 + 5 = 18.

Constraints:

    costs.length == n
    1 <= n <= 10^5
    1 <= costs[i] <= 10^5
    1 <= coins <= 10^8
*/

struct Solution;

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut bars = [0_u32; 100_001];
        let mut coins = coins;
        let mut price = 1;
        let mut count = 0;

        costs.into_iter().for_each(|c| bars[c as usize] += 1);

        while coins >= price && price <= 100_000 {
            while bars[price as usize] > 0 && coins >= price {
                bars[price as usize] -= 1;
                coins -= price;
                count += 1;
            }
            price += 1;
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
    assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
    assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
}
