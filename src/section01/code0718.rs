#![allow(dead_code)]

// 718. Maximum Length of Repeated Subarray
// https://leetcode.com/problems/maximum-length-of-repeated-subarray/
// https://leetcode.cn/problems/maximum-length-of-repeated-subarray/
//
// Given two integer arrays nums1 and nums2, return the maximum length of a subarray that appears in both arrays.
//
// Example 1:
//
// Input: nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
// Output: 3
// Explanation: The repeated subarray with maximum length is [3,2,1].
//
// Example 2:
//
// Input: nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
// Output: 5
// Explanation: The repeated subarray with maximum length is [0,0,0,0,0].
//
// Constraints:
//
// - 1 <= nums1.length, nums2.length <= 1000
// - 0 <= nums1[i], nums2[i] <= 100
//

struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (n1, n2) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; n2 + 1]; n1 + 1];
        let mut max = 0;

        for i in 1..=n1 {
            for j in 1..=n2 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    max = max.max(dp[i][j]);
                }
            }
        }

        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]), 3);
    assert_eq!(Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]), 5);
}
