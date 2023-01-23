#![allow(dead_code)]

// 1283. Find the Smallest Divisor Given a Threshold
// https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/
// https://leetcode.cn/problems/find-the-smallest-divisor-given-a-threshold/
//
// Medium
//
// Given an array of integers nums and an integer threshold, we will choose a positive integer divisor,
// divide all the array by it, and sum the division's result. Find the smallest divisor such that the result
// mentioned above is less than or equal to threshold.
//
// Each result of the division is rounded to the nearest integer greater than or equal to that element. (For example: 7/3 = 3 and 10/2 = 5).
//
// The test cases are generated so that there will be an answer.
//
// Example 1:
//
// Input: nums = [1,2,5,9], threshold = 6
// Output: 5
// Explanation: We can get a sum to 17 (1+2+5+9) if the divisor is 1.
// If the divisor is 4 we can get a sum of 7 (1+1+2+3) and if the divisor is 5 the sum will be 5 (1+1+1+2).
//
// Example 2:
//
// Input: nums = [44,22,33,11,1], threshold = 5
// Output: 44
//
// Constraints:
//
// -    1 <= nums.length <= 5 * 10^4
// -    1 <= nums[i] <= 10^6
// -    nums.length <= threshold <= 10^6
//

struct Solution;

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000;
        while left < right {
            let mid = left + (right - left) / 2;
            let sum = nums.iter().map(|&n| (n + mid - 1) / mid).sum::<i32>();
            if sum > threshold {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_divisor(vec![1, 2, 5, 9], 6), 5);
    assert_eq!(Solution::smallest_divisor(vec![44, 22, 33, 11, 1], 5), 44);
}
