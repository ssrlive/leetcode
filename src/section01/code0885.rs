#![allow(dead_code)]

// 885. Spiral Matrix III
// https://leetcode.com/problems/spiral-matrix-iii/
// https://leetcode.cn/problems/spiral-matrix-iii/
//
// You start at the cell (rStart, cStart) of an rows x cols grid facing east.
// The northwest corner is at the first row and column in the grid, and the southeast corner is at the last row and column.
//
// You will walk in a clockwise spiral shape to visit every position in this grid.
// Whenever you move outside the grid's boundary, we continue our walk outside the grid (but may return to the grid boundary later.).
// Eventually, we reach all rows * cols spaces of the grid.
//
// Return an array of coordinates representing the positions of the grid in the order you visited them.
//
// Example 1:
//
// Input: rows = 1, cols = 4, rStart = 0, cStart = 0
// Output: [[0,0],[0,1],[0,2],[0,3]]
//
// Example 2:
//
// Input: rows = 5, cols = 6, rStart = 1, cStart = 4
// Output: [[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]
//
// Constraints:
//
// - 1 <= rows, cols <= 100
// - 0 <= rStart < rows
// - 0 <= cStart < cols
//

struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let total = rows * cols;
        let mut cnt = 1;
        let mut step = 1;
        let mut i = 0;
        let mut r_start = r_start;
        let mut c_start = c_start;
        ans.push(vec![r_start, c_start]);
        let direction = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while cnt < total {
            for _ in 0..step {
                r_start += direction[i].0;
                c_start += direction[i].1;
                if 0 <= r_start && r_start < rows && 0 <= c_start && c_start < cols {
                    ans.push(vec![r_start, c_start]);
                    cnt += 1;
                }
            }
            i = (i + 1) % 4;
            step += i32::from(i.is_multiple_of(2));
        }
        ans
    }
}

#[test]
fn test() {
    let rows = 1;
    let cols = 4;
    let r_start = 0;
    let c_start = 0;
    let ans = vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]];
    assert_eq!(Solution::spiral_matrix_iii(rows, cols, r_start, c_start), ans);

    let rows = 5;
    let cols = 6;
    let r_start = 1;
    let c_start = 4;
    let ans = vec![
        vec![1, 4],
        vec![1, 5],
        vec![2, 5],
        vec![2, 4],
        vec![2, 3],
        vec![1, 3],
        vec![0, 3],
        vec![0, 4],
        vec![0, 5],
        vec![3, 5],
        vec![3, 4],
        vec![3, 3],
        vec![3, 2],
        vec![2, 2],
        vec![1, 2],
        vec![0, 2],
        vec![4, 5],
        vec![4, 4],
        vec![4, 3],
        vec![4, 2],
        vec![4, 1],
        vec![3, 1],
        vec![2, 1],
        vec![1, 1],
        vec![0, 1],
        vec![4, 0],
        vec![3, 0],
        vec![2, 0],
        vec![1, 0],
        vec![0, 0],
    ];
    assert_eq!(Solution::spiral_matrix_iii(rows, cols, r_start, c_start), ans);
}
