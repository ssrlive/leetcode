#![allow(dead_code)]

/*

// 2215. Find the Difference of Two Arrays
// https://leetcode.com/problems/find-the-difference-of-two-arrays/
// https://leetcode.cn/problems/find-the-difference-of-two-arrays/
//
// Easy
//
// Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:

    answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
    answer[1] is a list of all distinct integers in nums2 which are not present in nums1.

Note that the integers in the lists may be returned in any order.

Example 1:

Input: nums1 = [1,2,3], nums2 = [2,4,6]
Output: [[1,3],[4,6]]
Explanation:
For nums1, nums1[1] = 2 is present at index 0 of nums2, whereas nums1[0] = 1 and nums1[2] = 3 are not present in nums2. Therefore, answer[0] = [1,3].
For nums2, nums2[0] = 2 is present at index 1 of nums1, whereas nums2[1] = 4 and nums2[2] = 6 are not present in nums2. Therefore, answer[1] = [4,6].

Example 2:

Input: nums1 = [1,2,3,3], nums2 = [1,1,2,2]
Output: [[3],[]]
Explanation:
For nums1, nums1[2] and nums1[3] are not present in nums2. Since nums1[2] == nums1[3], their value is only included once and answer[0] = [3].
Every integer in nums2 is present in nums1. Therefore, answer[1] = [].

Constraints:

    1 <= nums1.length, nums2.length <= 1000
    -1000 <= nums1[i], nums2[i] <= 1000
*/

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let idx = |x: i32| -> usize { (x + 1000) as _ };
        let mut counter = [0_u8; 2001];
        nums1.iter().for_each(|&x| counter[idx(x)] |= 0b01);
        nums2.iter().for_each(|&x| counter[idx(x)] |= 0b10);

        let mut res = vec![Vec::new(), Vec::new()];
        counter.iter().enumerate().for_each(|(i, &x)| match x {
            0b01 => res[0].push(i as i32 - 1000),
            0b10 => res[1].push(i as i32 - 1000),
            _ => (),
        });
        res
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    let res = vec![vec![1, 3], vec![4, 6]];
    assert_eq!(Solution::find_difference(nums1, nums2), res);

    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 1, 2, 2];
    let res = vec![vec![3], vec![]];
    assert_eq!(Solution::find_difference(nums1, nums2), res);
}
