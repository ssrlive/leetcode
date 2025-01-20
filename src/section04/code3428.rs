#![allow(dead_code)]

// 3428. Maximum and Minimum Sums of at Most Size K Subsequences
// https://leetcode.com/problems/maximum-and-minimum-sums-of-at-most-size-k-subsequences/
// https://leetcode.cn/problems/maximum-and-minimum-sums-of-at-most-size-k-subsequences/
//
// Medium
//
// You are given an integer array nums and a positive integer k. Return the sum of the maximum and minimum elements of all
// subsequences of nums with at most k elements.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [1,2,3], k = 2
//
// Output: 24
//
// Explanation:
//
// The subsequences of nums with at most 2 elements are:
// Subsequence 	Minimum	Maximum	Sum
// [1]	1	1	2
// [2]	2	2	4
// [3]	3	3	6
// [1, 2]	1	2	3
// [1, 3]	1	3	4
// [2, 3]	2	3	5
// Final Total	 	 	24
//
// The output would be 24.
//
// Example 2:
//
// Input: nums = [5,0,6], k = 1
//
// Output: 22
//
// Explanation:
//
// For subsequences with exactly 1 element, the minimum and maximum values are the element itself. Therefore, the total is 5 + 5 + 0 + 0 + 6 + 6 = 22.
//
// Example 3:
//
// Input: nums = [1,1,1], k = 2
//
// Output: 12
//
// Explanation:
//
// The subsequences [1, 1] and [1] each appear 3 times. For all of them, the minimum and maximum are both 1. Thus, the total is 12.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 0 <= nums[i] <= 10^9
// - 1 <= k <= min(70, nums.length)
//

struct Solution;

impl Solution {
    pub fn min_max_sums(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        nums.sort();
        let mut ans = 0;
        let mod_num = 1_000_000_007;
        let mut ncr = vec![vec![0; k + 1]; nums.len() + 1];

        ncr[0][0] = 1;
        for n in 1..=nums.len() {
            ncr[n][0] = 1;
            for r in 1..=k {
                ncr[n][r] = (ncr[n - 1][r - 1] + ncr[n - 1][r]) % mod_num;
            }
        }

        for n in 0..nums.len() {
            let mut number_of_subsequences = 0;
            for r in 0..=k - 1 {
                if n >= r {
                    number_of_subsequences = (number_of_subsequences + ncr[n][r]) % mod_num;
                }
            }
            ans = (ans + (nums[n] + nums[nums.len() - n - 1]) * number_of_subsequences) % mod_num;
        }
        ans as _
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let k = 2;
    let output = 24;
    assert_eq!(Solution::min_max_sums(nums, k), output);

    let nums = vec![5, 0, 6];
    let k = 1;
    let output = 22;
    assert_eq!(Solution::min_max_sums(nums, k), output);

    let nums = vec![1, 1, 1];
    let k = 2;
    let output = 12;
    assert_eq!(Solution::min_max_sums(nums, k), output);

    let nums = vec![
        10, 5, 9, 9, 10, 10, 7, 7, 9, 6, 9, 6, 7, 6, 4, 9, 8, 4, 2, 0, 0, 3, 9, 3, 10, 3, 1, 9, 8, 2, 8, 2, 0, 7, 7, 6, 4, 6, 7, 3, 2, 5,
        6, 6, 5, 0, 5, 7, 8, 1,
    ];
    let k = 29;
    let output = 428790069;
    assert_eq!(Solution::min_max_sums(nums, k), output);
}
