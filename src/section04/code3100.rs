#![allow(dead_code)]

// 3100. Water Bottles II
// https://leetcode.com/problems/water-bottles-ii/
// https://leetcode.cn/problems/water-bottles-ii/
//
// Medium
//
// You are given two integers numBottles and numExchange.
//
// numBottles represents the number of full water bottles that you initially have.
// In one operation, you can perform one of the following operations:
//
// - Drink any number of full water bottles turning them into empty bottles.
// - Exchange numExchange empty bottles with one full water bottle. Then, increase numExchange by one.
//
// Note that you cannot exchange multiple batches of empty bottles for the same value of numExchange.
// For example, if numBottles == 3 and numExchange == 1, you cannot exchange 3 empty water bottles for 3 full bottles.
//
// Return the maximum number of water bottles you can drink.
//
// Example 1:
//
// Input: numBottles = 13, numExchange = 6
// Output: 15
// Explanation: The table above shows the number of full water bottles, empty water bottles,
// the value of numExchange, and the number of bottles drunk.
//
// Example 2:
//
// Input: numBottles = 10, numExchange = 3
// Output: 13
// Explanation: The table above shows the number of full water bottles, empty water bottles,
// the value of numExchange, and the number of bottles drunk.
//
// Constraints:
//
//     1 <= numBottles <= 100
//     1 <= numExchange <= 100
//

struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num_bottles = num_bottles;
        let mut num_exchange = num_exchange;
        let mut res = 0;
        while num_bottles >= num_exchange {
            res += num_exchange;
            num_bottles -= num_exchange - 1;
            num_exchange += 1;
        }
        res + num_bottles
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
    assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
}
