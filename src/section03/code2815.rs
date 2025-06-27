#![allow(dead_code)]

// 2815. Max Pair Sum in an Array
// https://leetcode.com/problems/max-pair-sum-in-an-array/
// https://leetcode.cn/problems/max-pair-sum-in-an-array/
//
// Easy
//
// You are given a 0-indexed integer array nums. You have to find the maximum sum of a pair of numbers
// from nums such that the maximum digit in both numbers are equal.
//
// Return the maximum sum or -1 if no such pair exists.
//
// Example 1:
//
// Input: nums = [51,71,17,24,42]
// Output: 88
// Explanation:
// For i = 1 and j = 2, nums[i] and nums[j] have equal maximum digits with a pair sum of 71 + 17 = 88.
// For i = 3 and j = 4, nums[i] and nums[j] have equal maximum digits with a pair sum of 24 + 42 = 66.
// It can be shown that there are no other pairs with equal maximum digits, so the answer is 88.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: -1
// Explanation: No pair exists in nums with equal maximum digits.
//
// Constraints:
//
// 2 <= nums.length <= 100
// 1 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for &num in nums.iter() {
            let max_digit = format!("{num}").chars().fold(0, |max_digit, n| {
                let digit = n.to_digit(10).unwrap();
                max_digit.max(digit)
            });
            map.entry(max_digit).and_modify(|x: &mut Vec<i32>| x.push(num)).or_insert(vec![num]);
        }
        let mut max = -1;
        for mut row in map.into_values() {
            if row.len() < 2 {
                continue;
            }
            row.sort_by(|a, b| b.cmp(a));
            max = max.max(row[0] + row[1]);
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sum(vec![51, 71, 17, 24, 42]), 88);
    assert_eq!(Solution::max_sum(vec![1, 2, 3, 4]), -1);
}
