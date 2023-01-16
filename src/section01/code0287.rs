#![allow(dead_code)]

// 287. Find the Duplicate Number
// https://leetcode.com/problems/find-the-duplicate-number/
// https://leetcode.cn/problems/find-the-duplicate-number/
//
// Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
//
// There is only one repeated number in nums, return this repeated number.
//
// You must solve the problem without modifying the array nums and uses only constant extra space.
//
// Example 1:
//
// Input: nums = [1,3,4,2,2]
// Output: 2
//
// Example 2:
//
// Input: nums = [3,1,3,4,2]
// Output: 3
//
// Constraints:
//
// - 1 <= n <= 10^5
// - nums.length == n + 1
// - 1 <= nums[i] <= n
// - All the integers in nums appear only once except for precisely one integer which appears two or more times.
//
// Follow up:
//
// How can we prove that at least one duplicate number must exist in nums?
// Can you solve the problem in linear runtime complexity?
//

struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (1, nums.len() as i32 - 1);
        while l < r {
            let (mut c, mut m) = (0, (l + r) / 2);
            if m == l {
                m = l + 1;
            }
            for &x in nums.iter() {
                if x < m {
                    c += 1;
                }
            }
            if c >= m {
                r = m - 1;
            } else {
                l = m;
            }
        }
        l
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
}
