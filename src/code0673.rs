#![allow(dead_code)]

// 673. Number of Longest Increasing Subsequence
// https://leetcode.com/problems/number-of-longest-increasing-subsequence/
// https://leetcode.cn/problems/number-of-longest-increasing-subsequence/
//
// Given an integer array nums, return the number of longest increasing subsequences.
//
// Notice that the sequence has to be strictly increasing.
//
// Example 1:
//
// Input: nums = [1,3,5,4,7]
// Output: 2
// Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
//
// Example 2:
//
// Input: nums = [2,2,2,2,2]
// Output: 5
// Explanation: The length of the longest increasing subsequence is 1, and there are 5 increasing subsequences of length 1, so output 5.
//
// Constraints:
//
// - 1 <= nums.length <= 2000
// - -10^6 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        Self::_find_number_of_lis(nums).unwrap_or(-1)
    }
    fn _find_number_of_lis(nums: Vec<i32>) -> Option<i32> {
        let n = nums.len();
        if n == 0 {
            return Some(0);
        }

        let mut dp = vec![1; n];
        let mut count = vec![1; n];

        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    match dp[i].cmp(&(dp[j] + 1)) {
                        std::cmp::Ordering::Less => {
                            dp[i] = dp[j] + 1;
                            count[i] = count[j];
                        }
                        std::cmp::Ordering::Equal => {
                            count[i] += count[j];
                        }
                        _ => {}
                    }
                }
            }
        }

        let max_len = dp.iter().max()?;
        let mut res = 0;
        for i in 0..n {
            if dp[i] == *max_len {
                res += count[i];
            }
        }

        Some(res)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
}
