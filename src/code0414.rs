#![allow(dead_code)]

// 414. Third Maximum Number
// https://leetcode.com/problems/third-maximum-number/
//
// Given integer array nums, return the third maximum number in this array. If the
// third maximum does not exist, return the maximum number.
//
// Example 1:
//
// Input: nums = [3,2,1]
// Output: 1
// Explanation: The third maximum is 1.
//
// Example 2:
//
// Input: nums = [1,2]
// Output: 2
// Explanation: The third maximum does not exist, so the maximum (2) is returned instead.
//
// Example 3:
//
// Input: nums = [2,2,3,1]
// Output: 1
// Explanation: Note that the third maximum here means the third maximum distinct number.
// Both numbers with value 2 are both considered as second maximum.
//
// Constraints:
//
// - 1 <= nums.length <= 104
// - -231 <= nums[i] <= 231 - 1
//
// Follow up: Can you find an O(n) solution?

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        pub fn _third_max(nums: Vec<i64>) -> i64 {
            let mut max1 = std::i64::MIN;
            let mut max2 = std::i64::MIN;
            let mut max3 = std::i64::MIN;
            for &num in nums.iter() {
                if num > max1 {
                    max3 = max2;
                    max2 = max1;
                    max1 = num;
                } else if num > max2 && num < max1 {
                    max3 = max2;
                    max2 = num;
                } else if num > max3 && num < max2 {
                    max3 = num;
                }
            }
            if max3 == std::i64::MIN {
                max1
            } else {
                max3
            }
        }
        _third_max(nums.into_iter().map(|x| x as i64).collect()) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    assert_eq!(Solution::third_max(vec![1, 2]), 2);
    assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    assert_eq!(Solution::third_max(vec![1, 2, -2147483648]), -2147483648);
}
