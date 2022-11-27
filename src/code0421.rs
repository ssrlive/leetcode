#![allow(dead_code)]

// 421. Maximum XOR of Two Numbers in an Array
// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
//
// Given an integer array nums, return the maximum result of nums[i] XOR nums[j], where 0 <= i <= j < n.
//
// Example 1:
//
// Input: nums = [3,10,5,25,2,8]
// Output: 28
// Explanation: The maximum result is 5 XOR 25 = 28.
//
// Example 2:
//
// Input: nums = [14,70,53,83,49,91,36,80,92,51,66,70]
// Output: 127
//
// Constraints:
//
// - 1 <= nums.length <= 2 * 105
// - 0 <= nums[i] <= 231 - 1
//

struct Solution;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut mask = 0;
        for i in (0..32).rev() {
            mask |= 1 << i;
            let mut set = std::collections::HashSet::new();
            for num in &nums {
                set.insert(num & mask);
            }
            let tmp = max | (1 << i);
            for prefix in &set {
                if set.contains(&(tmp ^ prefix)) {
                    max = tmp;
                    break;
                }
            }
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
}
