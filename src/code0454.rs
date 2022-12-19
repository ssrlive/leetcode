#![allow(dead_code)]

// 454. 4Sum II
// https://leetcode.com/problems/4sum-ii/
// https://leetcode.cn/problems/4sum-ii/
//
// Given four integer arrays nums1, nums2, nums3, and nums4 all of length n, return the number of tuples (i, j, k, l) such that:
//
// - 0 <= i, j, k, l < n
// - nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
//
// Example 1:
//
// Input: nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
// Output: 2
// Explanation:
// The two tuples are:
// 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
// 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0
//
// Example 2:
//
// Input: nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
// Output: 1
//
// Constraints:
//
// - n == nums1.length
// - n == nums2.length
// - n == nums3.length
// - n == nums4.length
// - 1 <= n <= 200
// - -228 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 228
//

struct Solution;

impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for item1 in &nums1 {
            for item2 in &nums2 {
                let sum = item1 + item2;
                *map.entry(sum).or_insert(0) += 1;
            }
        }
        let mut count = 0;
        for item3 in &nums3 {
            for item4 in &nums4 {
                let sum = item3 + item4;
                if let Some(v) = map.get(&-sum) {
                    count += v;
                }
            }
        }
        count
    }
}

#[test]
fn test_four_sum_count() {
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]), 1);
}
