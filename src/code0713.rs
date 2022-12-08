#![allow(dead_code)]

// 713. Subarray Product Less Than K
// https://leetcode.com/problems/subarray-product-less-than-k/
//
// Given an array of integers nums and an integer k, return the number of contiguous subarrays
// where the product of all the elements in the subarray is strictly less than k.
//
// Example 1:
//
// Input: nums = [10,5,2,6], k = 100
// Output: 8
// Explanation: The 8 subarrays that have product less than 100 are:
// [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
// Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
//
// Example 2:
//
// Input: nums = [1,2,3], k = 0
// Output: 0
//
// Constraints:
//
// - 1 <= nums.length <= 3 * 104
// - 1 <= nums[i] <= 1000
// - 0 <= k <= 106
//

struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let n: usize = nums.len();
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut cur: i32 = 1;
        let mut ans: i32 = 0;
        while right < n {
            if nums[right] >= k {
                cur = 1;
                right += 1;
                left = right;
                continue;
            }
            cur *= nums[right];
            while cur >= k {
                cur /= nums[left];
                left += 1;
            }
            ans += (right - left + 1) as i32;
            right += 1;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
    assert_eq!(Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
}
