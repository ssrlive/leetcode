#![allow(dead_code)]

// 3539. Find Sum of Array Product of Magical Sequences
// https://leetcode.com/problems/find-sum-of-array-product-of-magical-sequences
// https://leetcode.cn/problems/find-sum-of-array-product-of-magical-sequences
//
// Hard
//
// You are given two integers, m and k, and an integer array nums.
// A sequence of integers seq is called magical if:
//
//     seq has a size of m.
//     0 <= seq[i] < nums.length
//     The binary representation of 2seq[0] + 2seq[1] + ... + 2seq[m - 1] has k set bits.
//
// The array product of this sequence is defined as prod(seq) = (nums[seq[0]] * nums[seq[1]] * ... * nums[seq[m - 1]]).
//
// Return the sum of the array products for all valid magical sequences.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// A set bit refers to a bit in the binary representation of a number that has a value of 1.
//
// Example 1:
//
// Input: m = 5, k = 5, nums = [1,10,100,10000,1000000]
//
// Output: 991600007
//
// Explanation:
//
// All permutations of [0, 1, 2, 3, 4] are magical sequences, each with an array product of 1013.
//
// Example 2:
//
// Input: m = 2, k = 2, nums = [5,4,3,2,1]
//
// Output: 170
//
// Explanation:
//
// The magical sequences are [0, 1], [0, 2], [0, 3], [0, 4], [1, 0], [1, 2], [1, 3], [1, 4], [2, 0], [2, 1], [2, 3],
//                           [2, 4], [3, 0], [3, 1], [3, 2], [3, 4], [4, 0], [4, 1], [4, 2], and [4, 3].
//
// Example 3:
//
// Input: m = 1, k = 1, nums = [28]
//
// Output: 28
//
// Explanation:
//
// The only magical sequence is [0].
//
// Constraints:
//
//     1 <= k <= m <= 30
//     1 <= nums.length <= 50
//     1 <= nums[i] <= 10^8
//

struct Solution;

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        fn mod_pow(mut a: i64, mut e: i64, mod_: i64) -> i64 {
            let mut res = 1;
            while e > 0 {
                if e & 1 == 1 {
                    res = res * a % mod_;
                }
                a = a * a % mod_;
                e >>= 1;
            }
            res
        }

        let mod_ = 1_000_000_007;
        let m = m as usize;
        let k = k as usize;
        let n = nums.len();
        let mut f = vec![1; m + 1];
        let mut inverse_f = vec![1; m + 1];
        for i in 1..=m {
            f[i] = f[i - 1] * i as i64 % mod_;
        }
        inverse_f[m] = mod_pow(f[m], mod_ - 2, mod_);
        for i in (1..=m).rev() {
            inverse_f[i - 1] = inverse_f[i] * i as i64 % mod_;
        }
        let mut pow_nums = vec![vec![1; m + 1]; n];
        for i in 0..n {
            for c in 1..=m {
                pow_nums[i][c] = pow_nums[i][c - 1] * nums[i] as i64 % mod_;
            }
        }
        let mut dp = vec![vec![vec![vec![0; m + 1]; k + 1]; m + 1]; n + 1];
        dp[0][0][0][0] = 1;
        for i in 0..n {
            for m1 in 0..=m {
                for k1 in 0..=k {
                    for m2 in 0..=m {
                        let val = dp[i][m1][k1][m2];
                        if val == 0 {
                            continue;
                        }
                        for (c, &inverse_f_c) in inverse_f.iter().enumerate().take((m - m1) + 1) {
                            let m12 = m1 + c;
                            let s = c + m2;
                            let bit = s & 1;
                            let k2 = k1 + bit;
                            if k2 > k {
                                continue;
                            }
                            let m22 = s >> 1;
                            dp[i + 1][m12][k2][m22] = (dp[i + 1][m12][k2][m22] + val * inverse_f_c % mod_ * pow_nums[i][c]) % mod_;
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for k1 in 0..=k {
            for m2 in 0..=m {
                let val = dp[n][m][k1][m2];
                if val == 0 {
                    continue;
                }
                let bits = m2.count_ones() as usize;
                if k1 + bits == k {
                    ans = (ans + val) % mod_;
                }
            }
        }
        ans = ans * f[m] % mod_;
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::magical_sum(5, 5, vec![1, 10, 100, 10000, 1000000]), 991600007);
    assert_eq!(Solution::magical_sum(2, 2, vec![5, 4, 3, 2, 1]), 170);
    assert_eq!(Solution::magical_sum(1, 1, vec![28]), 28);
}
