#![allow(dead_code)]

// 1420. Build Array Where You Can Find The Maximum Exactly K Comparisons
// https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
// https://leetcode.cn/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
//
// Hard
//
// You are given three integers n, m and k. Consider the following algorithm to find the maximum element of an array of positive integers:
//
// You should build the array arr which has the following properties:
//
//     arr has exactly n integers.
//     1 <= arr[i] <= m where (0 <= i < n).
//     After applying the mentioned algorithm to arr, the value search_cost is equal to k.
//
// Return the number of ways to build the array arr under the mentioned conditions.
// As the answer may grow large, the answer must be computed modulo 109 + 7.
//
// Example 1:
//
// Input: n = 2, m = 3, k = 1
// Output: 6
// Explanation: The possible arrays are [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
//
// Example 2:
//
// Input: n = 5, m = 2, k = 3
// Output: 0
// Explanation: There are no possible arrays that satisify the mentioned conditions.
//
// Example 3:
//
// Input: n = 9, m = 1, k = 1
// Output: 1
// Explanation: The only possible array is [1, 1, 1, 1, 1, 1, 1, 1, 1]
//
// Constraints:
//
// -    1 <= n <= 50
// -    1 <= m <= 100
// -    0 <= k <= n
//

struct Solution;

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let k = k as usize;
        if m < k {
            return 0;
        }
        let mut dp = vec![vec![vec![0; k + 1]; m + 1]; 2];
        let mod_num = 1_000_000_007;
        for (j, row_j) in dp[0].iter_mut().enumerate().skip(1).take(m) {
            row_j[1] = j as i64;
        }
        for i in 1..n {
            for j in 1..=m {
                for l in 1..=std::cmp::min(i + 1, std::cmp::min(j, k)) {
                    dp[i & 1][j][l] = (dp[i & 1][j - 1][l]
                        + (dp[(i - 1) & 1][j][l] - dp[(i - 1) & 1][j - 1][l]) * j as i64
                        + dp[(i - 1) & 1][j - 1][l - 1])
                        % mod_num;
                }
            }
        }
        ((dp[(n - 1) & 1][m][k] + mod_num) % mod_num) as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, 3, 1, 6),
        (5, 2, 3, 0),
        (9, 1, 1, 1),
        (50, 100, 25, 34549172),
        (37, 17, 7, 418930126),
    ];
    for (n, m, k, expected) in cases {
        assert_eq!(Solution::num_of_arrays(n, m, k), expected);
    }
}
