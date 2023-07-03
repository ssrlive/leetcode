#![allow(dead_code)]

/*

// 1728. Cat and Mouse II
// https://leetcode.com/problems/cat-and-mouse-ii/
// https://leetcode.cn/problems/cat-and-mouse-ii/
//
// Hard
//
// A game is played by a cat and a mouse named Cat and Mouse.

The environment is represented by a grid of size rows x cols, where each element is a wall, floor, player (Cat, Mouse), or food.

    Players are represented by the characters 'C'(Cat),'M'(Mouse).
    Floors are represented by the character '.' and can be walked on.
    Walls are represented by the character '#' and cannot be walked on.
    Food is represented by the character 'F' and can be walked on.
    There is only one of each character 'C', 'M', and 'F' in grid.

Mouse and Cat play according to the following rules:

    Mouse moves first, then they take turns to move.
    During each turn, Cat and Mouse can jump in one of the four directions (left, right, up, down). They cannot jump over the wall nor outside of the grid.
    catJump, mouseJump are the maximum lengths Cat and Mouse can jump at a time, respectively. Cat and Mouse can jump less than the maximum length.
    Staying in the same position is allowed.
    Mouse can jump over Cat.

The game can end in 4 ways:

    If Cat occupies the same position as Mouse, Cat wins.
    If Cat reaches the food first, Cat wins.
    If Mouse reaches the food first, Mouse wins.
    If Mouse cannot get to the food within 1000 turns, Cat wins.

Given a rows x cols matrix grid and two integers catJump and mouseJump, return true if Mouse can win the game if both Cat and Mouse play optimally, otherwise return false.

Example 1:

Input: grid = ["####F","#C...","M...."], catJump = 1, mouseJump = 2
Output: true
Explanation: Cat cannot catch Mouse on its turn nor can it get the food before Mouse.

Example 2:

Input: grid = ["M.C...F"], catJump = 1, mouseJump = 4
Output: true

Example 3:

Input: grid = ["M.C...F"], catJump = 1, mouseJump = 3
Output: false

Constraints:

    rows == grid.length
    cols = grid[i].length
    1 <= rows, cols <= 8
    grid[i][j] consist only of characters 'C', 'M', 'F', '.', and '#'.
    There is only one of each character 'C', 'M', and 'F' in grid.
    1 <= catJump, mouseJump <= 8
*/

struct Solution;

impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        fn get_jumps(grid: &Vec<String>, p: i32, f: i32, max: i32) -> Vec<i32> {
            let dirs = [0, 1, 0, -1, 0];
            let mut res = vec![p];
            for d in 0..4 {
                for jump in 1..=max {
                    let i = p / 8 + dirs[d] * jump;
                    let j = p % 8 + dirs[d + 1] * jump;
                    if i < 0
                        || j < 0
                        || i as usize >= grid.len()
                        || j as usize >= grid[i as usize].len()
                        || grid[i as usize].chars().nth(j as usize).unwrap() == '#'
                    {
                        break;
                    }
                    res.push(i * 8 + j);
                }
            }
            res.sort_by(|&a, &b| {
                let v1 = (f / 8 - a / 8).abs() + (f % 8 - a % 8).abs();
                let v2 = (f / 8 - b / 8).abs() + (f % 8 - b % 8).abs();
                v1.cmp(&v2)
            });
            res
        }

        #[allow(clippy::too_many_arguments)]
        fn dfs(grid: &Vec<String>, turn: i32, c: i32, m: i32, f: i32, cat_jump: i32, mouse_jump: i32, dp: &mut Vec<Vec<Vec<i32>>>) -> bool {
            if turn >= 0 && (m == f || dp[turn as usize][m as usize][c as usize] == 1) {
                return true;
            }
            if turn < 0 || c == m || c == f || dp[turn as usize][m as usize][c as usize] == -1 {
                return false;
            }
            let cat_jumps = get_jumps(grid, c, f, cat_jump);
            for new_m in get_jumps(grid, m, f, mouse_jump) {
                let mut mouse_won = true;
                for &jmp in cat_jumps.iter() {
                    mouse_won &= dfs(grid, turn - 1, jmp, new_m, f, cat_jump, mouse_jump, dp);
                }
                if mouse_won {
                    dp[turn as usize][m as usize][c as usize] = 1;
                    return true;
                }
            }
            dp[turn as usize][m as usize][c as usize] = -1;
            false
        }

        let mut dp = vec![vec![vec![0; 64]; 64]; 64];
        let (mut c, mut m, mut f, rows, cols) = (0_i32, 0_i32, 0_i32, grid.len() as i32, grid[0].len() as i32);
        let turns = rows * cols - 1;
        for i in 0..rows {
            for j in 0..cols {
                m = if grid[i as usize].chars().nth(j as usize).unwrap() == 'M' {
                    i * 8 + j
                } else {
                    m
                };
                c = if grid[i as usize].chars().nth(j as usize).unwrap() == 'C' {
                    i * 8 + j
                } else {
                    c
                };
                f = if grid[i as usize].chars().nth(j as usize).unwrap() == 'F' {
                    i * 8 + j
                } else {
                    f
                };
            }
        }
        dfs(&grid, turns, c, m, f, cat_jump, mouse_jump, &mut dp)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["####F", "#C...", "M...."], 1, 2, true),
        (vec!["M.C...F"], 1, 4, true),
        (vec!["M.C...F"], 1, 3, false),
    ];
    for (grid, cat_jump, mouse_jump, expected) in cases {
        let grid = grid.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::can_mouse_win(grid, cat_jump, mouse_jump), expected);
    }
}
