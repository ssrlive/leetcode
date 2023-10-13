#![allow(dead_code)]

// 1250. Check If It Is a Good Array
// https://leetcode.com/problems/check-if-it-is-a-good-array/
// https://leetcode.cn/problems/check-if-it-is-a-good-array/
//
// Hard
// Given an array nums of positive integers. Your task is to select some subset of nums, multiply each element by an integer and add all these numbers. The array is said to be good if you can obtain a sum of 1 from the array by any possible subset and multiplicand.
//
// Return True if the array is good otherwise return False.
//
// Example 1:
//
// Input: nums = [12,5,7,23]
// Output: true
// Explanation: Pick numbers 5 and 7.
// 5*3 + 7*(-2) = 1
//
// Example 2:
//
// Input: nums = [29,6,10]
// Output: true
// Explanation: Pick numbers 29, 6 and 10.
// 29*1 + 6*(-3) + 10*(-1) = 1
//
// Example 3:
//
// Input: nums = [3,6]
// Output: false
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let mut res = nums[0];
        for i in nums.iter() {
            res = gcd(res, *i);
        }
        res == 1
    }
}

#[test]
fn test() {
    assert!(Solution::is_good_array(vec![12, 5, 7, 23]));
    assert!(Solution::is_good_array(vec![29, 6, 10]));
    assert!(!Solution::is_good_array(vec![3, 6]));
}
