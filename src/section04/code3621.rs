#![allow(dead_code)]

// 3621. Number of Integers With Popcount-Depth Equal to K I
// https://leetcode.com/problems/number-of-integers-with-popcount-depth-equal-to-k-i/
// https://leetcode.cn/problems/number-of-integers-with-popcount-depth-equal-to-k-i/
//
// Hard
//
// You are given two integers n and k.
//
// For any positive integer x, define the following sequence:
//
// p0 = x
// pi+1 = popcount(pi) for all i >= 0, where popcount(y) is the number of set bits (1's) in the binary representation of y.
// This sequence will eventually reach the value 1.
//
// The popcount-depth of x is defined as the smallest integer d >= 0 such that pd = 1.
//
// For example, if x = 7 (binary representation "111"). Then, the sequence is: 7 → 3 → 2 → 1, so the popcount-depth of 7 is 3.
//
// Your task is to determine the number of integers in the range [1, n] whose popcount-depth is exactly equal to k.
//
// Return the number of such integers.
//
// Example 1:
//
// Input: n = 4, k = 1
//
// Output: 2
//
// Explanation:
//
// The following integers in the range [1, 4] have popcount-depth exactly equal to 1:
//
// x	Binary	Sequence
// 2	"10"	2 → 1
// 4	"100"	4 → 1
// Thus, the answer is 2.
//
// Example 2:
//
// Input: n = 7, k = 2
//
// Output: 3
//
// Explanation:
//
// The following integers in the range [1, 7] have popcount-depth exactly equal to 2:
//
// x	Binary	Sequence
// 3	"11"	3 → 2 → 1
// 5	"101"	5 → 2 → 1
// 6	"110"	6 → 2 → 1
// Thus, the answer is 3.
//
// Constraints:
//
// 1 <= n <= 10^15
// 0 <= k <= 5
//

struct Solution;

impl Solution {
    pub fn popcount_depth(n: i64, k: i32) -> i64 {
        fn compute_valid_popcounts(k: i64) -> std::collections::BTreeSet<i64> {
            let mut valid = std::collections::BTreeSet::new();
            for c in 1..=64 {
                let mut x: i64 = c;
                let mut depth = 0;
                while x > 1 {
                    x = x.count_ones() as i64;
                    depth += 1;
                }
                if depth == k - 1 {
                    valid.insert(c);
                }
            }
            valid
        }

        fn to_binary(mut n: i64) -> String {
            let mut s = String::new();
            while n > 0 {
                s.push(((n % 2) as u8 + b'0') as char);
                n /= 2;
            }
            s.chars().rev().collect()
        }

        fn dfs(
            pos: i64,
            tight: bool,
            ones: i64,
            binary: &str,
            dp: &mut [[[i64; 65]; 2]; 65],
            valid_popcounts: &std::collections::BTreeSet<i64>,
        ) -> i64 {
            if pos as usize == binary.len() {
                return if valid_popcounts.contains(&ones) { 1 } else { 0 };
            }

            if dp[pos as usize][tight as usize][ones as usize] != -1 {
                return dp[pos as usize][tight as usize][ones as usize];
            }

            let limit = if tight { binary.as_bytes()[pos as usize] - b'0' } else { 1 };
            let mut res = 0;
            for d in 0..=limit {
                let new_tight = tight && (d == limit);
                res += dfs(pos + 1, new_tight, ones + d as i64, binary, dp, valid_popcounts);
            }

            dp[pos as usize][tight as usize][ones as usize] = res;
            res
        }

        if k == 0 {
            return 1;
        }
        let valid_popcounts = compute_valid_popcounts(k as i64);
        if valid_popcounts.is_empty() {
            return 0;
        }
        let binary = to_binary(n);
        let mut dp = [[[-1; 65]; 2]; 65];
        let result = dfs(0, true, 0, &binary, &mut dp, &valid_popcounts);
        if k == 1 && valid_popcounts.contains(&1) {
            return result - 1;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::popcount_depth(5, 1), 2);
    assert_eq!(Solution::popcount_depth(5, 2), 2);
    assert_eq!(Solution::popcount_depth(7, 2), 3);
}
