#![allow(dead_code)]

/*

// 1591. Strange Printer II
Hard
484
15
Companies

There is a strange printer with the following two special requirements:

    On each turn, the printer will print a solid rectangular pattern of a single color on the grid. This will cover up the existing colors in the rectangle.
    Once the printer has used a color for the above operation, the same color cannot be used again.

You are given a m x n matrix targetGrid, where targetGrid[row][col] is the color in the position (row, col) of the grid.

Return true if it is possible to print the matrix targetGrid, otherwise, return false.

Example 1:

Input: targetGrid = [[1,1,1,1],[1,2,2,1],[1,2,2,1],[1,1,1,1]]
Output: true

Example 2:

Input: targetGrid = [[1,1,1,1],[1,1,3,3],[1,1,3,4],[5,5,1,4]]
Output: true

Example 3:

Input: targetGrid = [[1,2,1],[2,1,2],[1,2,1]]
Output: false
Explanation: It is impossible to form targetGrid because it is not allowed to print the same color in different turns.

Constraints:

    m == targetGrid.length
    n == targetGrid[i].length
    1 <= m, n <= 60
    1 <= targetGrid[row][col] <= 60
*/

struct Solution;

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        use std::collections::HashSet;

        fn has_circle(curr: usize, dep: &Vec<HashSet<i32>>, vis: &mut Vec<i32>) -> bool {
            vis[curr] = 1;
            for &d in dep[curr].iter() {
                let d = d as usize;
                if vis[d] == 2 {
                    continue;
                }
                if vis[d] == 1 {
                    return true;
                }
                if has_circle(d, dep, vis) {
                    return true;
                }
            }
            vis[curr] = 2;
            false
        }

        let (m, n) = (target_grid.len() as i32, target_grid[0].len() as i32);
        let mut dep = vec![HashSet::new(); 61];
        for i in 1..=60 {
            let (mut minx, mut miny, mut maxx, mut maxy) = (m, n, -1, -1);
            for x in 0..m {
                for y in 0..n {
                    if target_grid[x as usize][y as usize] == i {
                        minx = minx.min(x);
                        miny = miny.min(y);
                        maxx = maxx.max(x);
                        maxy = maxy.max(y);
                    }
                }
            }
            for tx in minx..=maxx {
                for ty in miny..=maxy {
                    if target_grid[tx as usize][ty as usize] != i {
                        dep[i as usize].insert(target_grid[tx as usize][ty as usize]);
                    }
                }
            }
        }
        let mut vis = vec![0; 61];
        for i in 1..=60 {
            if vis[i] == 0 && has_circle(i, &dep, &mut vis) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let target_grid = vec![vec![1, 1, 1, 1], vec![1, 2, 2, 1], vec![1, 2, 2, 1], vec![1, 1, 1, 1]];
    assert_eq!(Solution::is_printable(target_grid), true);
    let target_grid = vec![vec![1, 1, 1, 1], vec![1, 1, 3, 3], vec![1, 1, 3, 4], vec![5, 5, 1, 4]];
    assert_eq!(Solution::is_printable(target_grid), true);
    let target_grid = vec![vec![1, 2, 1], vec![2, 1, 2], vec![1, 2, 1]];
    assert_eq!(Solution::is_printable(target_grid), false);
}
