#![allow(dead_code)]

/*

// 1648. Sell Diminishing-Valued Colored Balls
// https://leetcode.com/problems/sell-diminishing-valued-colored-balls/
// https://leetcode.cn/problems/sell-diminishing-valued-colored-balls/
//
// Medium
//
// You have an inventory of different colored balls, and there is a customer that wants orders balls of any color.

The customer weirdly values the colored balls. Each colored ball's value is the number of balls of that color you currently have in your inventory. For example, if you own 6 yellow balls, the customer would pay 6 for the first yellow ball. After the transaction, there are only 5 yellow balls left, so the next yellow ball is then valued at 5 (i.e., the value of the balls decreases as you sell more to the customer).

You are given an integer array, inventory, where inventory[i] represents the number of balls of the ith color that you initially own. You are also given an integer orders, which represents the total number of balls that the customer wants. You can sell the balls in any order.

Return the maximum total value that you can attain after selling orders colored balls. As the answer may be too large, return it modulo 109 + 7.

Example 1:

Input: inventory = [2,5], orders = 4
Output: 14
Explanation: Sell the 1st color 1 time (2) and the 2nd color 3 times (5 + 4 + 3).
The maximum total value is 2 + 5 + 4 + 3 = 14.

Example 2:

Input: inventory = [3,5], orders = 6
Output: 19
Explanation: Sell the 1st color 2 times (3 + 2) and the 2nd color 4 times (5 + 4 + 3 + 2).
The maximum total value is 3 + 2 + 5 + 4 + 3 + 2 = 19.

Constraints:

    1 <= inventory.length <= 10^5
    1 <= inventory[i] <= 10^9
    1 <= orders <= min(sum(inventory[i]), 10^9)
*/

struct Solution;

impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut inventory = inventory;
        let mut orders = orders;
        let mut answer: i64 = 0;
        inventory.sort_unstable_by(|a, b| b.cmp(a));
        let mut highest = inventory[0] as i64;
        let mut index = 0;
        for (i, &item) in inventory.iter().enumerate().skip(1) {
            let item = item as i64;
            if item < highest {
                let batch_size = (i as i64) * (highest - item);
                if batch_size < orders as i64 {
                    orders -= batch_size as i32;
                    answer += (i as i64) * (highest + item + 1) * (highest - item) / 2;
                    highest = item;
                } else {
                    break;
                }
            }
            index = i;
        }
        let i = index + 1;
        let h = orders / (i as i32);
        answer += (i as i64) * (h as i64) * (highest + (highest - (h as i64) + 1)) / 2;
        orders %= i as i32;
        highest -= h as i64;
        answer += (orders as i64) * highest;

        (answer % 1000000007) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit(vec![2, 5], 4), 14);
    assert_eq!(Solution::max_profit(vec![3, 5], 6), 19);
    assert_eq!(Solution::max_profit(vec![1000000000], 1000000000), 21);
}
