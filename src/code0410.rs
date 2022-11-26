#![allow(dead_code)]

// 410. Split Array Largest Sum
// https://leetcode.com/problems/split-array-largest-sum/
//
// Given an array which consists of non-negative integers and an integer m, you can split the array
// into m non-empty continuous subarrays. Write an algorithm to minimize the largest sum among these
// m subarrays.
//
// Note:
// If n is the length of array, assume the following constraints are satisfied:
//
// - 1 ≤ n ≤ 1000
// - 1 ≤ m ≤ min(50, n)
//
// Examples:
//
// Input:
// nums = [7,2,5,10,8]
// m = 2
//
// Output:
// 18
//
// Explanation:
// There are four ways to split nums into two subarrays.
// The best way is to split it into [7,2,5] and [10,8],
// where the largest sum among the two subarrays is only 18.
//
// Constraints:
//
// - 1 <= nums.length <= 1000
// - 0 <= nums[i] <= 10^6
// - 1 <= m <= min(50, nums.length)

struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for &n in &nums {
            left = left.max(n);
            right += n;
        }
        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::split(&nums, mid) <= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    fn split(nums: &[i32], max_sum: i32) -> i32 {
        let mut sum = 0;
        let mut count = 1;
        for &n in nums {
            if sum + n > max_sum {
                sum = n;
                count += 1;
            } else {
                sum += n;
            }
        }
        count
    }
}

#[test]
fn test() {
    let nums = vec![7, 2, 5, 10, 8];
    let m = 2;
    assert_eq!(Solution::split_array(nums, m), 18);
}
