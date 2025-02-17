#![allow(dead_code)]

// 3459. Length of Longest V-Shaped Diagonal Segment
// https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment/
// https://leetcode.cn/problems/length-of-longest-v-shaped-diagonal-segment/
//
// Hard
//
// You are given a 2D integer matrix grid of size n x m, where each element is either 0, 1, or 2.
//
// A V-shaped diagonal segment is defined as:
//
// - The segment starts with 1.
// - The subsequent elements follow this infinite sequence: 2, 0, 2, 0, ....
// - The segment:
//   - Starts along a diagonal direction (top-left to bottom-right, bottom-right to top-left,
//     top-right to bottom-left, or bottom-left to top-right).
//   - Continues the sequence in the same diagonal direction.
//   - Makes at most one clockwise 90-degree turn to another diagonal direction while maintaining the sequence.
//
// Return the length of the longest V-shaped diagonal segment. If no valid segment exists, return 0.
//
// Example 1:
//
// Input: grid = [[2,2,1,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]
//
// Output: 5
//
// Explanation:
//
// The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: (0,2) → (1,3) → (2,4),
// takes a 90-degree clockwise turn at (2,4), and continues as (3,3) → (4,2).
//
// Example 2:
//
// Input: grid = [[2,2,2,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]
//
// Output: 4
//
// Explanation:
//
// The longest V-shaped diagonal segment has a length of 4 and follows these coordinates: (2,3) → (3,2),
// takes a 90-degree clockwise turn at (3,2), and continues as (2,1) → (1,0).
//
// Example 3:
//
// Input: grid = [[1,2,2,2,2],[2,2,2,2,0],[2,0,0,0,0],[0,0,2,2,2],[2,0,0,2,0]]
//
// Output: 5
//
// Explanation:
//
// The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: (0,0) → (1,1) → (2,2) → (3,3) → (4,4).
//
// Example 4:
//
// Input: grid = [[1]]
//
// Output: 1
//
// Explanation:
//
// The longest V-shaped diagonal segment has a length of 1 and follows these coordinates: (0,0).
//
// Constraints:
//
//     n == grid.length
//     m == grid[i].length
//     1 <= n, m <= 500
//     grid[i][j] is either 0, 1 or 2.
//

struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        #[allow(clippy::too_many_arguments)]
        fn solve(
            r: i32,
            c: i32,
            turn: bool,
            curr: bool,
            dir: i32,
            grid: &Vec<Vec<i32>>,
            dp: &mut Vec<Vec<Vec<Vec<Vec<i32>>>>>,
            n: i32,
            m: i32,
        ) -> i32 {
            if r == n || c == m || r < 0 || c < 0 {
                return 0;
            }
            if dp[r as usize][c as usize][turn as usize][curr as usize][dir as usize] != -1 {
                return dp[r as usize][c as usize][turn as usize][curr as usize][dir as usize];
            }
            if curr && grid[r as usize][c as usize] != 2 {
                return 0;
            }
            if !curr && grid[r as usize][c as usize] != 0 {
                return 0;
            }
            let mut ans = 0;
            // 1 - top right , 2 - bottom right , 3 - bottom left , 4 - top left
            if dir == 1 {
                ans = 1 + solve(r - 1, c + 1, turn, !curr, 1, grid, dp, n, m);
                if !turn {
                    ans = ans.max(1 + solve(r + 1, c + 1, true, !curr, 2, grid, dp, n, m));
                }
            }
            if dir == 2 {
                ans = 1 + solve(r + 1, c + 1, turn, !curr, 2, grid, dp, n, m);
                if !turn {
                    ans = ans.max(1 + solve(r + 1, c - 1, true, !curr, 3, grid, dp, n, m));
                }
            }
            if dir == 3 {
                ans = 1 + solve(r + 1, c - 1, turn, !curr, 3, grid, dp, n, m);
                if !turn {
                    ans = ans.max(1 + solve(r - 1, c - 1, true, !curr, 4, grid, dp, n, m));
                }
            }
            if dir == 4 {
                ans = 1 + solve(r - 1, c - 1, turn, !curr, 4, grid, dp, n, m);
                if !turn {
                    ans = ans.max(1 + solve(r - 1, c + 1, true, !curr, 1, grid, dp, n, m));
                }
            }
            dp[r as usize][c as usize][turn as usize][curr as usize][dir as usize] = ans;
            ans
        }

        let n = grid.len() as i32;
        let m = grid[0].len() as i32;
        let mut dp = vec![vec![vec![vec![vec![-1; 5]; 2]; 2]; m as usize]; n as usize];
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i as usize][j as usize] == 1 {
                    ans = ans.max(1 + solve(i + 1, j + 1, false, true, 2, &grid, &mut dp, n, m));
                    ans = ans.max(1 + solve(i - 1, j + 1, false, true, 1, &grid, &mut dp, n, m));
                    ans = ans.max(1 + solve(i + 1, j - 1, false, true, 3, &grid, &mut dp, n, m));
                    ans = ans.max(1 + solve(i - 1, j - 1, false, true, 4, &grid, &mut dp, n, m));
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![2, 2, 1, 2, 2],
        vec![2, 0, 2, 2, 0],
        vec![2, 0, 1, 1, 0],
        vec![1, 0, 2, 2, 2],
        vec![2, 0, 0, 2, 2],
    ];

    let res = 5;
    assert_eq!(Solution::len_of_v_diagonal(grid), res);
    let grid = vec![
        vec![2, 2, 2, 2, 2],
        vec![2, 0, 2, 2, 0],
        vec![2, 0, 1, 1, 0],
        vec![1, 0, 2, 2, 2],
        vec![2, 0, 0, 2, 2],
    ];

    let res = 4;
    assert_eq!(Solution::len_of_v_diagonal(grid), res);
    let grid = vec![
        vec![1, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 0],
        vec![2, 0, 0, 0, 0],
        vec![0, 0, 2, 2, 2],
        vec![2, 0, 0, 2, 0],
    ];

    let res = 5;
    assert_eq!(Solution::len_of_v_diagonal(grid), res);
    let grid = vec![vec![1]];
    let res = 1;
    assert_eq!(Solution::len_of_v_diagonal(grid), res);
}
