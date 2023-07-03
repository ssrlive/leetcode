#![allow(dead_code)]

/*

// 2088. Count Fertile Pyramids in a Land
// https://leetcode.com/problems/count-fertile-pyramids-in-a-land/
// https://leetcode.cn/problems/count-fertile-pyramids-in-a-land/
//
// Hard
//
// A farmer has a rectangular grid of land with m rows and n columns that can be divided into unit cells. Each cell is either fertile (represented by a 1) or barren (represented by a 0). All cells outside the grid are considered barren.

A pyramidal plot of land can be defined as a set of cells with the following criteria:

    The number of cells in the set has to be greater than 1 and all cells must be fertile.
    The apex of a pyramid is the topmost cell of the pyramid. The height of a pyramid is the number of rows it covers. Let (r, c) be the apex of the pyramid, and its height be h. Then, the plot comprises of cells (i, j) where r <= i <= r + h - 1 and c - (i - r) <= j <= c + (i - r).

An inverse pyramidal plot of land can be defined as a set of cells with similar criteria:

    The number of cells in the set has to be greater than 1 and all cells must be fertile.
    The apex of an inverse pyramid is the bottommost cell of the inverse pyramid. The height of an inverse pyramid is the number of rows it covers. Let (r, c) be the apex of the pyramid, and its height be h. Then, the plot comprises of cells (i, j) where r - h + 1 <= i <= r and c - (r - i) <= j <= c + (r - i).

Some examples of valid and invalid pyramidal (and inverse pyramidal) plots are shown below. Black cells indicate fertile cells.

Given a 0-indexed m x n binary matrix grid representing the farmland, return the total number of pyramidal and inverse pyramidal plots that can be found in grid.

Example 1:

Input: grid = [[0,1,1,0],[1,1,1,1]]
Output: 2
Explanation: The 2 possible pyramidal plots are shown in blue and red respectively.
There are no inverse pyramidal plots in this grid.
Hence total number of pyramidal and inverse pyramidal plots is 2 + 0 = 2.

Example 2:

Input: grid = [[1,1,1],[1,1,1]]
Output: 2
Explanation: The pyramidal plot is shown in blue, and the inverse pyramidal plot is shown in red.
Hence the total number of plots is 1 + 1 = 2.

Example 3:

Input: grid = [[1,1,1,1,0],[1,1,1,1,1],[1,1,1,1,1],[0,1,0,0,1]]
Output: 13
Explanation: There are 7 pyramidal plots, 3 of which are shown in the 2nd and 3rd figures.
There are 6 inverse pyramidal plots, 2 of which are shown in the last figure.
The total number of plots is 7 + 6 = 13.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 1000
    1 <= m * n <= 10^5
    grid[i][j] is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        fn count_hanging(grid: &mut [Vec<i32>]) -> usize {
            let mut ret = 0;
            for x in 1..(grid[0].len() - 1) {
                if grid[0][x] > 1 {
                    grid[0][x] = 1;
                }
            }
            for y in 1..grid.len() {
                for x in 1..(grid[y].len() - 1) {
                    if grid[y][x] == 0 {
                        continue;
                    }
                    if grid[y - 1][x] > 0 {
                        grid[y][x] = 1 + std::cmp::min(grid[y - 1][x - 1], grid[y - 1][x + 1]);
                        ret += (grid[y][x] - 1) as usize;
                    } else if grid[y][x] > 1 {
                        grid[y][x] = 1;
                    }
                }
            }
            ret
        }

        let mut grid = grid;
        let ret = count_hanging(&mut grid[..]);
        grid.reverse();
        (ret + count_hanging(&mut grid[..])) as i32
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]];
    assert_eq!(Solution::count_pyramids(grid), 2);

    let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
    assert_eq!(Solution::count_pyramids(grid), 2);

    let grid = vec![vec![1, 1, 1, 1, 0], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![0, 1, 0, 0, 1]];
    assert_eq!(Solution::count_pyramids(grid), 13);
}
