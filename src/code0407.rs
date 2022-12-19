#![allow(dead_code)]

// 407. Trapping Rain Water II
// https://leetcode.com/problems/trapping-rain-water-ii/
// https://leetcode.cn/problems/trapping-rain-water-ii/
//
// Given an m x n matrix of positive integers representing the height of each unit cell in a 2D
// elevation map, compute the volume of water it is able to trap after raining.
//
// Note:
// Both m and n are less than 110. The height of each unit cell is greater than 0 and is less than 20,000.
//
// Example:
//
// Given the following 3x6 height map:
// [
//   [1,4,3,1,3,2],
//   [3,2,1,3,2,4],
//   [2,3,3,2,3,1]
// ]
//
// Return 4.
//
// The above image represents the elevation map [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]] before the rain.
//
// After the rain, water is trapped between the blocks. The total volume of water trapped is 4.
//
// Constraints:
//
// - 1 <= m, n <= 110
// - 0 <= heightMap[i][j] <= 20000
//

struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let m = height_map.len();
        let n = height_map[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut heap = BinaryHeap::new();
        for i in 0..m {
            heap.push(Reverse((height_map[i][0], i, 0)));
            heap.push(Reverse((height_map[i][n - 1], i, n - 1)));
            visited[i][0] = true;
            visited[i][n - 1] = true;
        }
        for j in 1..n - 1 {
            heap.push(Reverse((height_map[0][j], 0, j)));
            heap.push(Reverse((height_map[m - 1][j], m - 1, j)));
            visited[0][j] = true;
            visited[m - 1][j] = true;
        }
        let mut res = 0;
        while let Some(Reverse((h, i, j))) = heap.pop() {
            for &(di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if !visited[ni][nj] {
                        visited[ni][nj] = true;
                        res += h.max(height_map[ni][nj]) - height_map[ni][nj];
                        heap.push(Reverse((h.max(height_map[ni][nj]), ni, nj)));
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let height_map = vec![vec![1, 4, 3, 1, 3, 2], vec![3, 2, 1, 3, 2, 4], vec![2, 3, 3, 2, 3, 1]];
    assert_eq!(Solution::trap_rain_water(height_map), 4);
}
