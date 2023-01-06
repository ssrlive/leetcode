#![allow(dead_code)]

// 1004. Max Consecutive Ones III
// https://leetcode.com/problems/max-consecutive-ones-iii/
// https://leetcode.cn/problems/max-consecutive-ones-iii/
//
// Given a binary array nums and an integer k, return the maximum number of
// consecutive 1's in the array if you can flip at most k 0's.
//
// Example 1:
//
// Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
// Output: 6
// Explanation: [1,1,1,0,0,1,1,1,1,1,1]
// Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
//
// Example 2:
//
// Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
// Output: 10
// Explanation: [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
// Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - nums[i] is either 0 or 1.
// - 0 <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut k = k;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_n {
            if nums[hi] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[lo] == 0 {
                    k += 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        (hi - lo) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2, 6),
        (vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3, 10),
    ];
    for (nums, k, expect) in cases {
        assert_eq!(Solution::longest_ones(nums, k), expect);
    }
}
