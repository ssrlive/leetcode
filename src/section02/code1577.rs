#![allow(dead_code)]

/*

// 1577. Number of Ways Where Square of Number Is Equal to Product of Two Numbers
Medium
314
52
Companies

Given two arrays of integers nums1 and nums2, return the number of triplets formed (type 1 and type 2) under the following rules:

    Type 1: Triplet (i, j, k) if nums1[i]2 == nums2[j] * nums2[k] where 0 <= i < nums1.length and 0 <= j < k < nums2.length.
    Type 2: Triplet (i, j, k) if nums2[i]2 == nums1[j] * nums1[k] where 0 <= i < nums2.length and 0 <= j < k < nums1.length.

Example 1:

Input: nums1 = [7,4], nums2 = [5,2,8,9]
Output: 1
Explanation: Type 1: (1, 1, 2), nums1[1]2 = nums2[1] * nums2[2]. (42 = 2 * 8).

Example 2:

Input: nums1 = [1,1], nums2 = [1,1,1]
Output: 9
Explanation: All Triplets are valid, because 12 = 1 * 1.
Type 1: (0,0,1), (0,0,2), (0,1,2), (1,0,1), (1,0,2), (1,1,2).  nums1[i]2 = nums2[j] * nums2[k].
Type 2: (0,0,1), (1,0,1), (2,0,1). nums2[i]2 = nums1[j] * nums1[k].

Example 3:

Input: nums1 = [7,7,8,3], nums2 = [1,2,9,7]
Output: 2
Explanation: There are 2 valid triplets.
Type 1: (3,0,2).  nums1[3]2 = nums2[0] * nums2[2].
Type 2: (3,0,1).  nums2[3]2 = nums1[0] * nums1[1].

Constraints:

    1 <= nums1.length, nums2.length <= 1000
    1 <= nums1[i], nums2[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut ans = 0;
        let (mut m1, mut m2) = (HashMap::new(), HashMap::new());
        for i in 0..nums1.len() {
            for j in i + 1..nums1.len() {
                let p = nums1[i] as i64 * nums1[j] as i64;
                *m1.entry(p).or_insert(0) += 1;
            }
        }
        for i in 0..nums2.len() {
            for j in i + 1..nums2.len() {
                let p = nums2[i] as i64 * nums2[j] as i64;
                *m2.entry(p).or_insert(0) += 1;
            }
        }
        for &num in nums1.iter() {
            if let Some(&v) = m2.get(&(num as i64 * num as i64)) {
                ans += v;
            }
        }
        for &num in nums2.iter() {
            if let Some(&v) = m1.get(&(num as i64 * num as i64)) {
                ans += v;
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
    assert_eq!(Solution::num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
    assert_eq!(Solution::num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]), 2);
}
