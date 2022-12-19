#![allow(dead_code)]

// 321. Create Maximum Number
// https://leetcode.com/problems/create-maximum-number/
// https://leetcode.cn/problems/create-maximum-number/
//
// You are given two integer arrays nums1 and nums2 of lengths m and n respectively.
// nums1 and nums2 represent the digits of two numbers. You are also given an integer k.
//
// Create the maximum number of length k <= m + n from digits of the two numbers.
// The relative order of the digits from the same array must be preserved.
//
// Return an array of the k digits representing the answer.
//
// Example 1:
//
// Input: nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
// Output: [9,8,6,5,3]
//
// Example 2:
// Input: nums1 = [6,7], nums2 = [6,0,4], k = 5
// Output: [6,7,6,0,4]
//
// Example 3:
//
// Input: nums1 = [3,9], nums2 = [8,9], k = 3
// Output: [9,8,9]
//
// Constraints:
//
// - m == nums1.length
// - n == nums2.length
// - 1 <= m, n <= 500
// - 0 <= nums1[i], nums2[i] <= 9
// - 1 <= k <= m + n
//

struct Solution;

// use std::cmp::Ordering;

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        fn prep(nums: &[i32], k: usize) -> Vec<i32> {
            let mut drop = nums.len() - k;
            let mut out = Vec::with_capacity(k);
            for &num in nums {
                while drop > 0 && !out.is_empty() && out[out.len() - 1] < num {
                    out.pop();
                    drop -= 1;
                }
                out.push(num);
            }
            out.truncate(k);
            out
        }
        fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
            let mut out = Vec::with_capacity(a.len() + b.len());
            let mut a = a.to_vec();
            let mut b = b.to_vec();
            while !a.is_empty() || !b.is_empty() {
                out.push(match a.cmp(&b) {
                    std::cmp::Ordering::Less => b.remove(0),
                    _ => a.remove(0),
                });
            }
            out
        }
        (0..=k)
            .filter(|&i| i <= nums1.len() as i32 && k - i <= nums2.len() as i32)
            .map(|i| merge(&prep(&nums1, i as usize), &prep(&nums2, (k - i) as usize)))
            .max()
            .unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
        vec![9, 8, 6, 5, 3]
    );
    assert_eq!(Solution::max_number(vec![6, 7], vec![6, 0, 4], 5), vec![6, 7, 6, 0, 4]);
    assert_eq!(Solution::max_number(vec![3, 9], vec![8, 9], 3), vec![9, 8, 9]);
}
