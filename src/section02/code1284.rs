#![allow(dead_code)]

// 1284. Minimum Number of Flips to Convert Binary Matrix to Zero Matrix
// https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/
// https://leetcode.cn/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/
//
// Hard
//
// Given a m x n binary matrix mat. In one step, you can choose one cell and flip it and all the four neighbors of
// it if they exist (Flip is changing 1 to 0 and 0 to 1). A pair of cells are called neighbors if they share one edge.
//
// Return the minimum number of steps required to convert mat to a zero matrix or -1 if you cannot.
//
// A binary matrix is a matrix with all cells equal to 0 or 1 only.
//
// A zero matrix is a matrix with all cells equal to 0.
//
// Example 1:
//
// Input: mat = [[0,0],[0,1]]
// Output: 3
// Explanation: One possible solution is to flip (1, 0) then (0, 1) and finally (1, 1) as shown.
//
// Example 2:
//
// Input: mat = [[0]]
// Output: 0
// Explanation: Given matrix is a zero matrix. We do not need to change it.
//
// Example 3:
//
// Input: mat = [[1,0,0],[1,0,0]]
// Output: -1
// Explanation: Given matrix cannot be a zero matrix.
//
// Constraints:
//
// -    m == mat.length
// -    n == mat[i].length
// -    1 <= m, n <= 3
// -    mat[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        type Matrix = Vec<Vec<bool>>;

        fn is_zero(mat: &Matrix) -> bool {
            mat.iter().all(|row| row.iter().all(|x| !x))
        }

        fn flip(position: i32, mat: &mut Matrix) {
            let x = (position / (mat.len() as i32)) as usize;
            let y = (position % (mat.len() as i32)) as usize;
            mat[y][x] = !mat[y][x];
            if y >= 1 {
                mat[y - 1][x] = !mat[y - 1][x];
            }
            if y < mat.len() - 1 {
                mat[y + 1][x] = !mat[y + 1][x];
            }
            if x >= 1 {
                mat[y][x - 1] = !mat[y][x - 1];
            }
            if x < mat[0].len() - 1 {
                mat[y][x + 1] = !mat[y][x + 1];
            }
        }

        fn _min_flips(position: i32, mat: &mut Matrix) -> Option<i32> {
            // edge case
            if position as usize == mat.len() * mat[0].len() {
                if is_zero(mat) {
                    return Some(0);
                } else {
                    return None;
                }
            }

            // don't flip this one
            let result_0 = _min_flips(position + 1, mat);
            // flip the position
            flip(position, mat);
            let result_1 = _min_flips(position + 1, mat);
            // flip back
            flip(position, mat);

            if let Some(result_0) = result_0 {
                if let Some(result_1) = result_1 {
                    return Some(result_0.min(1 + result_1));
                } else {
                    return Some(result_0);
                }
            } else if let Some(result_1) = result_1 {
                return Some(1 + result_1);
            }

            None
        }

        let mat = &mut mat
            .iter()
            .map(|row| row.iter().map(|x| *x == 1).collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();

        if let Some(x) = _min_flips(0, mat) {
            x
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 0], vec![0, 1]], 3),
        (vec![vec![0]], 0),
        (vec![vec![1, 0, 0], vec![1, 0, 0]], -1),
    ];
    for (mat, expected) in cases {
        assert_eq!(Solution::min_flips(mat), expected);
    }
}
