#![allow(dead_code)]

// 1292. Maximum Side Length of a Square with Sum Less than or Equal to Threshold
// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/
// https://leetcode.cn/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/
//
// Medium
//
// Given a m x n matrix mat and an integer threshold, return the maximum side-length of a square
// with a sum less than or equal to threshold or return 0 if there is no such square.
//
// Example 1:
//
// Input: mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]], threshold = 4
// Output: 2
// Explanation: The maximum side length of square with sum less than 4 is 2 as shown.
//
// Example 2:
//
// Input: mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]], threshold = 1
// Output: 0
//
// Constraints:
//
// -    m == mat.length
// -    n == mat[i].length
// -    1 <= m, n <= 300
// -    0 <= mat[i][j] <= 10^4
// -    0 <= threshold <= 10^5
//

struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                dp[i + 1][j + 1] = dp[i][j + 1] + dp[i + 1][j] - dp[i][j] + mat[i][j];
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                let mut left = 0;
                let mut right = std::cmp::min(m - i, n - j);
                while left < right {
                    let mid = (left + right + 1) / 2;
                    let sum = dp[i + mid][j + mid] - dp[i + mid][j] - dp[i][j + mid] + dp[i][j];
                    if sum <= threshold {
                        left = mid;
                    } else {
                        right = mid - 1;
                    }
                }
                ans = std::cmp::max(ans, left);
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2],
            ],
            4,
            2,
        ),
        (
            vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
            ],
            1,
            0,
        ),
    ];
    for (mat, threshold, expect) in cases {
        assert_eq!(Solution::max_side_length(mat, threshold), expect);
    }
}
