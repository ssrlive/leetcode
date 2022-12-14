#![allow(dead_code)]

// 739. Daily Temperatures
// https://leetcode.com/problems/daily-temperatures/
//
// Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days
// you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
//
// Example 1:
//
// Input: temperatures = [73,74,75,71,69,72,76,73]
// Output: [1,1,4,2,1,1,0,0]
//
// Example 2:
//
// Input: temperatures = [30,40,50,60]
// Output: [1,1,1,0]
//
// Example 3:
//
// Input: temperatures = [30,60,90]
// Output: [1,1,0]
//
// Constraints:
//
// - 1 <= temperatures.length <= 10^5
// - 30 <= temperatures[i] <= 100
//

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack = VecDeque::new();

        for (i, &t) in temperatures.iter().enumerate() {
            while let Some(&j) = stack.back() {
                if temperatures[j] < t {
                    result[j] = (i - j) as i32;
                    stack.pop_back();
                } else {
                    break;
                }
            }
            stack.push_back(i);
        }

        result
    }
}

#[test]
fn test() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    assert_eq!(Solution::daily_temperatures(temperatures), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    assert_eq!(Solution::daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    assert_eq!(Solution::daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
}
