#![allow(dead_code)]

/*

// 1765. Map of Highest Peak
// https://leetcode.com/problems/map-of-highest-peak/
// https://leetcode.cn/problems/map-of-highest-peak/
//
// Medium
//
// You are given an integer matrix isWater of size m x n that represents a map of land and water cells.

    If isWater[i][j] == 0, cell (i, j) is a land cell.
    If isWater[i][j] == 1, cell (i, j) is a water cell.

You must assign each cell a height in a way that follows these rules:

    The height of each cell must be non-negative.
    If the cell is a water cell, its height must be 0.
    Any two adjacent cells must have an absolute height difference of at most 1. A cell is adjacent to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).

Find an assignment of heights such that the maximum height in the matrix is maximized.

Return an integer matrix height of size m x n where height[i][j] is cell (i, j)'s height. If there are multiple solutions, return any of them.

Example 1:

Input: isWater = [[0,1],[0,0]]
Output: [[1,0],[2,1]]
Explanation: The image shows the assigned heights of each cell.
The blue cell is the water cell, and the green cells are the land cells.

Example 2:

Input: isWater = [[0,0,1],[1,0,0],[0,0,0]]
Output: [[1,1,0],[0,1,1],[1,2,2]]
Explanation: A height of 2 is the maximum possible height of any assignment.
Any height assignment that has a maximum height of 2 while still meeting the rules will also be accepted.

Constraints:

    m == isWater.length
    n == isWater[i].length
    1 <= m, n <= 1000
    isWater[i][j] is 0 or 1.
    There is at least one water cell.
*/

struct Solution;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut height = vec![vec![-1; is_water[0].len()]; is_water.len()];
        let mut queue = Vec::new();
        for i in 0..is_water.len() {
            for j in 0..is_water[0].len() {
                if is_water[i][j] == 1 {
                    height[i][j] = 0;
                    queue.push((i as i32, j as i32));
                }
            }
        }
        let mut level = 1;
        while !queue.is_empty() {
            let mut next = Vec::new();
            for (i, j) in queue {
                for &(x, y) in &[(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                    if x >= 0
                        && x < is_water.len() as i32
                        && y >= 0
                        && y < is_water[0].len() as i32
                        && height[x as usize][y as usize] == -1
                    {
                        height[x as usize][y as usize] = level;
                        next.push((x, y));
                    }
                }
            }
            queue = next;
            level += 1;
        }
        height
    }
}

#[test]
fn test() {
    let is_water = vec![vec![0, 1], vec![0, 0]];
    let height = vec![vec![1, 0], vec![2, 1]];
    assert_eq!(Solution::highest_peak(is_water), height);
    let is_water = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
    let height = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]];
    assert_eq!(Solution::highest_peak(is_water), height);
}
