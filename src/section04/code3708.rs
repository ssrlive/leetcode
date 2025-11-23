#![allow(dead_code)]

// 3708. Longest Fibonacci Subarray
// https://leetcode.com/problems/longest-fibonacci-subarray/
// https://leetcode.cn/problems/longest-fibonacci-subarray/
//
// Medium
//
// You are given an array of positive integers nums.
//
// A Fibonacci array is a contiguous sequence whose third and subsequent terms each equal the sum of the two preceding terms.
//
// Return the length of the longest Fibonacci subarray in nums.
//
// Note: Subarrays of length 1 or 2 are always Fibonacci.
//
// Example 1:
//
// Input: nums = [1,1,1,1,2,3,5,1]
//
// Output: 5
//
// Explanation:
//
// The longest Fibonacci subarray is nums[2..6] = [1, 1, 2, 3, 5].
//
// [1, 1, 2, 3, 5] is Fibonacci because 1 + 1 = 2, 1 + 2 = 3, and 2 + 3 = 5.
//
// Example 2:
//
// Input: nums = [5,2,7,9,16]
//
// Output: 5
//
// Explanation:
//
// The longest Fibonacci subarray is nums[0..4] = [5, 2, 7, 9, 16].
//
// [5, 2, 7, 9, 16] is Fibonacci because 5 + 2 = 7, 2 + 7 = 9, and 7 + 9 = 16.
//
// Example 3:
//
// Input: nums = [1000000000,1000000000,1000000000]
//
// Output: 2
//
// Explanation:
//
// The longest Fibonacci subarray is nums[1..2] = [1000000000, 1000000000].
//
// [1000000000, 1000000000] is Fibonacci because its length is 2.
//
// Constraints:
//
// 3 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let mut curr = 0;
        let mut res = 0;

        for i in 0..nums.len() - 2 {
            if nums[i] + nums[i + 1] == nums[i + 2] {
                curr += 1;
                if curr > res {
                    res = curr;
                }
            } else {
                curr = 0;
            }
        }

        res + 2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 2, 3, 5, 1]), 5);
    assert_eq!(Solution::longest_subarray(vec![5, 2, 7, 9, 16]), 5);
    assert_eq!(Solution::longest_subarray(vec![1000000000, 1000000000, 1000000000]), 2);
}
