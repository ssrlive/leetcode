#![allow(dead_code)]

// 3676. Count Bowl Subarrays
// https://leetcode.com/problems/count-bowl-subarrays/
// https://leetcode.cn/problems/count-bowl-subarrays/
//
// Medium
//
// You are given an integer array nums with distinct elements.
//
// A subarray nums[l...r] of nums is called a bowl if:
//
// The subarray has length at least 3. That is, r - l + 1 >= 3.
// The minimum of its two ends is strictly greater than the maximum of all elements in between.
// That is, min(nums[l], nums[r]) > max(nums[l + 1], ..., nums[r - 1]).
// Return the number of bowl subarrays in nums.
//
// Example 1:
//
// Input: nums = [2,5,3,1,4]
//
// Output: 2
//
// Explanation:
//
// The bowl subarrays are [3, 1, 4] and [5, 3, 1, 4].
//
// [3, 1, 4] is a bowl because min(3, 4) = 3 > max(1) = 1.
// [5, 3, 1, 4] is a bowl because min(5, 4) = 4 > max(3, 1) = 3.
//
// Example 2:
//
// Input: nums = [5,1,2,3,4]
//
// Output: 3
//
// Explanation:
//
// The bowl subarrays are [5, 1, 2], [5, 1, 2, 3] and [5, 1, 2, 3, 4].
//
// Example 3:
//
// Input: nums = [1000000000,999999999,999999998]
//
// Output: 0
//
// Explanation:
//
// No subarray is a bowl.
//
// Constraints:
//
// 3 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^9
// nums consists of distinct elements.
//

struct Solution;

impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        let mut s: Vec<usize> = Vec::new();
        for r in 0..nums.len() {
            let a = nums[r];
            while !s.is_empty() && nums[*s.last().unwrap()] <= a {
                let l = s.pop().unwrap();
                if r - l + 1 >= 3 {
                    res += 1;
                }
            }
            if !s.is_empty() && r - *s.last().unwrap() + 1 >= 3 {
                res += 1;
            }
            s.push(r);
        }
        res
    }
}

#[test]
fn test() {
    let nums1 = vec![2, 5, 3, 1, 4];
    assert_eq!(Solution::bowl_subarrays(nums1), 2);

    let nums2 = vec![5, 1, 2, 3, 4];
    assert_eq!(Solution::bowl_subarrays(nums2), 3);

    let nums3 = vec![1_000_000_000, 999_999_999, 999_999_998];
    assert_eq!(Solution::bowl_subarrays(nums3), 0);
}
