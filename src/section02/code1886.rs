#![allow(dead_code)]

/*

// 1886. Determine Whether Matrix Can Be Obtained By Rotation
// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/
// https://leetcode.cn/problems/determine-whether-matrix-can-be-obtained-by-rotation/
//
// Easy
//
// Given two n x n binary matrices mat and target, return true if it is possible to make mat equal to target by rotating mat in 90-degree increments, or false otherwise.

Example 1:

Input: mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
Output: true
Explanation: We can rotate mat 90 degrees clockwise to make mat equal target.

Example 2:

Input: mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
Output: false
Explanation: It is impossible to make mat equal to target by rotating mat.

Example 3:

Input: mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
Output: true
Explanation: We can rotate mat 90 degrees clockwise two times to make mat equal target.

Constraints:

    n == mat.length == target.length
    n == mat[i].length == target[i].length
    1 <= n <= 10
    mat[i][j] and target[i][j] are either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len() - 1;
        let mut arr_counter_rotate = [0; 4];
        mat.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &val)| {
                arr_counter_rotate[0] += (val == target[i][j]) as usize;
                arr_counter_rotate[1] += (val == target[j][n - i]) as usize;
                arr_counter_rotate[2] += (val == target[n - i][n - j]) as usize;
                arr_counter_rotate[3] += (val == target[n - j][i]) as usize;
            })
        });
        arr_counter_rotate.iter().any(|x| *x == (n + 1).pow(2))
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 1], vec![1, 0]], vec![vec![1, 0], vec![0, 1]], true),
        (vec![vec![0, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 1]], false),
        (
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]],
            true,
        ),
    ];
    for (mat, target, expected) in cases {
        assert_eq!(Solution::find_rotation(mat, target), expected);
    }
}
