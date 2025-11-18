#![allow(dead_code)]

// 3671. Sum of Beautiful Subsequences
// https://leetcode.com/problems/sum-of-beautiful-subsequences/
// https://leetcode.cn/problems/sum-of-beautiful-subsequences/
//
// Hard
//
// You are given an integer array nums of length n.
//
// For every positive integer g, we define the beauty of g as the product of g and the number of strictly increasing subsequences of nums whose greatest common divisor (GCD) is exactly g.
//
// Return the sum of beauty values for all positive integers g.
//
// Since the answer could be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [1,2,3]
//
// Output: 10
//
// Explanation:
//
// All strictly increasing subsequences and their GCDs are:
//
// Subsequence	GCD
// [1]	1
// [2]	2
// [3]	3
// [1,2]	1
// [1,3]	1
// [2,3]	1
// [1,2,3]	1
// Calculating beauty for each GCD:
//
// GCD	Count of subsequences	Beauty (GCD × Count)
// 1	5	1 × 5 = 5
// 2	1	2 × 1 = 2
// 3	1	3 × 1 = 3
// Total beauty is 5 + 2 + 3 = 10.
//
// Example 2:
//
// Input: nums = [4,6]
//
// Output: 12
//
// Explanation:
//
// All strictly increasing subsequences and their GCDs are:
//
// Subsequence	GCD
// [4]	4
// [6]	6
// [4,6]	2
// Calculating beauty for each GCD:
//
// GCD	Count of subsequences	Beauty (GCD × Count)
// 2	1	2 × 1 = 2
// 4	1	4 × 1 = 4
// 6	1	6 × 1 = 6
// Total beauty is 2 + 4 + 6 = 12.
//
// Constraints:
//
// 1 <= n == nums.length <= 10^4
// 1 <= nums[i] <= 7 * 10^4
//

struct Solution;

impl Solution {
    pub fn total_beauty(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let _n = nums.len();
        let &max_a = nums.iter().max().unwrap();

        let mut divisors = vec![vec![]; (max_a + 1) as usize];
        for d in 1..=max_a {
            let mut m = d;
            while m <= max_a {
                divisors[m as usize].push(d);
                m += d;
            }
        }

        let mut a = vec![vec![]; (max_a + 1) as usize];
        for &x in &nums {
            for &d in &divisors[x as usize] {
                a[d as usize].push(x);
            }
        }

        let mut num_inc = vec![0i32; (max_a + 1) as usize];

        let count_increasing = |seq: &Vec<i32>| -> i32 {
            if seq.is_empty() {
                return 0;
            }
            let mut vals = seq.clone();
            vals.sort_unstable();
            vals.dedup();
            let m = vals.len();
            let mut bit = vec![0i32; m + 1];

            let add = |bit: &mut Vec<i32>, mut i: usize, v: i32| {
                while i <= m {
                    let x = bit[i] as i64 + v as i64;
                    bit[i] = (x % MOD) as i32;
                    i += i & (!i + 1);
                }
            };
            let sum = |bit: &Vec<i32>, mut i: usize| -> i32 {
                let mut s: i64 = 0;
                while i > 0 {
                    s += bit[i] as i64;
                    s %= MOD;
                    i -= i & (!i + 1);
                }
                s as i32
            };

            let mut total: i64 = 0;
            for &v in seq {
                let r = vals.binary_search(&v).unwrap() + 1;
                let less = sum(&bit, r - 1);
                let mut add_here = less + 1;
                if add_here >= MOD as i32 {
                    add_here -= MOD as i32;
                }
                add(&mut bit, r, add_here);
                total += add_here as i64;
                total %= MOD;
            }
            total as i32
        };
        for x in 1..=max_a {
            if !a[x as usize].is_empty() {
                num_inc[x as usize] = count_increasing(&a[x as usize]);
            }
        }
        let mut dp = vec![0i32; (max_a + 1) as usize];
        for x in (1..=max_a).rev() {
            let mut v = num_inc[x as usize] as i64;
            let mut y = x + x;
            while y <= max_a {
                v -= dp[y as usize] as i64;
                if v < 0 {
                    v += MOD;
                }
                y += x;
            }
            dp[x as usize] = (v % MOD) as i32;
        }
        let mut ans: i64 = 0;
        for x in 1..=max_a {
            if dp[x as usize] != 0 {
                ans += (x as i64) * (dp[x as usize] as i64);
                ans %= MOD;
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::total_beauty(nums), 10);

    let nums = vec![4, 6];
    assert_eq!(Solution::total_beauty(nums), 12);
}
