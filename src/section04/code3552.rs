#![allow(dead_code)]

// 3552. Grid Teleportation Traversal
// https://leetcode.com/problems/grid-teleportation-traversal/
// https://leetcode.cn/problems/grid-teleportation-traversal/
//
// Medium
//
// You are given a 2D character grid matrix of size m x n, represented as an array of strings, where matrix[i][j]
// represents the cell at the intersection of the ith row and jth column. Each cell is one of the following:
//
//     '.' representing an empty cell.
//     '#' representing an obstacle.
//     An uppercase letter ('A'-'Z') representing a teleportation portal.
//
// You start at the top-left cell (0, 0), and your goal is to reach the bottom-right cell (m - 1, n - 1).
// You can move from the current cell to any adjacent cell (up, down, left, right) as long as the
// destination cell is within the grid bounds and is not an obstacle.
//
// If you step on a cell containing a portal letter and you haven't used that portal letter before,
// you may instantly teleport to any other cell in the grid with the same letter. This teleportation
// does not count as a move, but each portal letter can be used at most once during your journey.
//
// Return the minimum number of moves required to reach the bottom-right cell. If it is not possible to reach the destination, return -1.
//
// Example 1:
//
// Input: matrix = ["A..",".A.","..."]
//
// Output: 2
//
// Explanation:
//
//     Before the first move, teleport from (0, 0) to (1, 1).
//     In the first move, move from (1, 1) to (1, 2).
//     In the second move, move from (1, 2) to (2, 2).
//
// Example 2:
//
// Input: matrix = [".#...",".#.#.",".#.#.","...#."]
//
// Output: 13
//
// Explanation:
//
// Constraints:
//
//     1 <= m == matrix.length <= 10^3
//     1 <= n == matrix[i].length <= 10^3
//     matrix[i][j] is either '#', '.', or an uppercase English letter.
//     matrix[0][0] is not an obstacle.
//

struct Solution;

impl Solution {
    pub fn min_moves(matrix: Vec<String>) -> i32 {
        fn solve(mat: &[Vec<char>], pos: &[Vec<(i32, i32)>]) -> i32 {
            let n = mat.len() as i32;
            let m = mat[0].len() as i32;
            if mat[0][0] == '#' || mat[n as usize - 1][m as usize - 1] == '#' {
                return -1;
            }

            let mut dist = vec![vec![i32::MAX; m as usize]; n as usize];
            let mut used = [false; 26];
            let mut q = std::collections::VecDeque::new();
            dist[0][0] = 0;
            q.push_front((0, 0));

            let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                let d = dist[x as usize][y as usize];
                if x == n - 1 && y == m - 1 {
                    return d;
                }

                let c = mat[x as usize][y as usize];
                if c.is_ascii_uppercase() {
                    let idx = (c as u8 - b'A') as usize;
                    if !used[idx] {
                        used[idx] = true;
                        for &(px, py) in &pos[idx] {
                            if d < dist[px as usize][py as usize] {
                                dist[px as usize][py as usize] = d;
                                q.push_front((px, py));
                            }
                        }
                    }
                }

                for &(dx, dy) in &dirs {
                    let new_x = x + dx;
                    let new_y = y + dy;
                    if new_x >= 0
                        && new_x < n
                        && new_y >= 0
                        && new_y < m
                        && mat[new_x as usize][new_y as usize] != '#'
                        && d + 1 < dist[new_x as usize][new_y as usize]
                    {
                        dist[new_x as usize][new_y as usize] = d + 1;
                        q.push_back((new_x, new_y));
                    }
                }
            }
            -1
        }

        let n = matrix.len() as i32;
        let m = matrix[0].len() as i32;
        let mut pos = vec![vec![]; 26];
        let mut mat = vec![vec![' '; m as usize]; n as usize];
        for i in 0..n {
            mat[i as usize] = matrix[i as usize].chars().collect();
            for j in 0..m {
                let c = mat[i as usize][j as usize];
                if c.is_ascii_uppercase() {
                    pos[(c as u8 - b'A') as usize].push((i, j));
                }
            }
        }
        solve(&mat, &pos)
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_moves(vec!["A..".to_string(), ".A.".to_string(), "...".to_string()]),
        2
    );
    assert_eq!(
        Solution::min_moves(vec![
            ".#...".to_string(),
            ".#.#.".to_string(),
            ".#.#.".to_string(),
            "...#.".to_string()
        ]),
        13
    );
}
