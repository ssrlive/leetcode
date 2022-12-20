#![allow(dead_code)]

// 2461. Maximum Sum of Distinct Subarrays With Length K
// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
// https://leetcode.cn/problems/maximum-sum-of-distinct-subarrays-with-length-k/
//
// You are given an integer array nums and an integer k. Find the maximum subarray
// sum of all the subarrays of nums that meet the following conditions:
//
// - The length of the subarray is k, and
// - All the elements of the subarray are distinct.
//
// Return the maximum subarray sum of all the subarrays that meet the conditions.
// If no subarray meets the conditions, return 0.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,5,4,2,9,9,9], k = 3
// Output: 15
// Explanation: The subarrays of nums with length 3 are:
// - [1,5,4] which meets the requirements and has a sum of 10.
// - [5,4,2] which meets the requirements and has a sum of 11.
// - [4,2,9] which meets the requirements and has a sum of 15.
// - [2,9,9] which does not meet the requirements because the element 9 is repeated.
// - [9,9,9] which does not meet the requirements because the element 9 is repeated.
// We return 15 because it is the maximum subarray sum of all the subarrays that meet the conditions
//
// Example 2:
//
// Input: nums = [4,4,4], k = 3
// Output: 0
// Explanation: The subarrays of nums with length 3 are:
// - [4,4,4] which does not meet the requirements because the element 4 is repeated.
// We return 0 because no subarrays meet the conditions.
//
// Constraints:
//
// - 1 <= k <= nums.length <= 10^5
// - 1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut sum = 0;
        let mut max = 0;
        let mut set = std::collections::HashSet::new();
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if set.insert(nums[j]) {
                sum += nums[j] as i64;
                if set.len() == k as usize {
                    max = std::cmp::max(max, sum);
                    set.remove(&nums[i]);
                    sum -= nums[i] as i64;
                    i += 1;
                }
            } else {
                while nums[i] != nums[j] {
                    set.remove(&nums[i]);
                    sum -= nums[i] as i64;
                    i += 1;
                }
                i += 1;
            }
            j += 1;
        }
        max
    }
}

#[test]
fn test() {
    let nums = vec![1, 5, 4, 2, 9, 9, 9];
    assert_eq!(Solution::maximum_subarray_sum(nums, 3), 15);

    assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0);
}
