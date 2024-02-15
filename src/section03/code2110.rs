#![allow(dead_code)]

/*

// 2110. Number of Smooth Descent Periods of a Stock
// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/
// https://leetcode.cn/problems/number-of-smooth-descent-periods-of-a-stock/
//
// Medium
//
// You are given an integer array prices representing the daily price history of a stock, where prices[i] is the stock price on the ith day.

A smooth descent period of a stock consists of one or more contiguous days such that the price on each day is lower than the price on the preceding day by exactly 1. The first day of the period is exempted from this rule.

Return the number of smooth descent periods.

Example 1:

Input: prices = [3,2,1,4]
Output: 7
Explanation: There are 7 smooth descent periods:
[3], [2], [1], [4], [3,2], [2,1], and [3,2,1]
Note that a period with one day is a smooth descent period by the definition.

Example 2:

Input: prices = [8,6,7,7]
Output: 4
Explanation: There are 4 smooth descent periods: [8], [6], [7], and [7]
Note that [8,6] is not a smooth descent period as 8 - 6 ≠ 1.

Example 3:

Input: prices = [1]
Output: 1
Explanation: There is 1 smooth descent period: [1]

Constraints:

    1 <= prices.length <= 10^5
    1 <= prices[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        pub fn smooth(arr: &[i32], index: usize) -> usize {
            let mut count: usize = 1;
            if index < arr.len() - 1 {
                for i in index..(arr.len() - 1) {
                    if arr[i] - arr[i + 1] == 1 {
                        count += 1;
                    } else {
                        break;
                    }
                }
            }
            count
        }

        let mut count: usize = 0;
        let mut index: usize = 0;
        while index < prices.len() {
            let temp = smooth(&prices, index);
            count += (temp * (temp + 1)) / 2;
            index += temp;
        }
        count as i64
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_descent_periods(vec![3, 2, 1, 4]), 7);
    assert_eq!(Solution::get_descent_periods(vec![8, 6, 7, 7]), 4);
    assert_eq!(Solution::get_descent_periods(vec![1]), 1);
}
