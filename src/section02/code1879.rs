#![allow(dead_code)]

/*

// 1879. Minimum XOR Sum of Two Arrays
// https://leetcode.com/problems/minimum-xor-sum-of-two-arrays/
// https://leetcode.cn/problems/minimum-xor-sum-of-two-arrays/
//
// Hard
//
// You are given two integer arrays nums1 and nums2 of length n.

The XOR sum of the two integer arrays is (nums1[0] XOR nums2[0]) + (nums1[1] XOR nums2[1]) + ... + (nums1[n - 1] XOR nums2[n - 1]) (0-indexed).

    For example, the XOR sum of [1,2,3] and [3,2,1] is equal to (1 XOR 3) + (2 XOR 2) + (3 XOR 1) = 2 + 0 + 2 = 4.

Rearrange the elements of nums2 such that the resulting XOR sum is minimized.

Return the XOR sum after the rearrangement.

Example 1:

Input: nums1 = [1,2], nums2 = [2,3]
Output: 2
Explanation: Rearrange nums2 so that it becomes [3,2].
The XOR sum is (1 XOR 3) + (2 XOR 2) = 2 + 0 = 2.

Example 2:

Input: nums1 = [1,0,3], nums2 = [5,3,4]
Output: 8
Explanation: Rearrange nums2 so that it becomes [5,4,3].
The XOR sum is (1 XOR 5) + (0 XOR 4) + (3 XOR 3) = 4 + 4 + 0 = 8.

Constraints:

    n == nums1.length
    n == nums2.length
    1 <= n <= 14
    0 <= nums1[i], nums2[i] <= 10^7
*/

struct Solution;

impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::*;
        let n = nums1.len();
        let limit = 1 << n;
        let mut map = HashMap::new();
        map.insert(0, 0);
        for &nums1_i in nums1.iter() {
            let mut new_map = HashMap::new();
            let v1 = nums1_i;
            for (j, &nums2_j) in nums2.iter().enumerate().take(n) {
                let v2 = nums2_j;

                for (key, val) in &map {
                    let new_key = key | (1 << j);
                    let entry = new_map.entry(new_key).or_insert(i32::max_value());
                    *entry = std::cmp::min(*entry, val + (v1 ^ v2));
                }
            }
            map = new_map;
        }
        *map.get(&(limit - 1)).unwrap()
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 2];
    let nums2 = vec![2, 3];
    assert_eq!(Solution::minimum_xor_sum(nums1, nums2), 2);

    let nums1 = vec![1, 0, 3];
    let nums2 = vec![5, 3, 4];
    assert_eq!(Solution::minimum_xor_sum(nums1, nums2), 8);
}
