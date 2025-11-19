#![allow(dead_code)]

// 3686. Number of Stable Subsequences
// https://leetcode.com/problems/number-of-stable-subsequences/
// https://leetcode.cn/problems/number-of-stable-subsequences/
//
// Hard
//
// You are given an integer array nums.
//
// A subsequence is stable if it does not contain three consecutive elements with the same parity when the subsequence is read in order (i.e., consecutive inside the subsequence).
//
// Return the number of stable subsequences.
//
// Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [1,3,5]
//
// Output: 6
//
// Explanation:
//
// Stable subsequences are [1], [3], [5], [1, 3], [1, 5], and [3, 5].
// Subsequence [1, 3, 5] is not stable because it contains three consecutive odd numbers. Thus, the answer is 6.
//
// Example 2:
//
// Input: nums = [2,3,4,2]
//
// Output: 14
//
// Explanation:
//
// The only subsequence that is not stable is [2, 4, 2], which contains three consecutive even numbers.
// All other subsequences are stable. Thus, the answer is 14.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn count_stable_subsequences(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let n = nums.len();
        let m: usize = 3;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; m]; m]; n + 1];
        dp[0][m - 1][m - 1] = 1; // base: empty subsequence previous parities sentinel
        for i in 1..=n {
            let curr_p = (nums[i - 1] % 2) as usize;
            // copy previous state (clone of 3x3 is negligible)
            let prev_layer = dp[i - 1].clone();
            dp[i] = prev_layer.clone();
            // transition: append nums[i-1] if not forming three same parity in a row
            for (last_p, prev_row) in prev_layer.iter().enumerate().take(m) {
                for (sec_p, &val) in prev_row.iter().enumerate().take(m) {
                    if !(curr_p == last_p && last_p == sec_p) {
                        let v = &mut dp[i][curr_p][last_p];
                        *v += val;
                        if *v >= MOD {
                            *v -= MOD;
                        }
                    }
                }
            }
        }
        let mut ans: i32 = 0;
        for row in dp[n].iter().take(m) {
            for &val in row.iter().take(m) {
                ans += val;
                if ans >= MOD {
                    ans -= MOD;
                }
            }
        }
        (ans - 1 + MOD) % MOD
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 3, 5];
    assert_eq!(Solution::count_stable_subsequences(nums1), 6);

    let nums2 = vec![2, 3, 4, 2];
    assert_eq!(Solution::count_stable_subsequences(nums2), 14);
}
