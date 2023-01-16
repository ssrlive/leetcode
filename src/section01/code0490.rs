#![allow(dead_code)]

// 490. The Maze
// https://leetcode.com/problems/the-maze/
// https://leetcode.cn/problems/the-maze/
//
// There is a ball in a maze with empty spaces and walls. The ball can go through empty spaces by rolling up, down, left or right,
// but it won't stop rolling until hitting a wall. When the ball stops, it could choose the next direction.
//
// Given the ball's start position, the destination and the maze, determine whether the ball could stop at the destination.
//
// The maze is represented by a binary 2D array. 1 means the wall and 0 means the empty space. You may assume that the borders
// of the maze are all walls. The start and destination coordinates are represented by row and column indexes.
//
// Example 1:
// Input 1: a maze represented by a 2D array
// 0 0 1 0 0
// 0 0 0 0 0
// 0 0 0 1 0
// 1 1 0 1 1
// 0 0 0 0 0
//
// Input 2: start coordinate (rowStart, colStart) = (0, 4)
// Input 3: destination coordinate (rowDest, colDest) = (4, 4)
// Output: true
// Explanation: One possible way is : left -> down -> left -> down -> right -> down -> right.
//
// Example 2:
// Input 1: a maze represented by a 2D array
// 0 0 1 0 0
// 0 0 0 0 0
// 0 0 0 1 0
// 1 1 0 1 1
// 0 0 0 0 0
// Input 2: start coordinate (rowStart, colStart) = (0, 4)
// Input 3: destination coordinate (rowDest, colDest) = (3, 2)
// Output: false
// Explanation: There is no way for the ball to stop at the destination.
//
// Note:
// There is only one ball and one destination in the maze.
// Both the ball and the destination exist on an empty space, and they will not be at the same position initially.
// The given maze does not contain border (like the red rectangle in the example pictures), but you could assume the border of the maze are all walls.
// The maze contains at least 2 empty spaces, and both the width and height of the maze won't exceed 100.
//

struct Solution;

impl Solution {
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        fn dfs(maze: &[Vec<i32>], x: usize, y: usize, destination: &[i32], visited: &mut Vec<Vec<bool>>) -> bool {
            if visited[x][y] {
                return false;
            }
            visited[x][y] = true;
            if x == destination[0] as usize && y == destination[1] as usize {
                return true;
            }
            let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dx, dy) in dirs {
                let mut nx = x as i32;
                let mut ny = y as i32;
                while nx >= 0
                    && nx < maze.len() as i32
                    && ny >= 0
                    && ny < maze[0].len() as i32
                    && maze[nx as usize][ny as usize] == 0
                {
                    nx += dx;
                    ny += dy;
                }
                nx -= dx;
                ny -= dy;
                if dfs(maze, nx as usize, ny as usize, destination, visited) {
                    return true;
                }
            }
            false
        }

        let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
        dfs(&maze, start[0] as usize, start[1] as usize, &destination, &mut visited)
    }
}

#[test]
fn test() {
    let maze = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];
    let start = vec![0, 4];
    let destination = vec![4, 4];
    assert_eq!(Solution::has_path(maze, start, destination), true);

    let maze = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];
    let start = vec![0, 4];
    let destination = vec![3, 2];
    assert_eq!(Solution::has_path(maze, start, destination), false);
}
