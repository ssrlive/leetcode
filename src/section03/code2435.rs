#![allow(dead_code)]

// 2435. Paths in Matrix Whose Sum Is Divisible by K
// https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/
// https://leetcode.cn/problems/paths-in-matrix-whose-sum-is-divisible-by-k/
//
// You are given a 0-indexed m x n integer matrix grid and an integer k. You are currently
// at position (0, 0) and you want to reach position (m - 1, n - 1) moving only down or right.
//
// Return the number of paths where the sum of the elements on the path is divisible by k.
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: grid = [[5,2,4],[3,0,5],[0,7,2]], k = 3
// Output: 2
// Explanation: There are two paths where the sum of the elements on the path is divisible by k.
// The first path highlighted in red has a sum of 5 + 2 + 4 + 5 + 2 = 18 which is divisible by 3.
// The second path highlighted in blue has a sum of 5 + 3 + 0 + 5 + 2 = 15 which is divisible by 3.
//
// Example 2:
//
// Input: grid = [[0,0]], k = 5
// Output: 1
// Explanation: The path highlighted in red has a sum of 0 + 0 = 0 which is divisible by 5.
//
// Example 3:
//
// Input: grid = [[7,3,4,9],[2,3,6,2],[2,3,7,0]], k = 1
// Output: 10
// Explanation: Every integer is divisible by 1 so the sum of the elements on every possible path is divisible by k.
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 5 * 10^4
// - 1 <= m * n <= 5 * 10^4
// - 0 <= grid[i][j] <= 100
// - 1 <= k <= 50
//

struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let k = k as usize;
        const MOD: usize = 1_000_000_007;
        let mut dp = vec![vec![vec![0; k]; n + 1]; m + 1];
        dp[0][1][0] = 1;
        for i in 1..=m {
            for j in 1..=n {
                for l in 0..k {
                    dp[i][j][(l + grid[i - 1][j - 1] as usize) % k] =
                        ((dp[i][j][(l + grid[i - 1][j - 1] as usize) % k] + dp[i - 1][j][l]) % MOD
                            + (dp[i][j][(l + grid[i - 1][j - 1] as usize) % k] + dp[i][j - 1][l]) % MOD)
                            % MOD;
                }
            }
        }
        dp[m][n][0] as _
    }
}

#[test]
fn test() {
    let cased = vec![
        (vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3, 2),
        (vec![vec![0, 0]], 5, 1),
        (vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]], 1, 10),
    ];
    for (grid, k, expect) in cased {
        assert_eq!(Solution::number_of_paths(grid, k), expect);
    }
}
