#![allow(dead_code)]

// 3351. Sum of Good Subsequences
// https://leetcode.com/problems/sum-of-good-subsequences/
// https://leetcode.cn/problems/sum-of-good-subsequences/
//
// Hard
//
// You are given an integer array nums. A good subsequence is defined as a subsequence of nums
// where the absolute difference between any two consecutive elements in the subsequence is exactly 1.
//
// Return the sum of all possible good subsequences of nums.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Note that a subsequence of size 1 is considered good by definition.
//
// Example 1:
//
// Input: nums = [1,2,1]
//
// Output: 14
//
// Explanation:
//
//     Good subsequences are: [1], [2], [1], [1,2], [2,1], [1,2,1].
//     The sum of elements in these subsequences is 14.
//
// Example 2:
//
// Input: nums = [3,4,5]
//
// Output: 40
//
// Explanation:
//
//     Good subsequences are: [3], [4], [5], [3,4], [4,5], [3,4,5].
//     The sum of elements in these subsequences is 40.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     0 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
        let m = nums.iter().max().unwrap_or(&0) + 3;
        let mod_val = 1_000_000_007;
        let mut total_sum = vec![0; m as usize];
        let mut total_cnt = vec![0; m as usize];

        for &v in &nums {
            let v = v as usize;
            let (tmp_sum, tmp_cnt) = if v != 0 {
                (
                    (total_sum[v - 1] + total_sum[v + 1]) % mod_val,
                    (total_cnt[v - 1] + total_cnt[v + 1]) % mod_val,
                )
            } else {
                (total_sum[v + 1], total_cnt[v + 1])
            };

            total_cnt[v] = (total_cnt[v] + tmp_cnt + 1) % mod_val;
            total_sum[v] = (total_sum[v] + tmp_sum + v * (tmp_cnt + 1) % mod_val) % mod_val;
        }

        total_sum.iter().fold(0, |acc, &x| (acc + x) % mod_val) as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1];
    let output = 14;
    assert_eq!(Solution::sum_of_good_subsequences(nums), output);

    let nums = vec![3, 4, 5];
    let output = 40;
    assert_eq!(Solution::sum_of_good_subsequences(nums), output);
}
