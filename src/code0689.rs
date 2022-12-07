#![allow(dead_code)]

// 689. Maximum Sum of 3 Non-Overlapping Subarrays
// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/
//
// Given an integer array nums and an integer k, find three non-overlapping subarrays of length k with maximum sum and return them.
//
// Return the result as a list of indices representing the starting position of each interval (0-indexed).
// If there are multiple answers, return the lexicographically smallest one.
//
// Example 1:
//
// Input: nums = [1,2,1,2,6,7,5,1], k = 2
// Output: [0,3,5]
// Explanation: Subarrays [1, 2], [2, 6], [7, 5] correspond to the starting indices [0, 3, 5].
// We could have also taken [2, 1], but an answer of [1, 3, 5] would be lexicographically larger.
//
// Example 2:
//
// Input: nums = [1,2,1,2,1,2,1,2,1], k = 2
// Output: [0,2,4]
//
// Constraints:
//
// - 1 <= nums.length <= 2 * 10^4
// - 1 <= nums[i] < 2^16
// - 1 <= k <= floor(nums.length / 3)
//

struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut pref = vec![0; n];
        let mut ipref = vec![0; n];
        let mut suff = vec![0; n];
        let mut isuff = vec![0; n];
        {
            let mut sum = 0;
            for &item in nums.iter().take(k) {
                sum += item;
            }
            pref[k - 1] = sum;
            ipref[k - 1] = 0;
            for i in k..n {
                sum += nums[i] - nums[i - k];
                ipref[i] = i - k + 1;
                pref[i] = sum;
                if pref[i - 1] >= pref[i] {
                    ipref[i] = ipref[i - 1];
                    pref[i] = pref[i - 1];
                }
            }
        }
        {
            let mut sum = 0;
            for i in (n - k..n).rev() {
                sum += nums[i];
            }
            suff[n - k] = sum;
            isuff[n - k] = n - k;
            for i in (0..n - k).rev() {
                sum += nums[i] - nums[i + k];
                suff[i] = sum;
                isuff[i] = i;
                if suff[i + 1] > suff[i] {
                    isuff[i] = isuff[i + 1];
                    suff[i] = suff[i + 1];
                }
            }
        }
        let mut sum = 0;
        for &item in nums.iter().take(k) {
            sum += item;
        }
        let mut maks = 0;
        let mut ans = vec![];
        for i in k..n {
            sum += nums[i] - nums[i - k];
            if i - k >= k - 1 && i + 1 < n {
                let cur = pref[i - k] + sum + suff[i + 1];
                if cur > maks {
                    maks = cur;
                    ans = vec![ipref[i - k] as i32, (i - k + 1) as i32, isuff[i + 1] as i32];
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
    assert_eq!(Solution::max_sum_of_three_subarrays(nums, 2), vec![0, 3, 5]);

    let nums = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
    assert_eq!(Solution::max_sum_of_three_subarrays(nums, 2), vec![0, 2, 4]);

    let nums = vec![9, 8, 7, 6, 2, 2, 2, 2];
    assert_eq!(Solution::max_sum_of_three_subarrays(nums, 2), vec![0, 2, 4]);
}
