#![allow(dead_code)]

/*

// 2177. Find Three Consecutive Integers That Sum to a Given Number
// https://leetcode.com/problems/find-three-consecutive-integers-that-sum-to-a-given-number/
// https://leetcode.cn/problems/find-three-consecutive-integers-that-sum-to-a-given-number/
//
// Medium
//
// Given an integer num, return three consecutive integers (as a sorted array) that sum to num. If num cannot be expressed as the sum of three consecutive integers, return an empty array.

Example 1:

Input: num = 33
Output: [10,11,12]
Explanation: 33 can be expressed as 10 + 11 + 12 = 33.
10, 11, 12 are 3 consecutive integers, so we return [10, 11, 12].

Example 2:

Input: num = 4
Output: []
Explanation: There is no way to express 4 as the sum of 3 consecutive integers.

Constraints:

    0 <= num <= 10^15
*/

struct Solution;

impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 == 0 {
            let mid = num / 3;
            return vec![mid - 1, mid, mid + 1];
        }
        vec![]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_of_three(33), vec![10, 11, 12]);
    assert_eq!(Solution::sum_of_three(4), vec![]);
}
