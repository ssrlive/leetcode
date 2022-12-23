#![allow(dead_code)]

// 41. First Missing Positive
// https://leetcode.com/problems/first-missing-positive/
// https://leetcode.cn/problems/first-missing-positive/
//
// Given an unsorted integer array nums, return the smallest missing positive integer.
//
// You must implement an algorithm that runs in O(n) time and uses constant extra space.
//
// Example 1:
//
// Input: nums = [1,2,0]
// Output: 3
// Explanation: The numbers in the range [1,2] are all in the array.
//
// Example 2:
//
// Input: nums = [3,4,-1,1]
// Output: 2
// Explanation: 1 is in the array but 2 is missing.
//
// Example 3:
//
// Input: nums = [7,8,9,11,12]
// Output: 1
// Explanation: The smallest positive integer 1 is missing.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - -2^31 <= nums[i] <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums_copy = nums.to_vec();
        nums_copy.sort_unstable();

        let f = |miss: i32, num: &i32| if miss == *num { miss + 1 } else { miss };
        nums_copy.iter().fold(1, f)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}
