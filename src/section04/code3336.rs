#![allow(dead_code)]

// 3336. Find the Number of Subsequences With Equal GCD
// https://leetcode.com/problems/find-the-number-of-subsequences-with-equal-gcd/
// https://leetcode.cn/problems/find-the-number-of-subsequences-with-equal-gcd/
//
// Hard
//
// You are given an integer array nums.
//
// Your task is to find the number of pairs of non-empty subsequences(seq1, seq2) of nums that satisfy the following conditions:
//
//     The subsequences seq1 and seq2 are disjoint, meaning no index of nums is common between them.
//     The GCD of the elements of seq1 is equal to the GCD of the elements of seq2.
//
// Return the total number of such pairs.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
//
// Output: 10
//
// Explanation:
//
// The subsequence pairs which have the GCD of their elements equal to 1 are:
//
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//     ([1, 2, 3, 4], [1, 2, 3, 4])
//
// Example 2:
//
// Input: nums = [10,20,30]
//
// Output: 2
//
// Explanation:
//
// The subsequence pairs which have the GCD of their elements equal to 10 are:
//
//    ([10, 20, 30], [10, 20, 30])
//    ([10, 20, 30], [10, 20, 30])
//
// Example 3:
//
// Input: nums = [1,1,1,1]
//
// Output: 50
//
// Constraints:
//
//    1 <= nums.length <= 200
//    1 <= nums[i] <= 200
//

struct Solution;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let mod_num = 1_000_000_007;
        let m = *nums.iter().max().unwrap() as usize;
        let mut dp = vec![vec![0; m + 1]; m + 1];
        dp[0][0] = 1;

        for a in nums {
            let mut dp2 = vec![vec![0; m + 1]; m + 1];
            for i in (0..=m).rev() {
                for j in (0..=m).rev() {
                    let v = dp[i][j];
                    let i2 = Self::gcd(i as i32, a);
                    let j2 = Self::gcd(j as i32, a);
                    dp2[i2 as usize][j] = (dp2[i2 as usize][j] + v) % mod_num;
                    dp2[i][j2 as usize] = (dp2[i][j2 as usize] + v) % mod_num;
                    dp2[i][j] = (dp2[i][j] + v) % mod_num;
                }
            }
            dp = dp2;
        }

        let mut res = 0;
        for (i, dp_i) in dp.iter().enumerate().take(m + 1).skip(1) {
            res = (res + dp_i[i]) % mod_num;
        }
        res
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
}

#[test]

fn test() {
    let nums = vec![1, 2, 3, 4];
    let res = 10;
    assert_eq!(Solution::subsequence_pair_count(nums), res);

    let nums = vec![10, 20, 30];
    let res = 2;
    assert_eq!(Solution::subsequence_pair_count(nums), res);

    let nums = vec![1, 1, 1, 1];
    let res = 50;
    assert_eq!(Solution::subsequence_pair_count(nums), res);
}
