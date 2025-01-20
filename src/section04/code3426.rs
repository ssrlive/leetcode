#![allow(dead_code)]

// 3426. Manhattan Distances of All Arrangements of Pieces
// https://leetcode.com/problems/manhattan-distances-of-all-arrangements-of-pieces/
// https://leetcode.cn/problems/manhattan-distances-of-all-arrangements-of-pieces/
//
// Hard
//
// You are given three integers m, n, and k.
//
// There is a rectangular grid of size m × n containing k identical pieces. Return the sum of Manhattan
// distances between every pair of pieces over all valid arrangements of pieces.
//
// A valid arrangement is a placement of all k pieces on the grid with at most one piece per cell.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// The Manhattan Distance between two cells (xi, yi) and (xj, yj) is |xi - xj| + |yi - yj|.
//
// Example 1:
//
// Input: m = 2, n = 2, k = 2
//
// Output: 8
//
// Explanation:
//
// The valid arrangements of pieces on the board are:
//
//     In the first 4 arrangements, the Manhattan distance between the two pieces is 1.
//     In the last 2 arrangements, the Manhattan distance between the two pieces is 2.
//
// Thus, the total Manhattan distance across all valid arrangements is 1 + 1 + 1 + 1 + 2 + 2 = 8.
//
// Example 2:
//
// Input: m = 1, n = 4, k = 3
//
// Output: 20
//
// Explanation:
//
// The valid arrangements of pieces on the board are:
//
//     The first and last arrangements have a total Manhattan distance of 1 + 1 + 2 = 4.
//     The middle two arrangements have a total Manhattan distance of 1 + 2 + 3 = 6.
//
// The total Manhattan distance between all pairs of pieces across all arrangements is 4 + 6 + 6 + 4 = 20.
//
// Constraints:
//
//     1 <= m, n <= 10^5
//     2 <= m * n <= 10^5
//     2 <= k <= m * n
//

struct Solution;

impl Solution {
    pub fn distance_sum(m: i32, n: i32, k: i32) -> i32 {
        fn comb(a: i64, b: i64, m: i64) -> i64 {
            if b > a {
                return 0;
            }
            let mut numer = 1;
            let mut denom = 1;
            for i in 0..b {
                numer = numer * (a - i) % m;
                denom = denom * (i + 1) % m;
            }

            // Fermat's Little Theorem
            let mut denom_inv = 1;
            let mut exp = m - 2;
            while exp > 0 {
                if exp % 2 == 1 {
                    denom_inv = denom_inv * denom % m;
                }
                denom = denom * denom % m;
                exp /= 2;
            }
            numer * denom_inv % m
        }

        let (m, n, k) = (m as i64, n as i64, k as i64);
        let _mod = 1_000_000_007;
        let base = comb(m * n - 2, k - 2, _mod);
        let mut res = 0;
        for d in 1..n {
            res = (res + d * (n - d) % _mod * m % _mod * m % _mod) % _mod;
        }
        for d in 1..m {
            res = (res + d * (m - d) % _mod * n % _mod * n % _mod) % _mod;
        }
        (res * base % _mod) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::distance_sum(2, 2, 2), 8);
    assert_eq!(Solution::distance_sum(1, 4, 3), 20);
    assert_eq!(Solution::distance_sum(2, 3, 2), 25);
}
