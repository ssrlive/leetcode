#![allow(dead_code)]

/*

// 2235. Add Two Integers
// https://leetcode.com/problems/add-two-integers/
// https://leetcode.cn/problems/add-two-integers/
//
// Easy
//
// Given two integers num1 and num2, return the sum of the two integers.

Example 1:

Input: num1 = 12, num2 = 5
Output: 17
Explanation: num1 is 12, num2 is 5, and their sum is 12 + 5 = 17, so 17 is returned.

Example 2:

Input: num1 = -10, num2 = 4
Output: -6
Explanation: num1 + num2 = -6, so -6 is returned.

Constraints:

    -100 <= num1, num2 <= 100
*/

struct Solution;

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum(12, 5), 17);
    assert_eq!(Solution::sum(-10, 4), -6);
}
