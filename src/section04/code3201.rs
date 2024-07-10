#![allow(dead_code)]

// 3201. Find the Maximum Length of Valid Subsequence I
// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/
// https://leetcode.cn/problems/find-the-maximum-length-of-valid-subsequence-i/
//
// Medium
//
// You are given an integer array nums.
//
// A subsequence sub of nums with length x is called valid if it satisfies:
//
//     (sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] + sub[x - 1]) % 2.
//
// Return the length of the longest valid subsequence of nums.
//
// A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
//
// Output: 4
//
// Explanation:
//
// The longest valid subsequence is [1, 2, 3, 4].
//
// Example 2:
//
// Input: nums = [1,2,1,1,2,1,2]
//
// Output: 6
//
// Explanation:
//
// The longest valid subsequence is [1, 2, 1, 2, 1, 2].
//
// Example 3:
//
// Input: nums = [1,3]
//
// Output: 2
//
// Explanation:
//
// The longest valid subsequence is [1, 3].
//
// Constraints:
//
//     2 <= nums.length <= 2 * 10^5
//     1 <= nums[i] <= 10^7
//

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut prev = 2;
        let (mut cnt0, mut cnt1, mut cnt2) = (0, 0, 0);
        for &nums_i in &nums {
            let curr = nums_i % 2;
            if curr == 1 {
                cnt0 += 1;
            } else {
                cnt1 += 1;
            }

            if curr != prev {
                cnt2 += 1;
                prev = curr;
            }
        }

        cnt0.max(cnt1).max(cnt2)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4]), 4);
    assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 2, 1, 2]), 6);
    assert_eq!(Solution::maximum_length(vec![1, 3]), 2);
}
