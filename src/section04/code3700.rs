#![allow(dead_code)]

// 3700. Number of ZigZag Arrays II
// https://leetcode.com/problems/number-of-zigzag-arrays-ii/
// https://leetcode.cn/problems/number-of-zigzag-arrays-ii/
//
// Hard
//
// You are given three integers n, l, and r.
//
// A ZigZag array of length n is defined as follows:
//
// Each element lies in the range [l, r].
// No two adjacent elements are equal.
// No three consecutive elements form a strictly increasing or strictly decreasing sequence.
// Return the total number of valid ZigZag arrays.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// A sequence is said to be strictly increasing if each element is strictly greater than its previous one (if exists).
//
// A sequence is said to be strictly decreasing if each element is strictly smaller than its previous one (if exists).
//
// Example 1:
//
// Input: n = 3, l = 4, r = 5
//
// Output: 2
//
// Explanation:
//
// There are only 2 valid ZigZag arrays of length n = 3 using values in the range [4, 5]:
//
// [4, 5, 4]
// [5, 4, 5]
//
// Example 2:
//
// Input: n = 3, l = 1, r = 3
//
// Output: 10
//
// Explanation:
//
// ​​​​​​​There are 10 valid ZigZag arrays of length n = 3 using values in the range [1, 3]:
//
// [1, 2, 1], [1, 3, 1], [1, 3, 2]
// [2, 1, 2], [2, 1, 3], [2, 3, 1], [2, 3, 2]
// [3, 1, 2], [3, 1, 3], [3, 2, 3]
//
// All arrays meet the ZigZag conditions.
//
// Constraints:
//
// 3 <= n <= 10^9
// 1 <= l < r <= 75​​​​​​​
//

struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn matmul(a: &mut Vec<Vec<i64>>, b: &[Vec<i64>]) {
            let n = a.len();
            let m = a[0].len();
            let p = b[0].len();
            let mut c = vec![vec![0; p]; n];
            for i in 0..n {
                for j in 0..p {
                    for k in 0..m {
                        c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
                    }
                }
            }
            *a = c;
        }

        fn pow_mod(x: &mut Vec<Vec<i64>>, y: i32) -> Vec<Vec<i64>> {
            let n = x.len();
            let mut res = vec![vec![0; n]; n];
            for (i, res_i) in res.iter_mut().enumerate().take(n) {
                res_i[i] = 1;
            }
            let mut y = y;
            while y > 0 {
                if y & 1 == 1 {
                    matmul(&mut res, x);
                }
                y /= 2;
                let temp = x.clone();
                matmul(x, &temp);
            }
            res
        }

        let m = (r - l + 1) as usize;
        let mut a = vec![vec![0; 2 * m]; 2 * m];
        let dp = vec![vec![1; 1]; 2 * m];
        for i in 0..m {
            for j in 0..i {
                a[i][j + m] = 1;
            }
            a[i + m][(i + 1)..m].fill(1);
        }
        a = pow_mod(&mut a, n - 1);
        matmul(&mut a, &dp);

        let mut ans = 0;
        for row in &a {
            ans = (ans + row[0]) % MOD;
        }
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2);
    assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10);
}
