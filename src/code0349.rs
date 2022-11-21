#![allow(dead_code)]

// 349. Intersection of Two Arrays
// https://leetcode.com/problems/intersection-of-two-arrays/
//
// Given two integer arrays nums1 and nums2, return an array of their intersection.
// Each element in the result must be unique and you may return the result in any order.
//
// Example 1:
//
// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2]
//
// Example 2:
//
// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [9,4]
// Explanation: [4,9] is also accepted.
//
// Constraints:
//
// 1 <= nums1.length, nums2.length <= 1000
// 0 <= nums1[i], nums2[i] <= 1000

struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut nums1 = nums1;
        let mut nums2 = nums2;

        nums1.sort();
        nums2.sort();

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => {
                    if result.is_empty() || result[result.len() - 1] != nums1[i] {
                        result.push(nums1[i]);
                    }
                    i += 1;
                    j += 1;
                }
            }
        }

        result
    }
}

#[test]
fn test_intersection() {
    assert_eq!(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
    assert_eq!(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
