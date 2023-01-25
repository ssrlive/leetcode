#![allow(dead_code)]

// 1314. Matrix Block Sum
// https://leetcode.com/problems/matrix-block-sum/
// https://leetcode.cn/problems/matrix-block-sum/
//
// Medium
//
// Given a m x n matrix mat and an integer k, return a matrix answer where each answer[i][j]
// is the sum of all elements mat[r][c] for:
//
//     i - k <= r <= i + k,
//     j - k <= c <= j + k, and
//     (r, c) is a valid position in the matrix.
//
// Example 1:
//
// Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
// Output: [[12,21,16],[27,45,33],[24,39,28]]
//
// Example 2:
//
// Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 2
// Output: [[45,45,45],[45,45,45],[45,45,45]]
//
// Constraints:
//
// -    m == mat.length
// -    n == mat[i].length
// -    1 <= m, n, k <= 100
// -    1 <= mat[i][j] <= 100
//

struct Solution;

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut sum = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + mat[i][j];
            }
        }
        let mut ans = vec![vec![0; n]; m];
        for (i, item) in ans.iter_mut().enumerate().take(m) {
            for (j, item2) in item.iter_mut().enumerate().take(n) {
                let r1 = (i as i32 - k).max(0) as usize;
                let c1 = (j as i32 - k).max(0) as usize;
                let r2 = (i as i32 + k + 1).min(m as i32) as usize;
                let c2 = (j as i32 + k + 1).min(n as i32) as usize;
                *item2 = sum[r2][c2] - sum[r2][c1] - sum[r1][c2] + sum[r1][c1];
            }
        }
        ans
    }
}

#[test]
fn test() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let k = 1;
    let ans = vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]];
    assert_eq!(Solution::matrix_block_sum(mat, k), ans);

    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let k = 2;
    let ans = vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]];
    assert_eq!(Solution::matrix_block_sum(mat, k), ans);
}
