#![allow(dead_code)]

/*

// 2032. Two Out of Three
// https://leetcode.com/problems/two-out-of-three/
// https://leetcode.cn/problems/two-out-of-three/
//
// Easy
//
// Given three integer arrays nums1, nums2, and nums3, return a distinct array containing all the values that are present in at least two out of the three arrays. You may return the values in any order.

Example 1:

Input: nums1 = [1,1,3,2], nums2 = [2,3], nums3 = [3]
Output: [3,2]
Explanation: The values that are present in at least two arrays are:
- 3, in all three arrays.
- 2, in nums1 and nums2.

Example 2:

Input: nums1 = [3,1], nums2 = [2,3], nums3 = [1,2]
Output: [2,3,1]
Explanation: The values that are present in at least two arrays are:
- 2, in nums2 and nums3.
- 3, in nums1 and nums2.
- 1, in nums1 and nums3.

Example 3:

Input: nums1 = [1,2,2], nums2 = [4,3,3], nums3 = [5]
Output: []
Explanation: No value is present in at least two arrays.

Constraints:

    1 <= nums1.length, nums2.length, nums3.length <= 100
    1 <= nums1[i], nums2[j], nums3[k] <= 100
*/

struct Solution;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut counter = [0_i32; 101];
        nums1.iter().for_each(|&x| counter[x as usize] |= 0b001);
        nums2.iter().for_each(|&x| counter[x as usize] |= 0b010);
        nums3.iter().for_each(|&x| counter[x as usize] |= 0b100);
        counter
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(ind, x)| match x.count_ones() >= 2 {
                true => Some(ind as i32),
                false => None,
            })
            .collect::<Vec<_>>()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1, 3, 2], vec![2, 3], vec![3], vec![2, 3]),
        (vec![3, 1], vec![2, 3], vec![1, 2], vec![1, 2, 3]),
        (vec![1, 2, 2], vec![4, 3, 3], vec![5], vec![]),
    ];
    for (nums1, nums2, nums3, expected) in cases {
        assert_eq!(Solution::two_out_of_three(nums1, nums2, nums3), expected);
    }
}
