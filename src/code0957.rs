#![allow(dead_code)]

// 957. Prison Cells After N Days
// https://leetcode.com/problems/prison-cells-after-n-days/
// https://leetcode.cn/problems/prison-cells-after-n-days/
//
// There are 8 prison cells in a row and each cell is either occupied or vacant.
//
// Each day, whether the cell is occupied or vacant changes according to the following rules:
//
// If a cell has two adjacent neighbors that are both occupied or both vacant, then the cell becomes occupied.
// Otherwise, it becomes vacant.
// Note that because the prison is a row, the first and the last cells in the row can't have two adjacent neighbors.
//
// You are given an integer array cells where cells[i] == 1 if the ith cell is occupied
// and cells[i] == 0 if the ith cell is vacant, and you are given an integer n.
//
// Return the state of the prison after n days (i.e., n such changes described above).
//
// Example 1:
//
// Input: cells = [0,1,0,1,1,0,0,1], n = 7
// Output: [0,0,1,1,0,0,0,0]
// Explanation: The following table summarizes the state of the prison on each day:
// Day 0: [0, 1, 0, 1, 1, 0, 0, 1]
// Day 1: [0, 1, 1, 0, 0, 0, 0, 0]
// Day 2: [0, 0, 0, 0, 1, 1, 1, 0]
// Day 3: [0, 1, 1, 0, 0, 1, 0, 0]
// Day 4: [0, 0, 0, 0, 0, 1, 0, 0]
// Day 5: [0, 1, 1, 1, 0, 1, 0, 0]
// Day 6: [0, 0, 1, 0, 1, 1, 0, 0]
// Day 7: [0, 0, 1, 1, 0, 0, 0, 0]
//
// Example 2:
//
// Input: cells = [1,0,0,1,0,0,1,0], n = 1000000000
// Output: [0,0,1,1,1,1,1,0]
//
// Constraints:
//
// - cells.length == 8
// - cells[i] is either 0 or 1.
// - 1 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut cells: Vec<i32> = cells;
        for _ in 0..(n - 1) % 14 + 1 {
            cells = (0..cells.len())
                .map(|i| i32::from(i > 0 && i < cells.len() - 1 && cells[i - 1] == cells[i + 1]))
                .collect();
        }
        cells
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![0, 1, 0, 1, 1, 0, 0, 1], 7, vec![0, 0, 1, 1, 0, 0, 0, 0]),
        (vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000, vec![0, 0, 1, 1, 1, 1, 1, 0]),
    ];
    for (cells, n, expected) in cases {
        assert_eq!(Solution::prison_after_n_days(cells, n), expected);
    }
}
