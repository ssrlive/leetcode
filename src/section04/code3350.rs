#![allow(dead_code)]

// 3350. Adjacent Increasing Subarrays Detection II
// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/
// https://leetcode.cn/problems/adjacent-increasing-subarrays-detection-ii/
//
// Medium
//
// Given an array nums of n integers, your task is to find the maximum value of k for which there exist two adjacent subarrays
// of length k each, such that both subarrays are strictly increasing. Specifically,
// check if there are two subarrays of length k starting at indices a and b (a < b), where:
//
//     Both subarrays nums[a..a + k - 1] and nums[b..b + k - 1] are strictly increasing.
//     The subarrays must be adjacent, meaning b = a + k.
//
// Return the maximum possible value of k.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [2,5,7,8,9,2,3,4,3,1]
//
// Output: 3
//
// Explanation:
//
//     The subarray starting at index 2 is [7, 8, 9], which is strictly increasing.
//     The subarray starting at index 5 is [2, 3, 4], which is also strictly increasing.
//     These two subarrays are adjacent, and 3 is the maximum possible value of k for which two such adjacent strictly increasing subarrays exist.
//
// Example 2:
//
// Input: nums = [1,2,3,4,4,4,4,5,6,7]
//
// Output: 2
//
// Explanation:
//
//     The subarray starting at index 0 is [1, 2], which is strictly increasing.
//     The subarray starting at index 2 is [3, 4], which is also strictly increasing.
//     These two subarrays are adjacent, and 2 is the maximum possible value of k for which two such adjacent strictly increasing subarrays exist.
//
// Constraints:
//
//     2 <= nums.length <= 2 * 10^5
//     -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        fn monoton(xs: &[i32]) -> usize {
            let mut i = 0;
            while i + 1 < xs.len() && xs[i] < xs[i + 1] {
                i += 1
            }
            i + 1
        }

        let mut i = 0;
        let mut best = 1;
        let mut l = 1;
        while i < nums.len() - 1 {
            let l1 = l;
            l = monoton(&nums[i..]);
            i += l;
            best = best.max((l / 2).max(l1.min(l)));
        }
        best as _
    }
}

#[test]
fn test() {
    let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
    let res = 3;
    assert_eq!(Solution::max_increasing_subarrays(nums), res);

    let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
    let res = 2;
    assert_eq!(Solution::max_increasing_subarrays(nums), res);
}
