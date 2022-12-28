#![allow(dead_code)]

// 2444. Count Subarrays With Fixed Bounds
// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/
// https://leetcode.cn/problems/count-subarrays-with-fixed-bounds/
//
// You are given an integer array nums and two integers minK and maxK.
//
// A fixed-bound subarray of nums is a subarray that satisfies the following conditions:
//
// The minimum value in the subarray is equal to minK.
// The maximum value in the subarray is equal to maxK.
//
// Return the number of fixed-bound subarrays.
//
// A subarray is a contiguous part of an array.
//
// Example 1:
//
// Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
// Output: 2
// Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
//
// Example 2:
//
// Input: nums = [1,1,1,1], minK = 1, maxK = 1
// Output: 10
// Explanation: Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
//
// Constraints:
//
// - 2 <= nums.length <= 10^5
// - 1 <= nums[i], minK, maxK <= 10^6
//

struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut rez = 0;

        for chunk in nums.split(|n| *n < min_k || *n > max_k) {
            let mut slow = 0;
            let (mut min_i_opt, mut max_i_opt) = (None, None);
            for fast in 0..chunk.len() {
                if chunk[fast] == min_k {
                    min_i_opt = Some(fast);
                }
                if chunk[fast] == max_k {
                    max_i_opt = Some(fast);
                }
                if let (Some(min_i), Some(max_i)) = (min_i_opt, max_i_opt) {
                    let (lo, hi) = (min_i.min(max_i), min_i.max(max_i));
                    rez += (lo - slow + 1) as u64 * (chunk.len() - hi) as u64;
                    if fast == min_i {
                        slow = max_i + 1;
                        max_i_opt = None;
                    } else {
                        slow = min_i + 1;
                        min_i_opt = None;
                    }
                }
            }
        }
        rez as _
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 5, 2, 7, 5];
    let min_k = 1;
    let max_k = 5;
    let rez = 2;
    assert_eq!(Solution::count_subarrays(nums, min_k, max_k), rez);

    let nums = vec![1, 1, 1, 1];
    let min_k = 1;
    let max_k = 1;
    let rez = 10;
    assert_eq!(Solution::count_subarrays(nums, min_k, max_k), rez);
}
