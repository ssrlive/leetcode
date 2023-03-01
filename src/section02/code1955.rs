#![allow(dead_code)]

/*

// 1955. Count Number of Special Subsequences
// https://leetcode.com/problems/count-number-of-special-subsequences/
// https://leetcode.cn/problems/count-number-of-special-subsequences/
//
// Hard
//
// A sequence is special if it consists of a positive number of 0s, followed by a positive number of 1s, then a positive number of 2s.

    For example, [0,1,2] and [0,0,1,1,1,2] are special.
    In contrast, [2,1,0], [1], and [0,1,2,0] are not special.

Given an array nums (consisting of only integers 0, 1, and 2), return the number of different subsequences that are special. Since the answer may be very large, return it modulo 109 + 7.

A subsequence of an array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements. Two subsequences are different if the set of indices chosen are different.

Example 1:

Input: nums = [0,1,2,2]
Output: 3
Explanation: The special subsequences are bolded [0,1,2,2], [0,1,2,2], and [0,1,2,2].

Example 2:

Input: nums = [2,2,0,0]
Output: 0
Explanation: There are no special subsequences in [2,2,0,0].

Example 3:

Input: nums = [0,1,2,0,1,2]
Output: 7
Explanation: The special subsequences are bolded:
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]

Constraints:

    1 <= nums.length <= 10^5
    0 <= nums[i] <= 2
*/

/*
   int countSpecialSubsequences(vector<int>& A) {
       vector<int> dp = {0, 0, 0};
       int mod = 1e9 + 7;
       for (int& a: A)
           dp[a] = ((dp[a] + dp[a]) % mod + (a > 0 ? dp[a-1] : 1)) % mod;
       return dp[2];
   }
*/

struct Solution;

impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, 0, 0];
        let mod_ = 1_000_000_007;
        for &a in nums.iter() {
            let a = a as usize;
            dp[a] = ((dp[a] + dp[a]) % mod_ + (if a > 0 { dp[a - 1] } else { 1 })) % mod_;
        }
        dp[2]
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 2, 2];
    let res = 3;
    assert_eq!(Solution::count_special_subsequences(nums), res);
    let nums = vec![2, 2, 0, 0];
    let res = 0;
    assert_eq!(Solution::count_special_subsequences(nums), res);
    let nums = vec![0, 1, 2, 0, 1, 2];
    let res = 7;
    assert_eq!(Solution::count_special_subsequences(nums), res);
}
