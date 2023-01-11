#![allow(dead_code)]

// 1030. Matrix Cells in Distance Order
// https://leetcode.com/problems/matrix-cells-in-distance-order/
// https://leetcode.cn/problems/matrix-cells-in-distance-order/
//
// You are given four integers row, cols, rCenter, and cCenter. There is a rows x cols matrix and you are on the cell with the coordinates (rCenter, cCenter).
//
// Return the coordinates of all cells in the matrix, sorted by their distance from (rCenter, cCenter) from the smallest distance to the largest distance. You may return the answer in any order that satisfies this condition.
//
// The distance between two cells (r1, c1) and (r2, c2) is |r1 - r2| + |c1 - c2|.
//
// Example 1:
//
// Input: rows = 1, cols = 2, rCenter = 0, cCenter = 0
// Output: [[0,0],[0,1]]
// Explanation: The distances from (0, 0) to other cells are: [0,1]
//
// Example 2:
//
// Input: rows = 2, cols = 2, rCenter = 0, cCenter = 1
// Output: [[0,1],[0,0],[1,1],[1,0]]
// Explanation: The distances from (0, 1) to other cells are: [0,1,1,2]
// The answer [[0,1],[1,1],[0,0],[1,0]] would also be accepted as correct.
//
// Example 3:
//
// Input: rows = 2, cols = 3, rCenter = 1, cCenter = 2
// Output: [[1,2],[0,2],[1,1],[0,1],[1,0],[0,0]]
// Explanation: The distances from (1, 2) to other cells are: [0,1,1,2,2,3]
// There are other answers that would also be accepted as correct, such as [[1,2],[1,1],[0,2],[1,0],[0,1],[0,0]].
//
// Constraints:
//
// - 1 <= rows, cols <= 100
// - 0 <= rCenter < rows
// - 0 <= cCenter < cols
//

struct Solution;

impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity((rows * cols) as usize);
        for r in 0..rows {
            for c in 0..cols {
                res.push(vec![r, c]);
            }
        }
        res.sort_unstable_by_key(|v| (v[0] - r_center).abs() + (v[1] - c_center).abs());
        res
    }
}

#[test]
fn test() {
    let cased = vec![
        (1, 2, 0, 0, vec![vec![0, 0], vec![0, 1]]),
        (2, 2, 0, 1, vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]),
        (
            2,
            3,
            1,
            2,
            vec![vec![1, 2], vec![0, 2], vec![1, 1], vec![0, 1], vec![1, 0], vec![0, 0]],
        ),
    ];
    for (rows, cols, r_center, c_center, expect) in cased {
        let res = Solution::all_cells_dist_order(rows, cols, r_center, c_center);
        assert_eq!(res, expect);
    }
}
