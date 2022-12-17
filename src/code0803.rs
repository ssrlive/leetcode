#![allow(dead_code)]

// 803. Bricks Falling When Hit
// https://leetcode.com/problems/bricks-falling-when-hit/
//
// You are given an m x n binary grid, where each 1 represents a brick and 0 represents an empty space. A brick is stable if:
//
// - It is directly connected to the top of the grid, or
// - At least one other brick in its four adjacent cells is stable.
//
// You are also given an array hits, which is a sequence of erasures we want to apply. Each time we want to erase the brick at the location hits[i] = (rowi, coli).
// The brick on that location (if it exists) will disappear. Some other bricks may no longer be stable because of that erasure and will fall. Once a brick falls,
// it is immediately erased from the grid (i.e., it does not land on other stable bricks).
//
// Return an array result, where each result[i] is the number of bricks that will fall after the ith erasure is applied.
//
// Note that an erasure may refer to a location with no brick, and if it does, no bricks drop.
//
// Example 1:
//
// Input: grid = [[1,0,0,0],[1,1,1,0]], hits = [[1,0]]
// Output: [2]
// Explanation: Starting with the grid:
// [[1,0,0,0],
//  [1,1,1,0]]
// We erase the underlined brick at (1,0), resulting in the grid:
// [[1,0,0,0],
//  [0,1,1,0]]
// The two underlined bricks are no longer stable as they are no longer connected to the top nor adjacent to another stable brick,
// so they will fall. The resulting grid is:
// [[1,0,0,0],
//  [0,0,0,0]]
// Hence the result is [2].
//
// Example 2:
//
// Input: grid = [[1,0,0,0],[1,1,0,0]], hits = [[1,1],[1,0]]
// Output: [0,0]
// Explanation: Starting with the grid:
// [[1,0,0,0],
//  [1,1,0,0]]
// We erase the underlined brick at (1,1), resulting in the grid:
// [[1,0,0,0],
//  [1,0,0,0]]
// All remaining bricks are still stable, so no bricks fall. The grid remains the same:
// [[1,0,0,0],
//  [1,0,0,0]]
// Next, we erase the underlined brick at (1,0), resulting in the grid:
// [[1,0,0,0],
//  [0,0,0,0]]
// Once again, all remaining bricks are still stable, so no bricks fall.
// Hence the result is [0,0].
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 200
// - grid[i][j] is 0 or 1.
// - 1 <= hits.length <= 4 * 10^4
// - hits[i].length == 2
// - 0 <= xi <= m - 1
// - 0 <= yi <= n - 1
// - All (xi, yi) are unique.
//

struct Solution;

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        fn find_parents(parents: &mut Vec<usize>, i: usize) -> usize {
            if parents[i] != i {
                parents[i] = find_parents(parents, parents[i]);
            }
            parents[i]
        }

        fn union2(parents: &mut Vec<usize>, _size: &mut [usize], i: usize, j: usize) {
            let mut pi = find_parents(parents, i);
            let mut pj = find_parents(parents, j);
            if pi != pj {
                if pi > pj {
                    std::mem::swap(&mut pi, &mut pj);
                }
                _size[pi] += _size[pj];
                parents[pj] = pi;
            }
        }

        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut parents = vec![0; m * n];
        let mut _size = vec![1; m * n];
        let mut ret = vec![0; hits.len()];
        let dirs = vec![vec![0, 1], vec![0, -1], vec![-1, 0], vec![1, 0]];
        let c21 = |i: usize, j: usize| -> usize { i * n + j };
        for h in hits.iter() {
            grid[h[0] as usize][h[1] as usize] -= 1;
        }
        for (i, item) in parents.iter_mut().enumerate() {
            *item = i;
        }
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    for item in dirs.iter().take(4) {
                        let x = i as i32 + item[0];
                        let y = j as i32 + item[1];
                        if x > -1 && x < m as i32 && y > -1 && y < n as i32 && grid[x as usize][y as usize] > 0 {
                            union2(&mut parents, &mut _size, c21(i, j), c21(x as usize, y as usize));
                        }
                    }
                }
            }
        }
        for i in (0..hits.len()).rev() {
            let point = &mut grid[hits[i][0] as usize][hits[i][1] as usize];
            *point += 1;
            if *point > 0 {
                let _i = hits[i][0] as usize;
                let _j = hits[i][1] as usize;
                let mut flag = usize::from(_i == 0);
                for item in dirs.iter().take(4) {
                    let x = _i as i32 + item[0];
                    let y = _j as i32 + item[1];
                    if x > -1 && x < m as i32 && y > -1 && y < n as i32 && grid[x as usize][y as usize] > 0 {
                        if find_parents(&mut parents, c21(x as usize, y as usize)) < n {
                            flag = 1;
                        } else if find_parents(&mut parents, c21(x as usize, y as usize))
                            != find_parents(&mut parents, c21(_i, _j))
                        {
                            ret[i] += _size[find_parents(&mut parents, c21(x as usize, y as usize))];
                        }
                        union2(&mut parents, &mut _size, c21(x as usize, y as usize), c21(_i, _j));
                    }
                }
                ret[i] *= flag;
            }
        }
        ret.iter().map(|x| *x as i32).collect()
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]];
    let hits = vec![vec![1, 0]];
    let res = vec![2];
    assert_eq!(Solution::hit_bricks(grid, hits), res);

    let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]];
    let hits = vec![vec![1, 1], vec![1, 0]];
    let res = vec![0, 0];
    assert_eq!(Solution::hit_bricks(grid, hits), res);
}
