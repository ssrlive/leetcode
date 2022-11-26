#![allow(dead_code)]

// 417. Pacific Atlantic Water Flow
// https://leetcode.com/problems/pacific-atlantic-water-flow/
//
// There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean.
// The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
//
// The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights
// where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
//
// The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south,
// east, and west if the neighboring cell's height is less than or equal to the current cell's height.
// Water can flow from any cell adjacent to an ocean into the ocean.
//
// Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water
// can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.
//
// Example 1:
//
// Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
// Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
// Explanation: The following cells can flow to the Pacific and Atlantic oceans, as shown below:
// [0,4]: [0,4] -> Pacific Ocean
//        [0,4] -> Atlantic Ocean
// [1,3]: [1,3] -> [0,3] -> Pacific Ocean
//        [1,3] -> [1,4] -> Atlantic Ocean
// [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean
//        [1,4] -> Atlantic Ocean
// [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean
//        [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
// [3,0]: [3,0] -> Pacific Ocean
//        [3,0] -> [4,0] -> Atlantic Ocean
// [3,1]: [3,1] -> [3,0] -> Pacific Ocean
//        [3,1] -> [4,1] -> Atlantic Ocean
// [4,0]: [4,0] -> Pacific Ocean
//        [4,0] -> Atlantic Ocean
// Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
//
// Example 2:
//
// Input: heights = [[1]]
// Output: [[0,0]]
// Explanation: The water can flow from the only cell to the Pacific and Atlantic oceans.
//
// Constraints:
//
// - m == heights.length
// - n == heights[r].length
// - 1 <= m, n <= 200
// - 0 <= heights[r][c] <= 105
//

struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];
        for i in 0..m {
            Self::dfs(&heights, i, 0, &mut pacific);
            Self::dfs(&heights, i, n - 1, &mut atlantic);
        }
        for j in 0..n {
            Self::dfs(&heights, 0, j, &mut pacific);
            Self::dfs(&heights, m - 1, j, &mut atlantic);
        }
        let mut res = vec![];
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }

    fn dfs(heights: &Vec<Vec<i32>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) {
        if visited[i][j] {
            return;
        }
        visited[i][j] = true;
        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0
                && ni < heights.len() as i32
                && nj >= 0
                && nj < heights[0].len() as i32
                && heights[ni as usize][nj as usize] >= heights[i][j]
            {
                Self::dfs(heights, ni as usize, nj as usize, visited);
            }
        }
    }
}

#[test]
fn test() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let res = vec![
        vec![0, 4],
        vec![1, 3],
        vec![1, 4],
        vec![2, 2],
        vec![3, 0],
        vec![3, 1],
        vec![4, 0],
    ];
    assert_eq!(Solution::pacific_atlantic(heights), res);
}
