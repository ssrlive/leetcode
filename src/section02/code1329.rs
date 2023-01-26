#![allow(dead_code)]

// 1329. Sort the Matrix Diagonally
// https://leetcode.com/problems/sort-the-matrix-diagonally/
// https://leetcode.cn/problems/sort-the-matrix-diagonally/
//
// Medium
//
// A matrix diagonal is a diagonal line of cells starting from some cell in either the topmost row or leftmost column and
// going in the bottom-right direction until reaching the matrix's end.
// For example, the matrix diagonal starting from mat[2][0], where mat is a 6 x 3 matrix, includes cells mat[2][0], mat[3][1], and mat[4][2].
//
// Given an m x n matrix mat of integers, sort each matrix diagonal in ascending order and return the resulting matrix.
//
// Example 1:
//
// Input: mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]
// Output: [[1,1,1,1],[1,2,2,2],[1,2,3,3]]
//
// Example 2:
//
// Input: mat = [[11,25,66,1,69,7],[23,55,17,45,15,52],[75,31,36,44,58,8],[22,27,33,25,68,4],[84,28,14,11,5,50]]
// Output: [[5,17,4,1,52,7],[11,11,25,45,8,69],[14,23,25,44,58,15],[22,27,31,36,50,66],[84,28,75,33,55,68]]
//
// Constraints:
//
// -    m == mat.length
// -    n == mat[i].length
// -    1 <= m, n <= 100
// -    1 <= mat[i][j] <= 100
//

struct Solution;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let m = mat.len();
        let n = mat[0].len();
        for i in 0..m {
            let mut j = 0;
            let mut k = i;
            let mut v = Vec::new();
            while k < m && j < n {
                v.push(mat[k][j]);
                k += 1;
                j += 1;
            }
            v.sort();
            let mut j = 0;
            let mut k = i;
            while k < m && j < n {
                mat[k][j] = v[j];
                k += 1;
                j += 1;
            }
        }
        for j in 1..n {
            let mut i = 0;
            let mut k = j;
            let mut v = Vec::new();
            while i < m && k < n {
                v.push(mat[i][k]);
                i += 1;
                k += 1;
            }
            v.sort();
            let mut i = 0;
            let mut k = j;
            while i < m && k < n {
                mat[i][k] = v[i];
                i += 1;
                k += 1;
            }
        }
        mat
    }
}

#[test]
fn test() {
    let mat = vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]];
    let res = vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]];
    assert_eq!(Solution::diagonal_sort(mat), res);
    let mat = vec![
        vec![11, 25, 66, 1, 69, 7],
        vec![23, 55, 17, 45, 15, 52],
        vec![75, 31, 36, 44, 58, 8],
        vec![22, 27, 33, 25, 68, 4],
        vec![84, 28, 14, 11, 5, 50],
    ];
    let res = vec![
        vec![5, 17, 4, 1, 52, 7],
        vec![11, 11, 25, 45, 8, 69],
        vec![14, 23, 25, 44, 58, 15],
        vec![22, 27, 31, 36, 50, 66],
        vec![84, 28, 75, 33, 55, 68],
    ];
    assert_eq!(Solution::diagonal_sort(mat), res);
}
