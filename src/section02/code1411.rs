#![allow(dead_code)]

// 1411. Number of Ways to Paint N × 3 Grid
// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/
// https://leetcode.cn/problems/number-of-ways-to-paint-n-3-grid/
//
// Hard
//
// You have a grid of size n x 3 and you want to paint each cell of the grid with exactly one of the
// three colors: Red, Yellow, or Green while making sure that no two adjacent cells have the same color
// (i.e., no two cells that share vertical or horizontal sides have the same color).
//
// Given n the number of rows of the grid, return the number of ways you can paint this grid.
// As the answer may grow large, the answer must be computed modulo 109 + 7.
//
// Example 1:
//
// Input: n = 1
// Output: 12
// Explanation: There are 12 possible way to paint the grid as shown.
//
// Example 2:
//
// Input: n = 5000
// Output: 30228214
//
// Constraints:
//
// -    n == grid.length
// -    1 <= n <= 5000
//

struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![(6, 6)];
        for _ in 1..n {
            let (a, b) = dp.last().unwrap();
            dp.push(((3 * a + 2 * b) % MOD, (2 * a + 2 * b) % MOD));
        }
        ((dp.last().unwrap().0 + dp.last().unwrap().1) % MOD) as _
    }
}

#[test]
fn test() {
    let cases = vec![(1, 12), (5000, 30228214)];
    for (n, expected) in cases {
        assert_eq!(Solution::num_of_ways(n), expected);
    }
}
