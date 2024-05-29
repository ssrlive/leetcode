#![allow(dead_code)]

// 3162. Find the Number of Good Pairs I
// https://leetcode.com/problems/find-the-number-of-good-pairs-i/
// https://leetcode.cn/problems/find-the-number-of-good-pairs-i/
//
// Easy
//
// You are given 2 integer arrays nums1 and nums2 of lengths n and m respectively. You are also given a positive integer k.
//
// A pair (i, j) is called good if nums1[i] is divisible by nums2[j] * k (0 <= i <= n - 1, 0 <= j <= m - 1).
//
// Return the total number of good pairs.
//
// Example 1:
//
// Input: nums1 = [1,3,4], nums2 = [1,3,4], k = 1
//
// Output: 5
//
// Explanation:
//
// The 5 good pairs are (0, 0), (1, 0), (1, 1), (2, 0), and (2, 2).
//
// Example 2:
//
// Input: nums1 = [1,2,4,12], nums2 = [2,4], k = 3
//
// Output: 2
//
// Explanation:
//
// The 2 good pairs are (3, 0) and (3, 1).
//
// Constraints:
//
// 1 <= n, m <= 50
// 1 <= nums1[i], nums2[j] <= 50
// 1 <= k <= 50
//

struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        nums1
            .into_iter()
            .map(|x| nums2.iter().filter(|&y| x % (y * k) == 0).count() as i32)
            .sum::<i32>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
    assert_eq!(Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
}
