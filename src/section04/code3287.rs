#![allow(dead_code)]

// 3287. Find the Maximum Sequence Value of Array
// https://leetcode.com/problems/find-the-maximum-sequence-value-of-array/
// https://leetcode.cn/problems/find-the-maximum-sequence-value-of-array/
//
// Hard
//
// You are given an integer array nums and a positive integer k.
//
// The value of a sequence seq of size 2 * x is defined as:
//
//     (seq[0] OR seq[1] OR ... OR seq[x - 1]) XOR (seq[x] OR seq[x + 1] OR ... OR seq[2 * x - 1]).
//
// Return the maximum value of any subsequence of nums having size 2 * k.
//
// Example 1:
//
// Input: nums = [2,6,7], k = 1
//
// Output: 5
//
// Explanation:
//
// The subsequence [2, 7] has the maximum value of 2 XOR 7 = 5.
//
// Example 2:
//
// Input: nums = [4,2,5,6,7], k = 2
//
// Output: 2
//
// Explanation:
//
// The subsequence [4, 5, 6, 7] has the maximum value of (4 OR 5) XOR (6 OR 7) = 2.
//
// Constraints:
//
//     2 <= nums.length <= 400
//     1 <= nums[i] < 2^7
//     1 <= k <= nums.length / 2
//

struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let left = Self::all_possible_ors_till_an_index(&nums, k);
        let mut nums = nums;
        nums.reverse();
        let mut right = Self::all_possible_ors_till_an_index(&nums, k);
        right.reverse();
        let mut res = 0;
        let mut i = k as usize - 1;
        while i + (k as usize) < nums.len() {
            for a in 0..=128 {
                for b in 0..=128 {
                    if left[i][k as usize][a] != 0 && right[i + 1][k as usize][b] != 0 {
                        res = res.max(a ^ b);
                    }
                }
            }
            i += 1;
        }

        res as _
    }

    fn all_possible_ors_till_an_index(nums: &[i32], k: i32) -> Vec<Vec<Vec<i32>>> {
        let n = nums.len();
        let mut dp = vec![vec![vec![0; 129]; (k + 1) as usize]; n];
        dp[0][1][nums[0] as usize] = 1;
        dp[0][0][0] = 1;

        for i in 1..n {
            dp[i][0][0] = 1;
            for j in 1..=k {
                for x in 0..=128 {
                    if dp[i][j as usize][x] == 0 {
                        dp[i][j as usize][x] = dp[i - 1][j as usize][x];
                    }
                    let next = nums[i] | x as i32;
                    if dp[i - 1][j as usize - 1][x] != 0 {
                        dp[i][j as usize][next as usize] = dp[i - 1][j as usize - 1][x];
                    }
                }
            }
        }

        dp
    }
}

#[test]
fn test() {
    let nums = vec![2, 6, 7];
    let k = 1;
    let output = 5;
    assert_eq!(Solution::max_value(nums, k), output);

    let nums = vec![4, 2, 5, 6, 7];
    let k = 2;
    let output = 2;
    assert_eq!(Solution::max_value(nums, k), output);

    let nums = vec![
        7, 118, 102, 88, 65, 43, 67, 93, 113, 78, 75, 34, 48, 8, 71, 2, 37, 34, 5, 43, 51, 125, 25, 3, 56, 56, 41, 26, 1, 19, 67, 126, 85,
        50, 77, 104, 37, 29, 115, 78, 112, 18, 73, 49, 15, 67, 70, 108, 107, 63,
    ];
    let k = 8;
    let output = 124;
    assert_eq!(Solution::max_value(nums, k), output);
}
