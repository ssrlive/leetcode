#![allow(dead_code)]

// 560. Subarray Sum Equals K
// https://leetcode.com/problems/subarray-sum-equals-k/
//
// Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,1,1], k = 2
// Output: 2
//
// Example 2:
//
// Input: nums = [1,2,3], k = 3
// Output: 2
//
// Constraints:
//
// - 1 <= nums.length <= 2 * 10^4
// - -1000 <= nums[i] <= 1000
// - -10^7 <= k <= 10^7
//

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, 1);
        for num in nums {
            sum += num;
            if let Some(&c) = map.get(&(sum - k)) {
                count += c;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

#[test]
fn test_subarray_sum() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
}
