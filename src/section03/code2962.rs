#![allow(dead_code)]

// 2962. Count Subarrays Where Max Element Appears at Least K Times
// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
// https://leetcode.cn/problems/count-subarrays-where-max-element-appears-at-least-k-times/
//
// Medium
//
// You are given an integer array nums and a positive integer k.
//
// Return the number of subarrays where the maximum element of nums appears at least k times in that subarray.
//
// A subarray is a contiguous sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,3,2,3,3], k = 2
// Output: 6
// Explanation: The subarrays that contain the element 3 at least 2 times are:
// [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
//
// Example 2:
//
// Input: nums = [1,4,2,1], k = 3
// Output: 0
// Explanation: No subarray contains the element 4 at least 3 times.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^6
//     1 <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut max: i32 = 0;
        let mut cnt: i64 = 0;
        let mut idx: Vec<i32> = vec![];

        for val in nums.iter() {
            max = max.max(*val);
        }

        for (i, val) in nums.iter().enumerate() {
            if *val == max {
                idx.push(i as i32);
            }
        }

        if idx.len() < k as usize {
            return 0 as _;
        }

        let mut offset = 0;
        let r_len = (idx[idx.len() - 1] as usize).max(nums.len());

        for left in 0..nums.len() {
            if offset < idx.len() && left > idx[offset] as usize {
                offset += 1;
            }
            if offset as i32 + k - 1 > idx.len() as i32 - 1 {
                break;
            }
            let right = idx[offset + k as usize - 1] as usize;
            for _j in right..r_len {
                cnt += 1;
            }
        }
        cnt
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
    assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
    assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1, 4, 2, 1], 3), 0);
}
