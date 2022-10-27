#![allow(dead_code)]

// 200. Number of Islands
// https://leetcode.com/problems/number-of-islands/
//
// Given a 2d grid map of '1's (land) and '0's (water), count the number of
// islands. An island is surrounded by water and is formed by connecting
// adjacent lands horizontally or vertically. You may assume all four edges
// of the grid are all surrounded by water.
//

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    let mut queue = std::collections::VecDeque::new();
                    queue.push_back((i, j));
                    while let Some((i, j)) = queue.pop_front() {
                        if grid[i][j] == '1' {
                            grid[i][j] = '0';
                            if i > 0 {
                                queue.push_back((i - 1, j));
                            }
                            if i < grid.len() - 1 {
                                queue.push_back((i + 1, j));
                            }
                            if j > 0 {
                                queue.push_back((i, j - 1));
                            }
                            if j < grid[i].len() - 1 {
                                queue.push_back((i, j + 1));
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

#[test]
fn test_num_islands() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ]),
        3
    );
}
