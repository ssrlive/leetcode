#![allow(dead_code)]

// 1210. Minimum Moves to Reach Target with Rotations
// https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/
// https://leetcode.cn/problems/minimum-moves-to-reach-target-with-rotations/
//
// Hard
//
// In an n*n grid, there is a snake that spans 2 cells and starts moving from the top left corner at (0, 0) and (0, 1). The grid has empty cells represented by zeros and blocked cells represented by ones. The snake wants to reach the lower right corner at (n-1, n-2) and (n-1, n-1).
//
// In one move the snake can:
//
//     Move one cell to the right if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.
//     Move down one cell if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.
//     Rotate clockwise if it's in a horizontal position and the two cells under it are both empty. In that case the snake moves from (r, c) and (r, c+1) to (r, c) and (r+1, c).
//     Rotate counterclockwise if it's in a vertical position and the two cells to its right are both empty. In that case the snake moves from (r, c) and (r+1, c) to (r, c) and (r, c+1).
//
// Return the minimum number of moves to reach the target.
//
// If there is no way to reach the target, return -1.
//
// Example 1:
//
// Input: grid = [[0,0,0,0,0,1],
//                [1,1,0,0,1,0],
//                [0,0,0,0,1,1],
//                [0,0,1,0,1,0],
//                [0,1,1,0,0,0],
//                [0,1,1,0,0,0]]
// Output: 11
// Explanation:
// One possible solution is [right, right, rotate clockwise, right, down, down, down, down, rotate counterclockwise, right, down].
//
// Example 2:
//
// Input: grid = [[0,0,1,1,1,1],
//                [0,0,0,0,1,1],
//                [1,1,0,0,0,1],
//                [1,1,1,0,0,1],
//                [1,1,1,0,0,1],
//                [1,1,1,0,0,0]]
// Output: 9
//
// Constraints:
//
// -    2 <= n <= 100
// -    0 <= grid[i][j] <= 1
// -    It is guaranteed that the snake starts at empty cells.
//

struct Solution;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, VecDeque};
        let n = grid.len();
        let mut ans = 0;
        let mut mp = HashMap::<(i32, (i32, i32)), i32>::new();
        let mut q = VecDeque::new();
        q.push_back((0, (0, 1)));
        mp.insert((0, (0, 1)), 1);
        while !q.is_empty() {
            let mut m = q.len();
            while m > 0 {
                let (pos, (x, y)) = q.pop_front().unwrap();
                if x == (n - 1) as i32 && y == (n - 1) as i32 && pos == 0 {
                    return ans;
                }
                if pos == 0 {
                    if y + 1 < n as i32 && grid[x as usize][y as usize + 1] == 0 && mp.get(&(0, (x, y + 1))).is_none() {
                        q.push_back((0, (x, y + 1)));
                        mp.insert((0, (x, y + 1)), 1);
                    }
                    if x + 1 < n as i32
                        && grid[x as usize + 1][y as usize] == 0
                        && grid[x as usize + 1][y as usize - 1] == 0
                        && mp.get(&(0, (x + 1, y))).is_none()
                    {
                        q.push_back((0, (x + 1, y)));
                        mp.insert((0, (x + 1, y)), 1);
                    }
                    if x + 1 < n as i32
                        && grid[x as usize + 1][y as usize] == 0
                        && grid[x as usize + 1][y as usize - 1] == 0
                        && mp.get(&(1, (x + 1, y - 1))).is_none()
                    {
                        q.push_back((1, (x + 1, y - 1)));
                        mp.insert((1, (x + 1, y - 1)), 1);
                    }
                } else {
                    if x + 1 < n as i32 && grid[x as usize + 1][y as usize] == 0 && mp.get(&(1, (x + 1, y))).is_none() {
                        q.push_back((1, (x + 1, y)));
                        mp.insert((1, (x + 1, y)), 1);
                    }
                    if y + 1 < n as i32
                        && grid[x as usize][y as usize + 1] == 0
                        && grid[x as usize - 1][y as usize + 1] == 0
                        && mp.get(&(1, (x, y + 1))).is_none()
                    {
                        q.push_back((1, (x, y + 1)));
                        mp.insert((1, (x, y + 1)), 1);
                    }
                    if y + 1 < n as i32
                        && grid[x as usize][y as usize + 1] == 0
                        && grid[x as usize - 1][y as usize + 1] == 0
                        && mp.get(&(0, (x - 1, y + 1))).is_none()
                    {
                        q.push_back((0, (x - 1, y + 1)));
                        mp.insert((0, (x - 1, y + 1)), 1);
                    }
                }
                m -= 1;
            }
            ans += 1;
        }
        -1
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![0, 0, 0, 0, 0, 1],
                vec![1, 1, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 1],
                vec![0, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 0],
            ],
            11,
        ),
        (
            vec![
                vec![0, 0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 0],
            ],
            9,
        ),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::minimum_moves(grid), expected);
    }
}
