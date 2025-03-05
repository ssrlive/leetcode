#![allow(dead_code)]

// 2455. Average Value of Even Numbers That Are Divisible by Three
// https://leetcode.com/problems/average-value-of-even-numbers-that-are-divisible-by-three/
// https://leetcode.cn/problems/average-value-of-even-numbers-that-are-divisible-by-three/
//
// Given an integer array nums of positive integers, return the average value of all even integers that are divisible by 3.
//
// Note that the average of n elements is the sum of the n elements divided by n and rounded down to the nearest integer.
//
// Example 1:
//
// Input: nums = [1,3,6,10,12,15]
// Output: 9
// Explanation: 6 and 12 are even numbers that are divisible by 3. (6 + 12) / 2 = 9.
//
// Example 2:
//
// Input: nums = [1,2,4,7,10]
// Output: 0
// Explanation: There is no single number that satisfies the requirement, so return 0.
//
// Constraints:
//
// - 1 <= nums.length <= 1000
// - 1 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        for num in nums {
            if num % 2 == 0 && num % 3 == 0 {
                sum += num;
                count += 1;
            }
        }
        if count == 0 { 0 } else { sum / count }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::average_value(vec![1, 3, 6, 10, 12, 15]), 9);
    assert_eq!(Solution::average_value(vec![1, 2, 4, 7, 10]), 0);
}
