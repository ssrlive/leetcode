#![allow(dead_code)]

// 2540. Minimum Common Value
// https://leetcode.com/problems/minimum-common-value/
// https://leetcode.cn/problems/minimum-common-value/
//
// Easy
//
// Given two integer arrays nums1 and nums2, sorted in non-decreasing order, return the minimum integer common to both arrays.
// If there is no common integer amongst nums1 and nums2, return -1.
//
// Note that an integer is said to be common to nums1 and nums2 if both arrays have at least one occurrence of that integer.
//
// Example 1:
//
// Input: nums1 = [1,2,3], nums2 = [2,4]
// Output: 2
// Explanation: The smallest element common to both arrays is 2, so we return 2.
//
// Example 2:
//
// Input: nums1 = [1,2,3,6], nums2 = [2,3,4,5]
// Output: 2
// Explanation: There are two common elements in the array 2 and 3 out of which 2 is the smallest, so 2 is returned.
//
// Constraints:
//
// -    1 <= nums1.length, nums2.length <= 10^5
// -    1 <= nums1[i], nums2[j] <= 10^9
// -    Both nums1 and nums2 are sorted in non-decreasing order.
//

struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut result = -1;
        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                std::cmp::Ordering::Equal => {
                    result = nums1[i];
                    break;
                }
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
    assert_eq!(Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
}
