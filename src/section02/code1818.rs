#![allow(dead_code)]

/*

// 1818. Minimum Absolute Sum Difference
// https://leetcode.com/problems/minimum-absolute-sum-difference/
// https://leetcode.cn/problems/minimum-absolute-sum-difference/
//
// Medium
//
// You are given two positive integer arrays nums1 and nums2, both of length n.

The absolute sum difference of arrays nums1 and nums2 is defined as the sum of |nums1[i] - nums2[i]| for each 0 <= i < n (0-indexed).

You can replace at most one element of nums1 with any other element in nums1 to minimize the absolute sum difference.

Return the minimum absolute sum difference after replacing at most one element in the array nums1. Since the answer may be large, return it modulo 109 + 7.

|x| is defined as:

    x if x >= 0, or
    -x if x < 0.

Example 1:

Input: nums1 = [1,7,5], nums2 = [2,3,5]
Output: 3
Explanation: There are two possible optimal solutions:
- Replace the second element with the first: [1,7,5] => [1,1,5], or
- Replace the second element with the third: [1,7,5] => [1,5,5].
Both will yield an absolute sum difference of |1-2| + (|1-3| or |5-3|) + |5-5| = 3.

Example 2:

Input: nums1 = [2,4,6,8,10], nums2 = [2,4,6,8,10]
Output: 0
Explanation: nums1 is equal to nums2 so no replacement is needed. This will result in an
absolute sum difference of 0.

Example 3:

Input: nums1 = [1,10,4,4,2,7], nums2 = [9,3,5,1,7,4]
Output: 20
Explanation: Replace the first element with the second: [1,10,4,4,2,7] => [10,10,4,4,2,7].
This yields an absolute sum difference of |10-9| + |10-3| + |4-5| + |4-1| + |2-7| + |7-4| = 20

Constraints:

    n == nums1.length
    n == nums2.length
    1 <= n <= 10^5
    1 <= nums1[i], nums2[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::cmp;
        let mut total: u64 = 0;
        let mut max_dec: u64 = 0;
        let mut sorted = nums1.clone();
        sorted.sort_unstable();
        for i in 0..nums1.len() {
            let diff = (nums1[i] - nums2[i]).abs();
            total += diff as u64;
            let pos = sorted.binary_search(&nums2[i]);
            let mut closest = 0;
            if let Ok(n) = pos {
                closest = n;
            } else if let Err(n) = pos {
                if n == 0 {
                    closest = n;
                } else if n == nums1.len() {
                    if (sorted[n - 1] - nums2[i]).abs() <= (sorted[n - 2] - nums2[i]).abs() {
                        closest = n - 1;
                    } else {
                        closest = n - 2;
                    }
                } else if (sorted[n] - nums2[i]).abs() <= (sorted[n - 1] - nums2[i]).abs() {
                    closest = n;
                } else {
                    closest = n - 1;
                }
            }
            max_dec = cmp::max(max_dec, (diff - (sorted[closest] - nums2[i]).abs()) as u64);
        }

        ((total - max_dec) % 1_000_000_007) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 7, 5], vec![2, 3, 5], 3),
        (vec![2, 4, 6, 8, 10], vec![2, 4, 6, 8, 10], 0),
        (vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4], 20),
    ];
    for (nums1, nums2, expected) in cases {
        assert_eq!(Solution::min_absolute_sum_diff(nums1, nums2), expected);
    }
}
