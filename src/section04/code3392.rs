#![allow(dead_code)]

// 3392. Count Subarrays of Length Three With a Condition
// https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/
// https://leetcode.cn/problems/count-subarrays-of-length-three-with-a-condition/
//
// Easy
//
// Given an integer array nums, return the number of subarrays of length 3 such that the sum of the first and third numbers equals exactly half of the second number.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,2,1,4,1]
//
// Output: 1
//
// Explanation:
//
// Only the subarray [1,4,1] contains exactly 3 elements where the sum of the first and third numbers equals half the middle number.
//
// Example 2:
//
// Input: nums = [1,1,1]
//
// Output: 0
//
// Explanation:
//
// [1,1,1] is the only subarray of length 3. However, its first and third numbers do not add to half the middle number.
//
// Constraints:
//
// 3 <= nums.length <= 100
// -100 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        for i in 0..n - 2 {
            if (nums[i] + nums[i + 2]) * 2 == nums[i + 1] {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 4, 1];
    let res = 1;
    assert_eq!(Solution::count_subarrays(nums), res);

    let nums = vec![1, 1, 1];
    let res = 0;
    assert_eq!(Solution::count_subarrays(nums), res);
}
