#![allow(dead_code)]

/*

// 1855. Maximum Distance Between a Pair of Values
// https://leetcode.com/problems/maximum-distance-between-a-pair-of-values/
// https://leetcode.cn/problems/maximum-distance-between-a-pair-of-values/
//
// Medium
//
// You are given two non-increasing 0-indexed integer arrays nums1​​​​​​ and nums2​​​​​​.

A pair of indices (i, j), where 0 <= i < nums1.length and 0 <= j < nums2.length, is valid if both i <= j and nums1[i] <= nums2[j]. The distance of the pair is j - i​​​​.

Return the maximum distance of any valid pair (i, j). If there are no valid pairs, return 0.

An array arr is non-increasing if arr[i-1] >= arr[i] for every 1 <= i < arr.length.

Example 1:

Input: nums1 = [55,30,5,4,2], nums2 = [100,20,10,10,5]
Output: 2
Explanation: The valid pairs are (0,0), (2,2), (2,3), (2,4), (3,3), (3,4), and (4,4).
The maximum distance is 2 with pair (2,4).

Example 2:

Input: nums1 = [2,2,2], nums2 = [10,10,1]
Output: 1
Explanation: The valid pairs are (0,0), (0,1), and (1,1).
The maximum distance is 1 with pair (0,1).

Example 3:

Input: nums1 = [30,29,19,5], nums2 = [25,25,25,25,25]
Output: 2
Explanation: The valid pairs are (2,2), (2,3), (2,4), (3,3), and (3,4).
The maximum distance is 2 with pair (2,4).

Constraints:

    1 <= nums1.length, nums2.length <= 10^5
    1 <= nums1[i], nums2[j] <= 10^5
    Both nums1 and nums2 are non-increasing.
*/

struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j, mut res) = (0_i32, 0_i32, 0_i32);
        while (i as usize) < nums1.len() && (j as usize) < nums2.len() {
            if nums1[i as usize] > nums2[j as usize] {
                i += 1;
            } else {
                res = res.max(j - i);
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5], 2),
        (vec![2, 2, 2], vec![10, 10, 1], 1),
        (vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25], 2),
    ];
    for (nums1, nums2, expected) in cases {
        assert_eq!(Solution::max_distance(nums1, nums2), expected);
    }
}
