#![allow(dead_code)]

// 719. Find K-th Smallest Pair Distance
// https://leetcode.com/problems/find-k-th-smallest-pair-distance/
//
// The distance of a pair of integers a and b is defined as the absolute difference between a and b.
//
// Given an integer array nums and an integer k, return the kth smallest distance among
// all the pairs nums[i] and nums[j] where 0 <= i < j < nums.length.
//
// Example 1:
//
// Input: nums = [1,3,1], k = 1
// Output: 0
// Explanation: Here are all the pairs:
// (1,3) -> 2
// (1,1) -> 0
// (3,1) -> 2
// Then the 1st smallest distance pair is (1,1), and its distance is 0.
//
// Example 2:
//
// Input: nums = [1,1,1], k = 2
// Output: 0
//
// Example 3:
//
// Input: nums = [1,6,1], k = 3
// Output: 5
//
// Constraints:
//
// n == nums.length
// 2 <= n <= 10^4
// 0 <= nums[i] <= 10^6
// 1 <= k <= n * (n - 1) / 2
//

struct Solution;

impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let (n, k) = (nums.len(), k as usize);
        let mut lo = 0;
        let mut hi = nums[n - 1] - nums[0];

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let mut count = 0;
            let mut left = 0;

            for right in 0..n {
                while nums[right] - nums[left] > mid {
                    left += 1;
                }
                count += right - left;
            }

            if count < k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
}
