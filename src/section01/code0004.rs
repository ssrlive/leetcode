#![allow(dead_code)]

// 4. Median of Two Sorted Arrays
// https://leetcode.com/problems/median-of-two-sorted-arrays/
// https://leetcode.cn/problems/median-of-two-sorted-arrays/
//
// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)).
//
// Example 1:
//
// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
//
// Example 2:
//
// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
// Constraints:
//
// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -10^6 <= nums1[i], nums2[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged_array_len = nums1.len() + nums2.len();
        let (mut arr1_index, mut arr2_index) = (0, 0);
        let target_index = merged_array_len / 2;
        let (mut last, mut one_before) = (0, 0);

        for _ in 0..=target_index {
            one_before = last;
            if arr1_index == nums1.len() {
                last = nums2[arr2_index];
                arr2_index += 1;
            } else if arr2_index == nums2.len() || nums1[arr1_index] < nums2[arr2_index] {
                last = nums1[arr1_index];
                arr1_index += 1;
            } else {
                last = nums2[arr2_index];
                arr2_index += 1;
            }
        }
        if merged_array_len % 2 == 0 {
            (last + one_before) as f64 / 2.0
        } else {
            last as f64
        }
    }

    pub fn find_median_sorted_arrays_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = Vec::with_capacity(nums1.len() + nums2.len());
        nums.extend(nums1);
        nums.extend(nums2);
        nums.sort_unstable();
        let n = nums.len();
        if n % 2 == 0 {
            (nums[n / 2 - 1] + nums[n / 2]) as f64 / 2.0
        } else {
            nums[n / 2] as f64
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
}
