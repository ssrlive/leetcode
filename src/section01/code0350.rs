#![allow(dead_code)]

// 350. Intersection of Two Arrays II
// https://leetcode.com/problems/intersection-of-two-arrays-ii/
// https://leetcode.cn/problems/intersection-of-two-arrays-ii/
//
// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result
// must appear as many times as it shows in both arrays and you may return the result in any order.
//
// Example 1:
//
// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2,2]
//
// Example 2:
//
// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [4,9]
// Explanation: [9,4] is also accepted.
//
// Constraints:
//
// - 1 <= nums1.length, nums2.length <= 1000
// - 0 <= nums1[i], nums2[i] <= 1000
//
// Follow up:
//
// What if the given array is already sorted? How would you optimize your algorithm?
// What if nums1's size is small compared to nums2's size? Which algorithm is better?
// What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?
//

struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
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
                    result.push(nums1[i]);
                    i += 1;
                    j += 1;
                }
            }
        }

        result
    }
}

#[test]
fn test_intersect() {
    assert_eq!(Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    assert_eq!(Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
