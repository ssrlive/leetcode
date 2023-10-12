#![allow(dead_code)]

/*

// 2605. Form Smallest Number From Two Digit Arrays
// https://leetcode.com/problems/form-smallest-number-from-two-digit-arrays/
// https://leetcode.cn/problems/form-smallest-number-from-two-digit-arrays/
//
// Easy
//
// Given two arrays of unique digits nums1 and nums2, return the smallest number that contains at least one digit from each array.

Example 1:

Input: nums1 = [4,1,3], nums2 = [5,7]
Output: 15
Explanation: The number 15 contains the digit 1 from nums1 and the digit 5 from nums2. It can be proven that 15 is the smallest number we can have.

Example 2:

Input: nums1 = [3,5,2,6], nums2 = [3,1,7]
Output: 3
Explanation: The number 3 contains the digit 3 which exists in both arrays.

Constraints:

    1 <= nums1.length, nums2.length <= 9
    1 <= nums1[i], nums2[i] <= 9
    All digits in each array are unique.
*/

struct Solution;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut freq = [0; 10];
        for i in 0..nums1.len() {
            freq[nums1[i] as usize] += 1;
        }
        for i in 0..nums2.len() {
            freq[nums2[i] as usize] += 1;
        }

        let mn1 = nums1.iter().min().unwrap();
        let mn2 = nums2.iter().min().unwrap();
        for i in 1..=9 {
            if freq[i as usize] == 2 {
                return i;
            }
        }

        if mn2 < mn1 {
            return mn2 * 10 + mn1;
        }
        mn1 * 10 + mn2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_number(vec![4, 1, 3], vec![5, 7]), 15);
    assert_eq!(Solution::min_number(vec![3, 5, 2, 6], vec![3, 1, 7]), 3);
}
