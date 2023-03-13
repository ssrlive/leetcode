#![allow(dead_code)]

/*

// 2261. K Divisible Elements Subarrays
// https://leetcode.com/problems/k-divisible-elements-subarrays/
// https://leetcode.cn/problems/k-divisible-elements-subarrays/
//
// Medium
//
// Given an integer array nums and two integers k and p, return the number of distinct subarrays which have at most k elements divisible by p.

Two arrays nums1 and nums2 are said to be distinct if:

    They are of different lengths, or
    There exists at least one index i where nums1[i] != nums2[i].

A subarray is defined as a non-empty contiguous sequence of elements in an array.

Example 1:

Input: nums = [2,3,3,2,2], k = 2, p = 2
Output: 11
Explanation:
The elements at indices 0, 3, and 4 are divisible by p = 2.
The 11 distinct subarrays which have at most k = 2 elements divisible by 2 are:
[2], [2,3], [2,3,3], [2,3,3,2], [3], [3,3], [3,3,2], [3,3,2,2], [3,2], [3,2,2], and [2,2].
Note that the subarrays [2] and [3] occur more than once in nums, but they should each be counted only once.
The subarray [2,3,3,2,2] should not be counted because it has 3 elements that are divisible by 2.

Example 2:

Input: nums = [1,2,3,4], k = 4, p = 1
Output: 10
Explanation:
All element of nums are divisible by p = 1.
Also, every subarray of nums will have at most 4 elements that are divisible by 1.
Since all subarrays are distinct, the total number of subarrays satisfying all the constraints is 10.

Constraints:

    1 <= nums.length <= 200
    1 <= nums[i], p <= 200
    1 <= k <= nums.length
*/

struct Solution;

impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        use std::collections::HashSet;
        let mut arrs: HashSet<String> = HashSet::new();
        let n = nums.len();
        for i in 0..n {
            let mut cnt = 0;
            let mut key = "".to_string();
            for &num in nums.iter().skip(i) {
                if num % p == 0 {
                    cnt += 1;
                    if cnt > k {
                        break;
                    }
                }
                key.push(',');
                key.push_str(&num.to_string());
                arrs.insert(key.clone());
            }
        }
        arrs.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_distinct(vec![2, 3, 3, 2, 2], 2, 2), 11);
    assert_eq!(Solution::count_distinct(vec![1, 2, 3, 4], 4, 1), 10);
}
