#![allow(dead_code)]

/*

// 1926. Nearest Exit from Entrance in Maze
// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/
// https://leetcode.cn/problems/nearest-exit-from-entrance-in-maze/
//
// Medium
//
// You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls (represented as '+'). You are also given the entrance of the maze, where entrance = [entrancerow, entrancecol] denotes the row and column of the cell you are initially standing at.

In one step, you can move one cell up, down, left, or right. You cannot step into a cell with a wall, and you cannot step outside the maze. Your goal is to find the nearest exit from the entrance. An exit is defined as an empty cell that is at the border of the maze. The entrance does not count as an exit.

Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if no such path exists.

Example 1:

Input: maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]], entrance = [1,2]
Output: 1
Explanation: There are 3 exits in this maze at [1,0], [0,2], and [2,3].
Initially, you are at the entrance cell [1,2].
- You can reach [1,0] by moving 2 steps left.
- You can reach [0,2] by moving 1 step up.
It is impossible to reach [2,3] from the entrance.
Thus, the nearest exit is [0,2], which is 1 step away.

Example 2:

Input: maze = [["+","+","+"],[".",".","."],["+","+","+"]], entrance = [1,0]
Output: 2
Explanation: There is 1 exit in this maze at [1,2].
[1,0] does not count as an exit since it is the entrance cell.
Initially, you are at the entrance cell [1,0].
- You can reach [1,2] by moving 2 steps right.
Thus, the nearest exit is [1,2], which is 2 steps away.

Example 3:

Input: maze = [[".","+"]], entrance = [0,0]
Output: -1
Explanation: There are no exits in this maze.

Constraints:

    maze.length == m
    maze[i].length == n
    1 <= m, n <= 100
    maze[i][j] is either '.' or '+'.
    entrance.length == 2
    0 <= entrancerow < m
    0 <= entrancecol < n
    entrance will always be an empty cell.
*/

struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        fn add(u: i32, i: i32) -> i32 {
            if i.is_negative() { u - i.wrapping_abs() } else { u + i }
        }

        let mut maze = maze;
        let mut queue = VecDeque::new();
        let (start_row, start_col) = (entrance[0] as usize, entrance[1] as usize);
        queue.push_back((start_row, start_col, 0));
        maze[start_row][start_col] = '+';
        let dir: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some((i, j, len)) = queue.pop_front() {
            for &(di, dj) in &dir {
                let (x, y) = (add(i as i32, di), add(j as i32, dj));
                if x >= 0 && y >= 0 && x < maze.len() as i32 && y < maze[0].len() as i32 && maze[x as usize][y as usize] == '.' {
                    if x == 0 || y == 0 || x == maze.len() as i32 - 1 || y == maze[0].len() as i32 - 1 {
                        return len + 1;
                    }
                    maze[x as usize][y as usize] = '+';
                    queue.push_back((x as usize, y as usize, len + 1));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let maze = vec![
        vec!['+', '+', '.', '+'],
        vec!['.', '.', '.', '+'],
        vec!['+', '+', '+', '.'],
    ];
    let entrance = vec![1, 2];
    assert_eq!(Solution::nearest_exit(maze, entrance), 1);

    #[rustfmt::skip]
    let maze = vec![
        vec!['+', '+', '+'],
        vec!['.', '.', '.'],
        vec!['+', '+', '+'],
    ];
    let entrance = vec![1, 0];
    assert_eq!(Solution::nearest_exit(maze, entrance), 2);

    let maze = vec![vec!['.', '+']];
    let entrance = vec![0, 0];
    assert_eq!(Solution::nearest_exit(maze, entrance), -1);
}
